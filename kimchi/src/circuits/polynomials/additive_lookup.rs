use crate::{
    circuits::{
        expr::{prologue::*, Cache, Column, ConstantExpr},
        gate::{CircuitGate, CurrOrNext::*},
        lookup::{
            constraints::{zk_patch, LookupConfiguration},
            lookups::{JointLookupSpec, LocalPosition, LookupInfo, LookupPatterns},
            runtime_tables,
        },
        wires::COLUMNS,
    },
    error::ProverError,
};
use ark_ff::{FftField, One, PrimeField, Zero};
use ark_poly::{EvaluationDomain, Evaluations, Radix2EvaluationDomain as D};
use rand::Rng;
use std::collections::HashMap;

pub const ZK_ROWS: usize = 3;

pub const INVERSE_BATCH_SIZE: usize = 6;

pub fn num_inverses_columns(lookup_info: &LookupInfo) -> usize {
    (lookup_info.max_per_row + INVERSE_BATCH_SIZE - 1) / INVERSE_BATCH_SIZE
}

/// Specifies the additive lookup constraints as expressions.
///
/// # Panics
///
/// Will panic if single `element` length is bigger than `max_per_row` length.
pub fn constraints<F: FftField + PrimeField>(
    configuration: &LookupConfiguration<F>,
    generate_feature_flags: bool,
    cache: &mut Cache,
) -> Vec<E<F>> {
    // Something important to keep in mind is that the last 2 rows of
    // all columns will have random values in them to maintain zero-knowledge.
    //
    // Another important thing to note is that there are no lookups permitted
    // in the 3rd to last row.
    //
    // This is because computing the lookup-aggregation requires
    // num_lookup_rows + 1
    // rows, so we need to have
    // num_lookup_rows + 1 = n - 2 (the last 2 being reserved for the zero-knowledge random
    // values) and thus
    //
    // num_lookup_rows = n - 3
    let lookup_info = &configuration.lookup_info;

    let column = |col: Column| E::cell(col, Curr);

    // The contributions given by each lookup pattern.
    let lookup_contributions = {
        let f_term = |spec: &Vec<JointLookupSpec<_>>| {
            assert!(spec.len() <= lookup_info.max_per_row);

            spec.iter()
                .step_by(INVERSE_BATCH_SIZE)
                .enumerate()
                .map(|(i, _)| column(Column::AdditiveLookupInverse(i)))
                .fold(None, |acc: Option<E<F>>, x| match acc {
                    Some(acc) => Some(acc + x),
                    None => Some(x),
                })
        };

        lookup_info
            .features
            .patterns
            .into_iter()
            .filter_map(|spec| {
                let mut term =
                    column(Column::LookupKindIndex(spec)) * f_term(&spec.lookups::<F>())?;
                if generate_feature_flags {
                    term = E::IfFeature(
                        FeatureFlag::LookupPattern(spec),
                        Box::new(term),
                        Box::new(E::zero()),
                    )
                }
                Some(term)
            })
            .fold(None, |acc, x| match acc {
                Some(acc) => Some(acc + x),
                None => Some(x),
            })
    };

    // aggregation[i] = aggregation[i-1] + lookups - table
    // Therefore
    // table = aggregation[i-1] - aggregation[i] + lookups
    let expected_table = {
        let mut res = E::cell(Column::AdditiveLookupAggregation, Curr)
            - E::cell(Column::AdditiveLookupAggregation, Next);
        if let Some(lookup_contributions) = lookup_contributions {
            res += lookup_contributions;
        }
        res
    };

    // table = count / (beta + table_entry)
    // (beta + table_entry) * table = count
    let aggreg_equation = {
        (E::Constant(ConstantExpr::Beta) + E::cell(Column::LookupTable, Curr)) * expected_table
            - E::cell(Column::AdditiveLookupCount, Curr)
    };

    let final_lookup_row: i32 = -(ZK_ROWS as i32) - 1;

    let mut res = vec![
        // the accumulator except for the last 4 rows
        // (contains the zk-rows and the last value of the accumulator)
        E::VanishesOnLast4Rows * aggreg_equation,
        // the initial value of the accumulator
        E::UnnormalizedLagrangeBasis(0)
            * (E::cell(Column::AdditiveLookupAggregation, Curr) - E::zero()),
        // Check that the final value of the accumulator is 1
        E::UnnormalizedLagrangeBasis(final_lookup_row)
            * (E::cell(Column::AdditiveLookupAggregation, Curr) - E::zero()),
    ];

    // Add the inverse constraints.
    {
        let joint_combiner = E::Constant(ConstantExpr::JointCombiner);
        let table_id_combiner =
            // Compute `joint_combiner.pow(lookup_info.max_joint_size)`, injecting feature flags if
            // needed.
            (1..lookup_info.max_joint_size).fold(joint_combiner.clone(), |acc, i| {
                let mut new_term = joint_combiner.clone();
                if generate_feature_flags {
                    new_term = E::IfFeature(
                        FeatureFlag::TableWidth((i + 1) as isize),
                        Box::new(new_term),
                        Box::new(E::one()),
                    );
                }
                acc * new_term
            });

        // Note: we iterate over all possible patterns to ensure that the constraints always end up
        // at the same position in the list. For constraints that we aren't using, we hard-code
        // them to 0.
        let all_lookup_patterns = LookupPatterns {
            xor: true,
            lookup: true,
            range_check: true,
            foreign_field_mul: true,
        };
        for spec in all_lookup_patterns.into_iter() {
            let is_used = lookup_info.features.patterns[spec];
            let mut product_of_lookups: Option<E<_>> = None;
            let mut sum_of_products_of_lookups: Option<E<_>> = None;
            let mut inverse_count = 0;
            let mut insert_inverses_constraint =
                |product_of_lookups: &mut Option<E<_>>,
                 sum_of_products_of_lookups: &mut Option<E<_>>,
                 inverse_count: &mut usize| {
                    let product_of_lookups = product_of_lookups.take();
                    let sum_of_products_of_lookups = sum_of_products_of_lookups.take();
                    match (product_of_lookups, sum_of_products_of_lookups) {
                        (Some(product_of_lookups), Some(sum_of_products_of_lookups)) => {
                            let mut term = if is_used {
                                // Assert that AdditiveLookupInverse(i)
                                // = 1/(lookup[6*i]) + 1/(lookup[6*i+1]) + ... + 1/(lookup[6*i+5).
                                // Since we can't perform division, instead we multiply through by the product
                                // of all of the lookups (product_of_lookups) and assert that that is equal to
                                // the sums of all products of 5 out of the 6 lookups
                                // (sum_of_products_of_lookups).
                                let res = column(Column::LookupKindIndex(spec))
                                    * (product_of_lookups
                                        * column(Column::AdditiveLookupInverse(*inverse_count))
                                        - sum_of_products_of_lookups);
                                *inverse_count += 1;
                                res
                            } else {
                                E::zero()
                            };
                            if generate_feature_flags {
                                term = E::IfFeature(
                                    FeatureFlag::LookupPattern(spec),
                                    Box::new(term),
                                    Box::new(E::zero()),
                                )
                            }
                            res.push(term)
                        }
                        _ => (),
                    }
                };
            for (i, joint_lookup) in spec.lookups::<F>().into_iter().enumerate() {
                let eval = |pos: LocalPosition| witness(pos.column, pos.row);
                let lookup_term = E::Constant(ConstantExpr::Beta)
                    + joint_lookup.evaluate(&joint_combiner, &table_id_combiner, &eval);
                let lookup_term = cache.cache(lookup_term);
                // We are gradually constructing the value
                // a_1 * a_2 * .. * a_n + a_0 * a_2 * a_3 * ... * a_n + a_0 * a_1 * .. * a_{n-1}.
                {
                    let new_sum_of_products_of_lookups = match sum_of_products_of_lookups.take() {
                        None => Some(E::one()),
                        Some(e) => Some(
                            e * lookup_term.clone() + product_of_lookups.as_ref().unwrap().clone(),
                        ),
                    };
                    sum_of_products_of_lookups = new_sum_of_products_of_lookups;
                }
                // We then update the product to include the current term as well.
                product_of_lookups = match product_of_lookups.take() {
                    None => Some(lookup_term),
                    Some(product_of_lookups) => Some(product_of_lookups * lookup_term),
                };
                // We multiply in batches of INVERSE_BATCH_SIZE, to keep the degree low enough.
                if i % INVERSE_BATCH_SIZE == INVERSE_BATCH_SIZE - 1 {
                    insert_inverses_constraint(
                        &mut product_of_lookups,
                        &mut sum_of_products_of_lookups,
                        &mut inverse_count,
                    );
                }
            }
            // The final batch may be incomplete; if so, add it here.
            insert_inverses_constraint(
                &mut product_of_lookups,
                &mut sum_of_products_of_lookups,
                &mut inverse_count,
            );
        }
    }

    // if we are using runtime tables, we add:
    // $RT(x) (1 - \text{selector}_{RT}(x)) = 0$
    if configuration.lookup_info.features.uses_runtime_tables {
        let mut rt_constraints = runtime_tables::constraints();
        if generate_feature_flags {
            for term in rt_constraints.iter_mut() {
                // Dummy value, to appease the borrow checker.
                let mut boxed_term = Box::new(constant(F::zero()));
                std::mem::swap(term, &mut *boxed_term);
                *term = E::IfFeature(
                    FeatureFlag::RuntimeLookupTables,
                    boxed_term,
                    Box::new(E::zero()),
                )
            }
        }
        res.extend(rt_constraints);
    }

    res
}

pub struct ComputedColumns<F> {
    pub counts: F,
    pub aggregation: F,
    pub inverses: Vec<F>,
}

/// Compute the aggregation and counts polynomials
#[allow(clippy::too_many_arguments)]
pub fn compute_aggregations<R: Rng + ?Sized, F: PrimeField>(
    joint_lookup_table_d8: &Evaluations<F, D<F>>,
    d1: D<F>,
    gates: &[CircuitGate<F>],
    witness: &[Vec<F>; COLUMNS],
    joint_combiner: F,
    table_id_combiner: F,
    lookup_info: &LookupInfo,
    beta: F,
    rng: &mut R,
) -> Result<ComputedColumns<Evaluations<F, D<F>>>, ProverError> {
    let n = d1.size();
    let lookup_rows = n - ZK_ROWS - 1;

    // Compute the inversions corresponding to the table, and create a map from table values to
    // their index in the lookup table.
    let (joint_lookup_value_to_index_map, negative_inverted_lookup_values_offset_by_one) = {
        let mut joint_lookup_value_to_index_map: HashMap<F, usize> = HashMap::new();

        // Optimisation: we will later reuse this as our aggregation, so we initialize it with a
        // placeholder value (one) for batch inversion that we can replace.
        let mut lookup_values_offset_by_one = Vec::with_capacity(d1.size());
        lookup_values_offset_by_one.push(F::one());

        for (i, joint_lookup_value) in joint_lookup_table_d8
            .evals
            .iter()
            .step_by(8)
            .take(lookup_rows)
            .enumerate()
        {
            lookup_values_offset_by_one.push(beta + *joint_lookup_value);
            joint_lookup_value_to_index_map
                .entry(*joint_lookup_value)
                .or_insert(i);
        }
        let mut negative_inverted_lookup_values_offset_by_one = lookup_values_offset_by_one;
        ark_ff::batch_inversion_and_mul(
            &mut negative_inverted_lookup_values_offset_by_one,
            &-F::one(),
        );
        (
            joint_lookup_value_to_index_map,
            negative_inverted_lookup_values_offset_by_one,
        )
    };

    let mut counts = vec![0u64; lookup_rows];
    let num_inverses_columns = num_inverses_columns(lookup_info);
    let mut inverses = vec![Vec::with_capacity(d1.size()); num_inverses_columns];

    let by_row = lookup_info.by_row(gates);

    // Accumulate the counts for each value in the lookup table, and use the index map to look up
    // the precomputed inverses for each looked-up value.
    for (i, spec) in by_row
        .iter()
        .enumerate()
        // avoid zk rows
        .take(lookup_rows)
    {
        let num_lookups = spec.len();
        for (j, joint_lookup) in spec.iter().enumerate() {
            let eval = |pos: LocalPosition| -> F {
                let row = match pos.row {
                    Curr => i,
                    Next => i + 1,
                };
                witness[pos.column][row]
            };
            // Compute the value that will appear in the joint lookup table.
            let joint_lookup_evaluation =
                joint_lookup.evaluate(&joint_combiner, &table_id_combiner, &eval);
            // Find the index of the value in the table
            let index = joint_lookup_value_to_index_map
                .get(&joint_lookup_evaluation)
                .ok_or(ProverError::ValueNotInTable)?;
            // Use the cached inverted values from the table to insert the corresponding inverse.
            if j % INVERSE_BATCH_SIZE == 0 {
                inverses[j / INVERSE_BATCH_SIZE]
                    .push(-negative_inverted_lookup_values_offset_by_one[index + 1]);
            } else {
                inverses[j / INVERSE_BATCH_SIZE][i] -=
                    negative_inverted_lookup_values_offset_by_one[index + 1];
            }
            // Increase the count for the lookup entry by one.
            counts[*index] += 1u64;
        }
        for inverses in inverses
            .iter_mut()
            .take(num_inverses_columns)
            .skip(num_lookups)
        {
            inverses.push(F::zero());
        }
    }

    // Convert the usage counts for each entry to a field element.
    let counts: Vec<F> = counts.into_iter().map(Into::into).collect();

    // We have now finished using this as an inversion cache, so we can compute the aggregation.
    let mut aggregation = negative_inverted_lookup_values_offset_by_one;

    // Replace the placeholder in the first entry with the initial value of the aggregation
    // polynomial.
    aggregation[0] = F::zero();

    for (i, spec) in by_row.iter().enumerate().take(lookup_rows) {
        // Scale the table inverse by its number of uses
        aggregation[i + 1] *= counts[i];
        // Cascade the aggregation from the previous row through to this one.
        let acc = aggregation[i];
        aggregation[i + 1] += acc;
        for (j, _) in spec.iter().enumerate().step_by(INVERSE_BATCH_SIZE) {
            // Add the inverse term for this lookup (already computed above).
            aggregation[i + 1] += inverses[j][i];
        }
    }

    // Add randomness to the last ZK_ROWS rows of each polynomial, to provide zero-knowledge.
    let counts = zk_patch(counts, d1, rng);
    let aggregation = zk_patch(aggregation, d1, rng);
    let inverses = inverses.into_iter().map(|x| zk_patch(x, d1, rng)).collect();

    assert_eq!(F::zero(), aggregation[0]);
    assert_eq!(F::zero(), aggregation[lookup_rows]);

    Ok(ComputedColumns {
        counts,
        aggregation,
        inverses,
    })
}
