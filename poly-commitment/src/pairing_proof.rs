use crate::commitment::*;
use crate::evaluation_proof::{combine_polys, DensePolynomialOrEvaluations};
use crate::srs::SRS;
use crate::{CommitmentError, PolynomialsToCombine, SRS as SRSTrait};
use ark_ec::pairing::Pairing;
use ark_ec::AffineRepr;
use ark_ec::VariableBaseMSM;
use ark_ff::{PrimeField, Zero};
use ark_poly::{
    univariate::{DenseOrSparsePolynomial, DensePolynomial},
    DenseUVPolynomial, EvaluationDomain, Evaluations, Polynomial, Radix2EvaluationDomain as D,
};
use mina_poseidon::FqSponge;
use rand_core::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(
    bound = "Pair::G1Affine: ark_serialize::CanonicalDeserialize + ark_serialize::CanonicalSerialize"
)]
pub struct PairingProof<Pair: Pairing> {
    #[serde_as(as = "o1_utils::serialization::SerdeAs")]
    pub quotient: Pair::G1Affine,
    #[serde_as(as = "o1_utils::serialization::SerdeAs")]
    pub blinding: <Pair::G1Affine as AffineRepr>::ScalarField,
}

impl<Pair: Pairing> Default for PairingProof<Pair> {
    fn default() -> Self {
        Self {
            quotient: Pair::G1Affine::generator().mul_by_cofactor(),
            blinding: <Pair::G1Affine as AffineRepr>::ScalarField::zero(),
        }
    }
}

impl<Pair: Pairing> Clone for PairingProof<Pair> {
    fn clone(&self) -> Self {
        Self {
            quotient: self.quotient,
            blinding: self.blinding,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairingSRS<Pair: Pairing> {
    pub full_srs: SRS<Pair::G1Affine>,
    pub verifier_srs: SRS<Pair::G2Affine>,
}

impl<Pair: Pairing> Default for PairingSRS<Pair> {
    fn default() -> Self {
        Self {
            full_srs: SRS::default(),
            verifier_srs: SRS::default(),
        }
    }
}

impl<Pair: Pairing> Clone for PairingSRS<Pair> {
    fn clone(&self) -> Self {
        Self {
            full_srs: self.full_srs.clone(),
            verifier_srs: self.verifier_srs.clone(),
        }
    }
}

impl<
        F: PrimeField,
        G: CommitmentCurve<ScalarField = F>,
        G2: CommitmentCurve<ScalarField = F>,
        Pair: Pairing<G1Affine = G, G2Affine = G2>,
    > PairingSRS<Pair>
{
    pub fn create(x: F, n: usize) -> Self {
        PairingSRS {
            full_srs: SRS::create_trusted_setup(x, n),
            verifier_srs: SRS::create_trusted_setup(x, 3),
        }
    }
}

impl<
        F: PrimeField,
        G: CommitmentCurve<ScalarField = F>,
        G2: CommitmentCurve<ScalarField = F>,
        Pair: Pairing<G1Affine = G, G2Affine = G2>,
    > crate::OpenProof<G> for PairingProof<Pair>
{
    type SRS = PairingSRS<Pair>;

    fn open<EFqSponge, RNG, D: EvaluationDomain<F>>(
        srs: &Self::SRS,
        _group_map: &<G as CommitmentCurve>::Map,
        plnms: &[(
            DensePolynomialOrEvaluations<F, D>,
            Option<usize>,
            PolyComm<F>,
        )], // vector of polynomial with optional degree bound and commitment randomness
        elm: &[F],          // vector of evaluation points
        polyscale: F,       // scaling factor for polynoms
        _evalscale: F,      // scaling factor for evaluation point powers
        _sponge: EFqSponge, // sponge
        _rng: &mut RNG,
    ) -> Self
    where
        EFqSponge: Clone + FqSponge<<G as AffineRepr>::BaseField, G, F>,
        RNG: RngCore + CryptoRng,
    {
        PairingProof::create(srs, plnms, elm, polyscale).unwrap()
    }

    fn verify<EFqSponge, RNG>(
        srs: &Self::SRS,
        _group_map: &G::Map,
        batch: &mut [BatchEvaluationProof<G, EFqSponge, Self>],
        _rng: &mut RNG,
    ) -> bool
    where
        EFqSponge: FqSponge<G::BaseField, G, F>,
        RNG: RngCore + CryptoRng,
    {
        for BatchEvaluationProof {
            sponge: _,
            evaluations,
            evaluation_points,
            polyscale,
            evalscale: _,
            opening,
            combined_inner_product: _,
        } in batch.iter()
        {
            if !opening.verify(srs, evaluations, *polyscale, evaluation_points) {
                return false;
            }
        }
        true
    }
}

impl<
        F: PrimeField,
        G: CommitmentCurve<ScalarField = F>,
        G2: CommitmentCurve<ScalarField = F>,
        Pair: Pairing<G1Affine = G, G2Affine = G2>,
    > SRSTrait<G> for PairingSRS<Pair>
{
    fn max_poly_size(&self) -> usize {
        self.full_srs.max_poly_size()
    }

    fn get_lagrange_basis(&self, domain_size: usize) -> Option<&Vec<PolyComm<G>>> {
        self.full_srs.get_lagrange_basis(domain_size)
    }

    fn blinding_commitment(&self) -> G {
        self.full_srs.blinding_commitment()
    }

    fn commit(
        &self,
        plnm: &DensePolynomial<F>,
        num_chunks: usize,
        max: Option<usize>,
        rng: &mut (impl RngCore + CryptoRng),
    ) -> BlindedCommitment<G> {
        self.full_srs.commit(plnm, num_chunks, max, rng)
    }

    fn mask_custom(
        &self,
        com: PolyComm<G>,
        blinders: &PolyComm<F>,
    ) -> Result<BlindedCommitment<G>, CommitmentError> {
        self.full_srs.mask_custom(com, blinders)
    }

    fn mask(
        &self,
        comm: PolyComm<G>,
        rng: &mut (impl RngCore + CryptoRng),
    ) -> BlindedCommitment<G> {
        self.full_srs.mask(comm, rng)
    }

    fn commit_non_hiding(
        &self,
        plnm: &DensePolynomial<F>,
        num_chunks: usize,
        max: Option<usize>,
    ) -> PolyComm<G> {
        self.full_srs.commit_non_hiding(plnm, num_chunks, max)
    }

    fn commit_evaluations_non_hiding(
        &self,
        domain: D<F>,
        plnm: &Evaluations<F, D<F>>,
    ) -> PolyComm<G> {
        self.full_srs.commit_evaluations_non_hiding(domain, plnm)
    }

    fn commit_evaluations(
        &self,
        domain: D<F>,
        plnm: &Evaluations<F, D<F>>,
        rng: &mut (impl RngCore + CryptoRng),
    ) -> BlindedCommitment<G> {
        self.full_srs.commit_evaluations(domain, plnm, rng)
    }

    fn create(_depth: usize) -> Self {
        todo!()
    }

    fn add_lagrange_basis(&mut self, domain: D<F>) {
        self.full_srs.add_lagrange_basis(domain);
    }

    fn size(&self) -> usize {
        self.full_srs.g.len()
    }
}

/// The polynomial that evaluates to each of `evals` for the respective `elm`s.
fn eval_polynomial<F: PrimeField>(elm: &[F], evals: &[F]) -> DensePolynomial<F> {
    assert_eq!(elm.len(), evals.len());
    let (zeta, zeta_omega) = if elm.len() == 2 {
        (elm[0], elm[1])
    } else {
        todo!()
    };
    let (eval_zeta, eval_zeta_omega) = if evals.len() == 2 {
        (evals[0], evals[1])
    } else {
        todo!()
    };

    // The polynomial that evaluates to `p(zeta)` at `zeta` and `p(zeta_omega)` at
    // `zeta_omega`.
    // We write `p(x) = a + bx`, which gives
    // ```text
    // p(zeta) = a + b * zeta
    // p(zeta_omega) = a + b * zeta_omega
    // ```
    // and so
    // ```text
    // b = (p(zeta_omega) - p(zeta)) / (zeta_omega - zeta)
    // a = p(zeta) - b * zeta
    // ```
    let b = (eval_zeta_omega - eval_zeta) / (zeta_omega - zeta);
    let a = eval_zeta - b * zeta;
    DensePolynomial::from_coefficients_slice(&[a, b])
}

/// The polynomial that evaluates to `0` at the evaluation points.
fn divisor_polynomial<F: PrimeField>(elm: &[F]) -> DensePolynomial<F> {
    elm.iter()
        .map(|value| DensePolynomial::from_coefficients_slice(&[-(*value), F::one()]))
        .reduce(|poly1, poly2| &poly1 * &poly2)
        .unwrap()
}

impl<
        F: PrimeField,
        G: CommitmentCurve<ScalarField = F>,
        G2: CommitmentCurve<ScalarField = F>,
        Pair: Pairing<G1Affine = G, G2Affine = G2>,
    > PairingProof<Pair>
{
    pub fn create<D: EvaluationDomain<F>>(
        srs: &PairingSRS<Pair>,
        plnms: PolynomialsToCombine<G, D>, // vector of polynomial with optional degree bound and commitment randomness
        elm: &[F],                         // vector of evaluation points
        polyscale: F,                      // scaling factor for polynoms
    ) -> Option<Self> {
        let (p, blinding_factor) = combine_polys::<G, D>(plnms, polyscale, srs.full_srs.g.len());
        let evals: Vec<_> = elm.iter().map(|pt| p.evaluate(pt)).collect();

        let quotient_poly = {
            let eval_polynomial = eval_polynomial(elm, &evals);
            let divisor_polynomial = divisor_polynomial(elm);
            let numerator_polynomial = &p - &eval_polynomial;
            let (quotient, remainder) = DenseOrSparsePolynomial::divide_with_q_and_r(
                &numerator_polynomial.into(),
                &divisor_polynomial.into(),
            )?;
            if !remainder.is_zero() {
                return None;
            }
            quotient
        };

        let quotient = srs
            .full_srs
            .commit_non_hiding(&quotient_poly, 1, None)
            .unshifted[0];

        Some(PairingProof {
            quotient,
            blinding: blinding_factor,
        })
    }

    pub fn verify(
        &self,
        srs: &PairingSRS<Pair>,           // SRS
        evaluations: &Vec<Evaluation<G>>, // commitments to the polynomials
        polyscale: F,                     // scaling factor for polynoms
        elm: &[F],                        // vector of evaluation points
    ) -> bool {
        let poly_commitment = {
            let mut scalars: Vec<F> = Vec::new();
            let mut points: Vec<G> = Vec::new();
            combine_commitments(
                evaluations,
                &mut scalars,
                &mut points,
                polyscale,
                F::one(), /* TODO: This is inefficient */
            );
            let scalars: Vec<_> = scalars.iter().map(|x| x.into_bigint()).collect();
            G::Group::msm_bigint(&points, &scalars)
        };
        let evals = combine_evaluations(evaluations, polyscale);
        let blinding_commitment = srs.full_srs.h.mul(self.blinding);
        let eval_commitment = srs
            .full_srs
            .commit_non_hiding(&eval_polynomial(elm, &evals), 1, None)
            .unshifted[0]
            .into_group();
        let numerator_commitment = { poly_commitment - eval_commitment - blinding_commitment };
        let g = Pair::G2Affine::generator();
        // FIXME: we should not do two pairing. We should do one miller loop +
        // final exponentiation and check it is equals to 1
        let numerator = Pair::pairing(numerator_commitment.into(), g);

        // Right side pairing
        let divisor_commitment = srs
            .verifier_srs
            .commit_non_hiding(&divisor_polynomial(elm), 1, None)
            .unshifted[0];
        let scaled_quotient = Pair::pairing(self.quotient, divisor_commitment);
        numerator == scaled_quotient
    }
}

#[cfg(test)]
mod tests {
    use super::{PairingProof, PairingSRS};
    use crate::commitment::Evaluation;
    use crate::evaluation_proof::DensePolynomialOrEvaluations;
    use crate::srs::SRS;
    use crate::SRS as _;
    use ark_bn254::{Bn254, Fr as ScalarField};
    use ark_bn254::{G1Affine as G1, G2Affine as G2};
    use ark_ff::UniformRand;
    use ark_poly::{
        univariate::DensePolynomial, DenseUVPolynomial, EvaluationDomain, Polynomial,
        Radix2EvaluationDomain as D,
    };

    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn test_pairing_proof() {
        let n = 64;
        let domain = D::<ScalarField>::new(n).unwrap();

        let rng = &mut StdRng::from_seed([0u8; 32]);

        let x = ScalarField::rand(rng);

        let mut srs = SRS::<G1>::create_trusted_setup(x, n);
        let verifier_srs = SRS::<G2>::create_trusted_setup(x, 3);
        srs.add_lagrange_basis(domain);

        let srs = PairingSRS {
            full_srs: srs,
            verifier_srs,
        };

        let polynomials: Vec<_> = (0..4)
            .map(|_| {
                let coeffs = (0..63).map(|_| ScalarField::rand(rng)).collect();
                DensePolynomial::from_coefficients_vec(coeffs)
            })
            .collect();

        let comms: Vec<_> = polynomials
            .iter()
            .map(|p| srs.full_srs.commit(p, 1, None, rng))
            .collect();

        let polynomials_and_blinders: Vec<(DensePolynomialOrEvaluations<_, D<_>>, _, _)> =
            polynomials
                .iter()
                .zip(comms.iter())
                .map(|(p, comm)| {
                    let p = DensePolynomialOrEvaluations::DensePolynomial(p);
                    (p, None, comm.blinders.clone())
                })
                .collect();

        let evaluation_points = vec![ScalarField::rand(rng), ScalarField::rand(rng)];

        let evaluations: Vec<_> = polynomials
            .iter()
            .zip(comms)
            .map(|(p, commitment)| {
                let evaluations = evaluation_points
                    .iter()
                    .map(|x| {
                        // Inputs are chosen to use only 1 chunk
                        vec![p.evaluate(x)]
                    })
                    .collect();
                Evaluation {
                    commitment: commitment.commitment,
                    evaluations,
                    degree_bound: None,
                }
            })
            .collect();

        let polyscale = ScalarField::rand(rng);

        let pairing_proof = PairingProof::<Bn254>::create(
            &srs,
            polynomials_and_blinders.as_slice(),
            &evaluation_points,
            polyscale,
        )
        .unwrap();

        let res = pairing_proof.verify(&srs, &evaluations, polyscale, &evaluation_points);
        assert!(res);
    }
}
