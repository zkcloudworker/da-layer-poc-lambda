//! This module includes the definition of the XOR gadget for 64, 32, and 16 bits,
//! the definition of the constraints of the `Xor16` circuit gate,
//! and the code for witness generation for the XOR gadget.
use crate::{
    alphas::Alphas,
    circuits::{
        argument::{Argument, ArgumentEnv, ArgumentType},
        constraints::ConstraintSystem,
        expr::{self, constraints::ExprOps, l0_1, Environment, LookupEnvironment},
        gate::{CircuitGate, CircuitGateError, CircuitGateResult, Connect, GateType},
        lookup::{
            self,
            lookups::{LookupInfo, LookupsUsed},
            tables::{GateLookupTable, LookupTable},
        },
        polynomial::COLUMNS,
        wires::Wire,
        witness::{self, ConstantCell, CopyBitsCell, CrumbCell, Variables, WitnessCell},
    },
    curve::KimchiCurve,
    variable_map,
};
use ark_ff::{PrimeField, Zero};
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, Evaluations, Radix2EvaluationDomain as D,
};
use num_bigint::BigUint;
use o1_utils::{big_bits, big_not, big_xor, FieldFromBig, FieldHelpers};
use rand::{rngs::StdRng, SeedableRng};
use std::{array, cmp::max, collections::HashMap, marker::PhantomData};

use super::generic::GenericGateSpec;

pub const GATE_COUNT: usize = 1;

impl<F: PrimeField> CircuitGate<F> {
    /// Creates a XOR gadget for `bits` length
    /// Includes:
    /// - num_xors Xor16 gates
    /// - 1 Generic gate to constrain the final row to be zero with itself
    /// Outputs tuple (next_row, circuit_gates) where
    /// - next_row  : next row after this gate
    /// - gates     : vector of circuit gates comprising this gate
    pub fn create_xor_gadget(new_row: usize, bits: u32) -> (usize, Vec<Self>) {
        let num_xors = num_xors(bits);
        let mut gates = (0..num_xors)
            .map(|i| CircuitGate {
                typ: GateType::Xor16,
                wires: Wire::for_row(new_row + i),
                coeffs: vec![],
            })
            .collect::<Vec<_>>();
        let zero_row = new_row + num_xors;
        gates.push(CircuitGate::create_generic_gadget(
            Wire::for_row(zero_row),
            GenericGateSpec::Const(F::zero()),
            None,
        ));
        // check fin_in1, fin_in2, fin_out are zero
        gates.connect_cell_pair((zero_row, 0), (zero_row, 1));
        gates.connect_cell_pair((zero_row, 0), (zero_row, 2));

        (zero_row + 1, gates)
    }

    /// Creates a negation gadget with `nots` NOT components of some length previously constrained using a generic gate
    /// (checking that a cell stores `2^bits-1` value). Assumes that the inputs are known to have at most `bits` length.
    /// Starts the gates in the `new_row` position.
    /// Includes:
    /// - ceil(nots/2) Double Generic gates to perform the `( 2^(bits) - 1 ) - input` operation for every two inputs in each row
    /// BEWARE:
    /// - If the input is not known to have at most `bits` length, this should be constrained somewhere else.
    /// - Otherwise, use the Xor builder instead (but this one requires about 8 times more rows).
    /// INTEGRATION:
    /// - Needs a leading public input generic gate in `pub_row` to constrain the left input of each generic gate for negation to be `2^bits-1`.
    pub fn extend_not_gnrc_gadget(
        gates: &mut Vec<Self>,
        nots: usize,
        pub_row: usize,
        new_row: usize,
    ) -> usize {
        // taking advantage of double generic gates to negate two words in each row
        let mut new_row = new_row;
        for _ in 0..nots / 2 {
            new_row = Self::not_gnrc(gates, pub_row, new_row, true);
        }
        // odd number of NOTs require one more row to negate the last word only
        if nots % 2 == 1 {
            new_row = Self::not_gnrc(gates, pub_row, new_row, false);
        }
        new_row
    }

    // Returns a double generic gate for negation for one or two words
    // Input:
    // - new_row : row to start the double NOT generic gate
    // - pub_row : row where the public inputs is stored (the 2^bits - 1) value (in the column 0 of that row)
    // - double  : whether to perform two NOTs or only one inside the generic gate
    // Output:
    // - new_row : next row after the double NOT generic gate, corresponds to `new_row+1`
    fn not_gnrc(gates: &mut Vec<Self>, pub_row: usize, new_row: usize, double: bool) -> usize {
        let g1 = GenericGateSpec::Add {
            left_coeff: None,
            right_coeff: Some(-F::one()),
            output_coeff: None,
        };
        let g2 = {
            if double {
                Some(GenericGateSpec::Add {
                    left_coeff: None,
                    right_coeff: Some(-F::one()),
                    output_coeff: None,
                })
            } else {
                None
            }
        };
        let mut not_gate = vec![CircuitGate::create_generic_gadget(
            Wire::for_row(new_row),
            g1,
            g2,
        )];
        gates.append(&mut not_gate);
        // check left inputs of the double generic gate correspond to the 2^bits - 1 value
        gates.connect_cell_pair((pub_row, 0), (new_row, 0));
        if double {
            gates.connect_cell_pair((pub_row, 0), (new_row, 3));
        }
        new_row + 1
    }

    /// Creates a NOT gadget for `bits` length using Xor gates.
    /// It implicitly constrains the length of the input to be at most 16 * num_xors bits.
    /// Includes:
    /// - num_xors Xor16 gates
    /// - 1 Generic gate to constrain the final row to be zero with itself
    /// Input:
    /// - new_row : row to start the NOT gadget
    /// Requires:
    /// - 1 initial public input generic gate in `pub_row` to constrain the input to be `2^bits-1`.
    /// INTEGRATION:
    /// - Connect the left input to a public input row containing the 2^bits-1 value
    pub fn extend_not_xor_gadget(
        gates: &mut Vec<Self>,
        pub_row: usize,
        new_row: usize,
        bits: u32,
    ) -> usize {
        let num_xors = num_xors(bits);
        let mut not_gates = (0..num_xors)
            .map(|i| CircuitGate {
                typ: GateType::Xor16,
                wires: Wire::for_row(new_row + i),
                coeffs: vec![],
            })
            .collect::<Vec<_>>();
        let zero_row = new_row + num_xors;
        not_gates.push(CircuitGate::create_generic_gadget(
            Wire::for_row(zero_row),
            GenericGateSpec::Const(F::zero()),
            None,
        ));
        gates.extend(not_gates);
        // check fin_in1, fin_in2, fin_out are zero
        gates.connect_cell_pair((zero_row, 0), (zero_row, 1));
        gates.connect_cell_pair((zero_row, 0), (zero_row, 2));
        // Integration
        gates.connect_cell_pair((pub_row, 0), (new_row, 1)); // input2 of xor is all ones

        gates.len()
    }

    /// Verifies the xor gadget
    pub fn verify_xor<G: KimchiCurve<ScalarField = F>>(
        &self,
        _: usize,
        witness: &[Vec<F>; COLUMNS],
        cs: &ConstraintSystem<F>,
    ) -> CircuitGateResult<()> {
        if GateType::Xor16 != self.typ {
            return Err(CircuitGateError::InvalidCircuitGateType(self.typ));
        }

        // Pad the witness to domain d1 size
        let padding_length = cs
            .domain
            .d1
            .size
            .checked_sub(witness[0].len() as u64)
            .unwrap();
        let mut witness = witness.clone();
        for w in &mut witness {
            w.extend(std::iter::repeat(F::zero()).take(padding_length as usize));
        }

        // Compute witness polynomial
        let witness_poly: [DensePolynomial<F>; COLUMNS] = array::from_fn(|i| {
            Evaluations::<F, D<F>>::from_vec_and_domain(witness[i].clone(), cs.domain.d1)
                .interpolate()
        });

        // Compute permutation polynomial
        let rng = &mut StdRng::from_seed([0u8; 32]);
        let beta = F::rand(rng);
        let gamma = F::rand(rng);
        let z_poly = cs
            .perm_aggreg(&witness, &beta, &gamma, rng)
            .map_err(|_| CircuitGateError::InvalidCopyConstraint(self.typ))?;

        // Compute witness polynomial evaluations
        let witness_evals = cs.evaluate(&witness_poly, &z_poly);

        let mut index_evals = HashMap::new();
        index_evals.insert(self.typ, &cs.xor_selector_poly.as_ref().unwrap().eval8);

        // Set up lookup environment
        let lcs = cs
            .lookup_constraint_system
            .as_ref()
            .ok_or(CircuitGateError::MissingLookupConstraintSystem(self.typ))?;

        let lookup_env_data = set_up_lookup_env_data(
            self.typ,
            cs,
            &witness,
            &beta,
            &gamma,
            &lcs.configuration.lookup_info,
        )?;
        let lookup_env = Some(LookupEnvironment {
            aggreg: &lookup_env_data.aggreg8,
            sorted: &lookup_env_data.sorted8,
            selectors: &lcs.lookup_selectors,
            table: &lookup_env_data.joint_lookup_table_d8,
            runtime_selector: None,
            runtime_table: None,
        });

        // Set up the environment
        let env = {
            Environment {
                constants: expr::Constants {
                    alpha: F::rand(rng),
                    beta: F::rand(rng),
                    gamma: F::rand(rng),
                    joint_combiner: Some(F::rand(rng)),
                    endo_coefficient: cs.endo,
                    mds: &G::sponge_params().mds,
                    foreign_field_modulus: None,
                },
                witness: &witness_evals.d8.this.w,
                coefficient: &cs.coefficients8,
                vanishes_on_last_4_rows: &cs.precomputations().vanishes_on_last_4_rows,
                z: &witness_evals.d8.this.z,
                l0_1: l0_1(cs.domain.d1),
                domain: cs.domain,
                index: index_evals,
                lookup: lookup_env,
            }
        };

        // Setup powers of alpha
        let mut alphas = Alphas::<F>::default();
        alphas.register(ArgumentType::Gate(self.typ), Xor16::<F>::CONSTRAINTS);

        // Get constraints for this circuit gate
        let constraints = Xor16::combined_constraints(&alphas);

        // Verify it against the environment
        if constraints
            .evaluations(&env)
            .interpolate()
            .divide_by_vanishing_poly(cs.domain.d1)
            .unwrap()
            .1
            .is_zero()
        {
            Ok(())
        } else {
            Err(CircuitGateError::InvalidConstraint(self.typ))
        }
    }
}

// Data required by the lookup environment
struct LookupEnvironmentData<F: PrimeField> {
    // Aggregation evaluations
    aggreg8: Evaluations<F, D<F>>,
    // Sorted evaluations
    sorted8: Vec<Evaluations<F, D<F>>>,
    // Combined lookup table
    joint_lookup_table_d8: Evaluations<F, D<F>>,
}

// Helper to create the lookup environment data by setting up the joint- and table-id- combiners,
// computing the dummy lookup value, creating the combined lookup table, computing the sorted plookup
// evaluations and the plookup aggregation evaluations.
// Note: This function assumes the cs contains a lookup constraint system.
fn set_up_lookup_env_data<F: PrimeField>(
    gate_type: GateType,
    cs: &ConstraintSystem<F>,
    witness: &[Vec<F>; COLUMNS],
    beta: &F,
    gamma: &F,
    lookup_info: &LookupInfo,
) -> CircuitGateResult<LookupEnvironmentData<F>> {
    let lcs = cs
        .lookup_constraint_system
        .as_ref()
        .ok_or(CircuitGateError::MissingLookupConstraintSystem(gate_type))?;

    let rng = &mut StdRng::from_seed([1u8; 32]);

    // Set up joint-combiner and table-id-combiner
    let joint_lookup_used = matches!(lcs.configuration.lookup_used, LookupsUsed::Joint);
    let joint_combiner = if joint_lookup_used {
        F::rand(rng)
    } else {
        F::zero()
    };
    let table_id_combiner: F = if lcs.table_ids8.as_ref().is_some() {
        joint_combiner.pow([u64::from(lcs.configuration.lookup_info.max_joint_size)])
    } else {
        // TODO: just set this to None in case multiple tables are not used
        F::zero()
    };

    // Compute the dummy lookup value as the combination of the last entry of the XOR table (so `(0, 0, 0)`).
    // Warning: This assumes that we always use the XOR table when using lookups.
    let dummy_lookup_value = lcs
        .configuration
        .dummy_lookup
        .evaluate(&joint_combiner, &table_id_combiner);

    // Compute the lookup table values as the combination of the lookup table entries.
    let joint_lookup_table_d8 = {
        let mut evals = Vec::with_capacity(cs.domain.d1.size());

        for idx in 0..(cs.domain.d1.size() * 8) {
            let table_id = match lcs.table_ids8.as_ref() {
                Some(table_ids8) => table_ids8.evals[idx],
                None =>
                // If there is no `table_ids8` in the constraint system,
                // every table ID is identically 0.
                {
                    F::zero()
                }
            };

            let combined_entry = {
                let table_row = lcs.lookup_table8.iter().map(|e| &e.evals[idx]);

                lookup::tables::combine_table_entry(
                    &joint_combiner,
                    &table_id_combiner,
                    table_row,
                    &table_id,
                )
            };
            evals.push(combined_entry);
        }

        Evaluations::from_vec_and_domain(evals, cs.domain.d8)
    };

    // Compute the sorted plookup evaluations
    // TODO: Once we switch to committing using lagrange commitments, `witness` will be consumed when we interpolate,
    //       so interpolation will have to moved below this.
    let sorted: Vec<_> = lookup::constraints::sorted(
        dummy_lookup_value,
        &joint_lookup_table_d8,
        cs.domain.d1,
        &cs.gates,
        witness,
        joint_combiner,
        table_id_combiner,
        lookup_info,
    )
    .map_err(|_| CircuitGateError::InvalidLookupConstraintSorted(gate_type))?;

    // Randomize the last `EVALS` rows in each of the sorted polynomials in order to add zero-knowledge to the protocol.
    let sorted: Vec<_> = sorted
        .into_iter()
        .map(|chunk| lookup::constraints::zk_patch(chunk, cs.domain.d1, rng))
        .collect();

    let sorted_coeffs: Vec<_> = sorted.iter().map(|e| e.clone().interpolate()).collect();
    let sorted8 = sorted_coeffs
        .iter()
        .map(|v| v.evaluate_over_domain_by_ref(cs.domain.d8))
        .collect::<Vec<_>>();

    // Compute the plookup aggregation evaluations
    let aggreg = lookup::constraints::aggregation::<_, F>(
        dummy_lookup_value,
        &joint_lookup_table_d8,
        cs.domain.d1,
        &cs.gates,
        witness,
        &joint_combiner,
        &table_id_combiner,
        *beta,
        *gamma,
        &sorted,
        rng,
        lookup_info,
    )
    .map_err(|_| CircuitGateError::InvalidLookupConstraintAggregation(gate_type))?;

    // Precompute different forms of the aggregation polynomial for later
    let aggreg_coeffs = aggreg.interpolate();
    // TODO: There's probably a clever way to expand the domain without interpolating
    let aggreg8 = aggreg_coeffs.evaluate_over_domain_by_ref(cs.domain.d8);

    Ok(LookupEnvironmentData {
        aggreg8,
        sorted8,
        joint_lookup_table_d8,
    })
}

/// Get the xor lookup table
pub fn lookup_table<F: PrimeField>() -> LookupTable<F> {
    lookup::tables::get_table::<F>(GateLookupTable::Xor)
}

//~ `Xor16` - Chainable XOR constraints for words of multiples of 16 bits.
//~
//~ * This circuit gate is used to constrain that `in1` xored with `in2` equals `out`
//~ * The length of `in1`, `in2` and `out` must be the same and a multiple of 16bits.
//~ * This gate operates on the `Curr` and `Next` rows.
//~
//~ It uses three different types of constraints
//~ * copy          - copy to another cell (32-bits)
//~ * plookup       - xor-table plookup (4-bits)
//~ * decomposition - the constraints inside the gate
//~
//~ The 4-bit crumbs are assumed to be laid out with `0` column being the least significant crumb.
//~ Given values `in1`, `in2` and `out`, the layout looks like this:
//~
//~ | Column |          `Curr`  |          `Next`  |
//~ | ------ | ---------------- | ---------------- |
//~ |      0 | copy     `in1`   | copy     `in1'`  |
//~ |      1 | copy     `in2`   | copy     `in2'`  |
//~ |      2 | copy     `out`   | copy     `out'`  |
//~ |      3 | plookup0 `in1_0` |                  |
//~ |      4 | plookup1 `in1_1` |                  |
//~ |      5 | plookup2 `in1_2` |                  |
//~ |      6 | plookup3 `in1_3` |                  |
//~ |      7 | plookup0 `in2_0` |                  |
//~ |      8 | plookup1 `in2_1` |                  |
//~ |      9 | plookup2 `in2_2` |                  |
//~ |     10 | plookup3 `in2_3` |                  |
//~ |     11 | plookup0 `out_0` |                  |
//~ |     12 | plookup1 `out_1` |                  |
//~ |     13 | plookup2 `out_2` |                  |
//~ |     14 | plookup3 `out_3` |                  |
//~
//~ One single gate with next values of `in1'`, `in2'` and `out'` being zero can be used to check
//~ that the original `in1`, `in2` and `out` had 16-bits. We can chain this gate 4 times as follows
//~ to obtain a gadget for 64-bit words XOR:
//~
//~ | Row | `CircuitGate` | Purpose                                    |
//~ | --- | ------------- | ------------------------------------------ |
//~ |   0 | `Xor16`       | Xor 2 least significant bytes of the words |
//~ |   1 | `Xor16`       | Xor next 2 bytes of the words              |
//~ |   2 | `Xor16`       | Xor next 2 bytes of the words              |
//~ |   3 | `Xor16`       | Xor 2 most significant bytes of the words  |
//~ |   4 | `Zero`        | Zero values, can be reused as generic gate |
//~
//~ ```admonition::notice
//~  We could half the number of rows of the 64-bit XOR gadget by having lookups
//~  for 8 bits at a time, but for now we will use the 4-bit XOR table that we have.
//~  Rough computations show that if we run 8 or more Keccaks in one circuit we should
//~  use the 8-bit XOR table.
//~ ```
#[derive(Default)]
pub struct Xor16<F>(PhantomData<F>);

impl<F> Argument<F> for Xor16<F>
where
    F: PrimeField,
{
    const ARGUMENT_TYPE: ArgumentType = ArgumentType::Gate(GateType::Xor16);
    const CONSTRAINTS: u32 = 3;

    // Constraints for Xor16
    //   * Operates on Curr and Next rows
    //   * Constrain the decomposition of `in1`, `in2` and `out` of multiples of 16 bits
    //   * The actual XOR is performed thanks to the plookups of 4-bit XORs.
    fn constraint_checks<T: ExprOps<F>>(env: &ArgumentEnv<F, T>) -> Vec<T> {
        let two = T::from(2u64);
        // in1 = in1_0 + in1_1 * 2^4 + in1_2 * 2^8 + in1_3 * 2^12 + next_in1 * 2^16
        // in2 = in2_0 + in2_1 * 2^4 + in2_2 * 2^8 + in2_3 * 2^12 + next_in2 * 2^16
        // out = out_0 + out_1 * 2^4 + out_2 * 2^8 + out_3 * 2^12 + next_out * 2^16
        (0..3)
            .map(|i| {
                env.witness_curr(3 + 4 * i)
                    + env.witness_curr(4 + 4 * i) * two.clone().pow(4)
                    + env.witness_curr(5 + 4 * i) * two.clone().pow(8)
                    + env.witness_curr(6 + 4 * i) * two.clone().pow(12)
                    + two.clone().pow(16) * env.witness_next(i)
                    - env.witness_curr(i)
            })
            .collect::<Vec<T>>()
    }
}

// Witness layout
fn layout<F: PrimeField>(curr_row: usize, bits: u32) -> Vec<[Box<dyn WitnessCell<F>>; COLUMNS]> {
    let num_xor = num_xors(bits);
    let mut layout = (0..num_xor)
        .map(|i| xor_row(i, curr_row + i))
        .collect::<Vec<_>>();
    layout.push(zero_row());
    layout
}

fn xor_row<F: PrimeField>(crumb: usize, curr_row: usize) -> [Box<dyn WitnessCell<F>>; COLUMNS] {
    [
        CrumbCell::create("in1", crumb),
        CrumbCell::create("in2", crumb),
        CrumbCell::create("out", crumb),
        CopyBitsCell::create(curr_row, 0, 0, 4), // First 4-bit crumb of in1
        CopyBitsCell::create(curr_row, 0, 4, 8), // Second 4-bit crumb of in1
        CopyBitsCell::create(curr_row, 0, 8, 12), // Third 4-bit crumb of in1
        CopyBitsCell::create(curr_row, 0, 12, 16), // Fourth 4-bit crumb of in1
        CopyBitsCell::create(curr_row, 1, 0, 4), // First 4-bit crumb of in2
        CopyBitsCell::create(curr_row, 1, 4, 8), // Second 4-bit crumb of in2
        CopyBitsCell::create(curr_row, 1, 8, 12), // Third 4-bit crumb of in2
        CopyBitsCell::create(curr_row, 1, 12, 16), // Fourth 4-bit crumb of in2
        CopyBitsCell::create(curr_row, 2, 0, 4), // First 4-bit crumb of out
        CopyBitsCell::create(curr_row, 2, 4, 8), // Second 4-bit crumb of out
        CopyBitsCell::create(curr_row, 2, 8, 12), // Third 4-bit crumb of out
        CopyBitsCell::create(curr_row, 2, 12, 16), // Fourth 4-bit crumb of out
    ]
}

fn zero_row<F: PrimeField>() -> [Box<dyn WitnessCell<F>>; COLUMNS] {
    [
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
        ConstantCell::create(F::zero()),
    ]
}

fn init_xor<F: PrimeField>(
    witness: &mut [Vec<F>; COLUMNS],
    curr_row: usize,
    bits: u32,
    words: (F, F, F),
) {
    let xor_rows = layout(curr_row, bits);

    witness::init(
        witness,
        curr_row,
        &xor_rows,
        &variable_map!["in1" => words.0, "in2" => words.1, "out" => words.2],
    )
}

/// Extends the Xor rows to the full witness
pub fn extend_xor_rows<F: PrimeField>(
    witness: &mut [Vec<F>; COLUMNS],
    bits: u32,
    words: (F, F, F),
) {
    let xor_witness: [Vec<F>; COLUMNS] = array::from_fn(|_| vec![F::zero(); num_xors(bits) + 1]);
    let xor_row = witness[0].len();
    for col in 0..COLUMNS {
        witness[col].extend(xor_witness[col].iter());
    }
    init_xor(witness, xor_row, bits, words);
}

/// Create a Xor for up to the native length starting at row 0
/// Input: first input and second input, bits length, current row
/// Panics if the input is larger than the field
pub fn create_xor_witness<F: PrimeField>(
    input1: &BigUint,
    input2: &BigUint,
    bits: u32,
) -> [Vec<F>; COLUMNS] {
    if *input1 >= F::modulus_biguint() || *input2 >= F::modulus_biguint() {
        panic!("Input too large for the native field");
    }

    let output = big_xor(input1, input2);

    let mut xor_witness: [Vec<F>; COLUMNS] =
        array::from_fn(|_| vec![F::zero(); num_xors(bits) + 1]);

    init_xor(
        &mut xor_witness,
        0,
        bits,
        (
            F::from_biguint(input1).unwrap(),
            F::from_biguint(input2).unwrap(),
            F::from_biguint(&output).unwrap(),
        ),
    );

    xor_witness
}

/// Create a Not for less than 255 bits (native field) starting at row 0
/// Input: first input and second input
/// Panics if the input is too large for the field
pub fn create_not_xor_witness<F: PrimeField>(
    input: &BigUint,
    bits: Option<u32>,
) -> [Vec<F>; COLUMNS] {
    if *input > F::modulus_biguint() {
        panic!("This number must be split because it does not fit into the native field");
    }
    let output = big_not(input, bits);
    let bits = max(big_bits(input) as u32, bits.unwrap_or(0));
    let mut not_witness: [Vec<F>; COLUMNS] =
        array::from_fn(|_| vec![F::zero(); num_xors(bits) + 1]);
    init_xor(
        &mut not_witness,
        0,
        bits,
        (
            F::from_biguint(input).unwrap(),
            F::from(2u8).pow(&[bits as u64]) - F::one(),
            F::from_biguint(&output).unwrap(),
        ),
    );

    not_witness
}

/// Creates as many negations as the number of inputs. The inputs must fit in the native field.
/// We start at the row 0 using generic gates to perform the negations.
/// Input: a vector of words to be negated, and the number of bits (all the same)
/// Panics if the bits length is too small for the inputs, or if the inputs are too large for the field
/// INTEGRATION: Set public input of bits in public generic gate
/// TODO: `witness[0][pub] = 2^bits - 1`
pub fn create_not_gnrc_witness<F: PrimeField>(
    inputs: &Vec<BigUint>,
    bits: u32,
) -> [Vec<F>; COLUMNS] {
    // Check inputs fit in bits and in native field
    for input in inputs {
        if *input > F::modulus_biguint() {
            panic!("This number must be split because it does not fit into the native field");
        }
        if bits < big_bits(input) {
            panic!("Bits must be greater or equal than the inputs length");
        }
    }
    let all_ones = F::from(2u8).pow(&[bits as u64]) - F::one();
    let rows = (inputs.len() as f64 / 2.0).ceil() as usize;
    let mut not_witness = array::from_fn(|_| vec![F::zero(); rows]);
    for (i, input) in inputs.iter().enumerate().step_by(2) {
        let row = i / 2;
        // fill in first NOT
        let not1 = all_ones - F::from_biguint(input).unwrap();
        not_witness[0][row] = all_ones;
        not_witness[1][row] = F::from_biguint(input).unwrap();
        not_witness[2][row] = not1;
        // Next element exists
        if i < inputs.len() - 1 {
            let next = &inputs[i + 1];
            // fill in second NOT
            let not2 = all_ones - F::from_biguint(next).unwrap();
            not_witness[3][row] = all_ones;
            not_witness[4][row] = F::from_biguint(next).unwrap();
            not_witness[5][row] = not2;
        }
    }
    not_witness
}

/// Returns the number of XOR rows needed for inputs of usize bits
pub fn num_xors(bits: u32) -> usize {
    (bits as f64 / 16.0).ceil() as usize
}
