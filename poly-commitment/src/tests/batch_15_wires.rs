//! This module tests polynomial commitments, batched openings and
//! verification of a batch of batched opening proofs of polynomial commitments

use crate::{
    commitment::{combined_inner_product, BatchEvaluationProof, CommitmentCurve, Evaluation},
    evaluation_proof::DensePolynomialOrEvaluations,
    srs::SRS,
    SRS as _,
};
use ark_ec::AffineRepr;
use ark_ff::{UniformRand, Zero};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Radix2EvaluationDomain};
use colored::Colorize;
use groupmap::GroupMap;
use mina_curves::pasta::{Fp, Vesta, VestaParameters};
use mina_poseidon::constants::PlonkSpongeConstantsKimchi as SC;
use mina_poseidon::sponge::DefaultFqSponge;
use mina_poseidon::FqSponge;
use o1_utils::ExtendedDensePolynomial as _;
use rand::Rng;
use std::time::{Duration, Instant};

#[test]
fn dlog_commitment_test()
where
    <Fp as std::str::FromStr>::Err: std::fmt::Debug,
{
    let rng = &mut rand::thread_rng();
    let mut random = rand::thread_rng();

    let size = 1 << 7;
    let srs = SRS::<Vesta>::create(size);

    let num_chunks = 1;

    let group_map = <Vesta as CommitmentCurve>::Map::setup();

    let sponge = DefaultFqSponge::<VestaParameters, SC>::new(
        mina_poseidon::pasta::fq_kimchi::static_params(),
    );

    let mut commit = Duration::new(0, 0);
    let mut open = Duration::new(0, 0);

    let prfs = (0..7)
        .map(|_| {
            let length = (0..11)
                .map(|_| {
                    let polysize = 500;
                    let len: usize = random.gen();
                    (len % polysize) + 1
                })
                .collect::<Vec<_>>();
            println!("{}{:?}", "sizes: ".bright_cyan(), length);

            let a = length
                .iter()
                .map(|s| {
                    if *s == 0 {
                        DensePolynomial::<Fp>::zero()
                    } else {
                        DensePolynomial::<Fp>::rand(s - 1, rng)
                    }
                })
                .collect::<Vec<_>>();
            let bounds = a
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    if i % 2 == 0 {
                        Some(v.coeffs.len())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let x = (0..7).map(|_| Fp::rand(rng)).collect::<Vec<Fp>>();
            let polymask = Fp::rand(rng);
            let evalmask = Fp::rand(rng);

            let mut start = Instant::now();
            let comm = (0..a.len())
                .map(|i| {
                    (
                        srs.commit(&a[i].clone(), num_chunks, bounds[i], rng),
                        x.iter()
                            .map(|xx| a[i].to_chunked_polynomial(1, size).evaluate_chunks(*xx))
                            .collect::<Vec<_>>(),
                        bounds[i],
                    )
                })
                .collect::<Vec<_>>();
            commit += start.elapsed();

            start = Instant::now();
            let polys: Vec<(
                DensePolynomialOrEvaluations<_, Radix2EvaluationDomain<_>>,
                _,
                _,
            )> = (0..a.len())
                .map(|i| {
                    (
                        DensePolynomialOrEvaluations::DensePolynomial(&a[i]),
                        bounds[i],
                        (comm[i].0).blinders.clone(),
                    )
                })
                .collect();
            let proof = srs.open::<DefaultFqSponge<VestaParameters, SC>, _, _>(
                &group_map,
                &polys,
                &x,
                polymask,
                evalmask,
                sponge.clone(),
                rng,
            );
            open += start.elapsed();

            let combined_inner_product = {
                let es: Vec<_> = comm
                    .iter()
                    .map(|(commitment, evaluations, degree_bound)| {
                        let bound: Option<usize> = (|| {
                            let b = (*degree_bound)?;
                            let x = commitment.commitment.shifted?;
                            if x.is_zero() {
                                None
                            } else {
                                Some(b)
                            }
                        })();
                        (evaluations.clone(), bound)
                    })
                    .collect();
                combined_inner_product(&x, &polymask, &evalmask, &es, srs.g.len())
            };

            (
                sponge.clone(),
                x,
                polymask,
                evalmask,
                comm,
                proof,
                combined_inner_product,
            )
        })
        .collect::<Vec<_>>();

    let mut proofs = prfs
        .iter()
        .map(|proof| BatchEvaluationProof {
            sponge: proof.0.clone(),
            evaluation_points: proof.1.clone(),
            polyscale: proof.2,
            evalscale: proof.3,
            evaluations: proof
                .4
                .iter()
                .map(|poly| Evaluation {
                    commitment: (poly.0).commitment.clone(),
                    evaluations: poly.1.clone(),
                    degree_bound: poly.2,
                })
                .collect::<Vec<_>>(),
            opening: &proof.5,
            combined_inner_product: proof.6,
        })
        .collect::<Vec<_>>();

    println!("{}{:?}", "commitment time: ".yellow(), commit);
    println!("{}{:?}", "open time: ".magenta(), open);

    let start = Instant::now();
    assert!(srs.verify::<DefaultFqSponge<VestaParameters, SC>, _>(&group_map, &mut proofs, rng));
    println!("{}{:?}", "verification time: ".green(), start.elapsed());
}
