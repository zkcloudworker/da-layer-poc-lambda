use ark_bn254::Fr;
use ark_ff::{Field, PrimeField};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain as D};
use groth16_plus_lookups::{
    prover::{prove_stage_1, prove_stage_2},
    proving_key::{trusted_setup, CircuitLayout},
    verifier::verify,
};

type BN254 = ark_ec::bn::Bn<ark_bn254::Parameters>;

struct LayoutPerRow {
    a: Vec<(usize, Fr)>,
    b: Vec<(usize, Fr)>,
    c: Vec<(usize, Fr)>,
    a_delayed: Vec<(usize, Fr)>,
}

fn create_layout(
    domain_size: usize,
    public_input_size: usize,
    witness_size: usize,
    layout: Vec<LayoutPerRow>,
) -> CircuitLayout<Fr> {
    let domain = D::new(domain_size).unwrap();
    let domain_d2 = D::new(domain_size << 1).unwrap();

    let mut a_contributions = vec![vec![]; witness_size];
    let mut b_contributions = vec![vec![]; witness_size];
    let mut c_contributions = vec![vec![]; witness_size];
    let mut a_delayed_contributions = vec![vec![]; witness_size];
    let mut c_delayed_equality_contributions = vec![vec![]; witness_size];

    for (row_idx, row_layout) in layout.into_iter().enumerate() {
        for (idx, scalar) in row_layout.a {
            a_contributions[idx].push((row_idx, scalar));
        }
        for (idx, scalar) in row_layout.b {
            b_contributions[idx].push((row_idx, scalar));
        }
        for (idx, scalar) in row_layout.c {
            c_contributions[idx].push((row_idx, scalar));
        }
        for (idx, scalar) in row_layout.a_delayed {
            a_delayed_contributions[idx].push((row_idx, scalar));
            // May not need a separate field for this if not useful.
            c_delayed_equality_contributions[idx].push((row_idx, scalar));
        }
    }

    CircuitLayout {
        public_input_size,
        a_contributions: a_contributions
            .into_iter()
            .map(|x| x.into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        a_delayed_contributions: a_delayed_contributions
            .into_iter()
            .map(|x| x.into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        b_contributions: b_contributions
            .into_iter()
            .map(|x| x.into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        c_contributions: c_contributions
            .into_iter()
            .map(|x| x.into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        c_delayed_equality_contributions: c_delayed_equality_contributions
            .into_iter()
            .map(|x| x.into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        domain,
        domain_d2,
    }
}

struct Lookup {
    scalar: Fr,
    idx: usize,
}

pub fn main() {
    let size = 1 << 7;
    let public_input_size = 3;

    let mut witness = vec![];
    let mut constraints = vec![];
    let mut lookups = vec![];
    let mut store = |x| {
        let idx = witness.len();
        witness.push(x);
        idx
    };

    // Public
    let constant_1 = store(Fr::from(1u64));
    let delayed_lookup_randomizer = store(Fr::from(0u64));
    let delayed_lookup_table_combiner = store(Fr::from(0u64));

    // Private
    let x1 = store(Fr::from(4u64));
    // x1 = 2 * 2
    constraints.push(LayoutPerRow {
        a: vec![(constant_1, Fr::from(1u64))],
        b: vec![(constant_1, Fr::from(2u64))],
        c: vec![(x1, Fr::from(1u64))],
        a_delayed: vec![(constant_1, Fr::from(1u64))],
    });
    let x2 = store(Fr::from(16u64));
    // x2 = x1 * x1
    constraints.push(LayoutPerRow {
        a: vec![(x1, Fr::from(1u64))],
        b: vec![(x1, Fr::from(1u64))],
        c: vec![(x2, Fr::from(1u64))],
        a_delayed: vec![],
    });

    // Lookup table initialisation
    let mut delayed_lookup_inverse_indicies = Vec::with_capacity(16);
    let mut delayed_lookup_multiplicities = Vec::with_capacity(16);
    let mut delayed_lookup_terms = Vec::with_capacity(16);
    for lookup_value in 0..16u64 {
        let delayed_inv = store(Fr::from(0u64));
        delayed_lookup_inverse_indicies.push(delayed_inv);
        // (r + i) * inv = 1
        constraints.push(LayoutPerRow {
            a: vec![(constant_1, Fr::from(lookup_value))],
            b: vec![(delayed_inv, Fr::from(1u64))],
            c: vec![(constant_1, Fr::from(1u64))],
            a_delayed: vec![(delayed_lookup_randomizer, Fr::from(1u64))],
        });
        let delayed_multiplicity = store(Fr::from(0u64));
        delayed_lookup_multiplicities.push(delayed_multiplicity);
        let delayed_lookup_term = store(Fr::from(0u64));
        delayed_lookup_terms.push(delayed_lookup_term);
        // m * inv = m_inv
        constraints.push(LayoutPerRow {
            a: vec![(delayed_multiplicity, Fr::from(1u64))],
            b: vec![(delayed_inv, Fr::from(1u64))],
            c: vec![(delayed_lookup_term, Fr::from(1u64))],
            a_delayed: vec![],
        });
        lookups.push(Lookup {
            scalar: Fr::from(1u64),
            idx: delayed_lookup_term,
        });
    }

    // Lookup x1
    let delayed_lookup_x1_inv = {
        let delayed_inv = store(Fr::from(0u64));
        // (r + i) * inv = 1
        constraints.push(LayoutPerRow {
            a: vec![(x1, Fr::from(1u64))],
            b: vec![(delayed_inv, Fr::from(1u64))],
            c: vec![(constant_1, Fr::from(1u64))],
            a_delayed: vec![(delayed_lookup_randomizer, Fr::from(1u64))],
        });
        lookups.push(Lookup {
            scalar: -Fr::from(1u64),
            idx: delayed_inv,
        });
        delayed_inv
    };

    // Final lookup check
    constraints.push(LayoutPerRow {
        a: vec![],
        b: vec![(constant_1, Fr::from(1u64))],
        c: vec![],
        a_delayed: lookups
            .into_iter()
            .map(|Lookup { scalar, idx }| (idx, scalar))
            .collect(),
    });

    let layout = create_layout(size, public_input_size, witness.len(), constraints);

    // Finalize lookup multiplicities
    {
        // Finalize lookup x1
        let _x1_value = witness[x1];
        let x1_value_idx: usize = /* TODO */ 4;
        witness[delayed_lookup_multiplicities[x1_value_idx]] += Fr::from(1u64);
    }

    let (prover_setup, vk) = trusted_setup::<_, _, BN254>(&layout, &mut rand::rngs::OsRng);

    let prover_env = prove_stage_1::<_, _, BN254>(
        witness.as_slice(),
        &prover_setup,
        &layout,
        &mut rand::rngs::OsRng,
    );

    // Update witness with now-known values
    {
        let lookup_randomizer = prover_env.neg_a_digest.0;
        let lookup_table_combiner = prover_env.neg_a_digest.1;

        witness[delayed_lookup_randomizer] = lookup_randomizer;
        witness[delayed_lookup_table_combiner] = lookup_table_combiner;

        for lookup_value in 0..16u64 {
            witness[delayed_lookup_inverse_indicies[lookup_value as usize]] = (lookup_randomizer
                + Fr::from(lookup_value))
            .inverse()
            .unwrap();
        }

        // Handle lookup for x1
        {
            let x1_value = witness[x1];
            witness[delayed_lookup_x1_inv] = (lookup_randomizer + x1_value).inverse().unwrap();
        }

        // Handle multiplicities
        for (inverse, (multiplicity, term)) in delayed_lookup_inverse_indicies.into_iter().zip(
            delayed_lookup_multiplicities
                .into_iter()
                .zip(delayed_lookup_terms.into_iter()),
        ) {
            witness[term] = witness[inverse] * witness[multiplicity];
        }
    }

    let proof = prove_stage_2::<_, BN254>(prover_env, witness.as_slice(), &prover_setup, &layout);

    let public_input: Vec<_> = witness[0..public_input_size]
        .into_iter()
        .map(|x| x.into_repr())
        .collect();

    let verifies = verify::<BN254>(public_input.as_slice(), &proof, &vk);

    println!("verifies? {}", verifies);
}
