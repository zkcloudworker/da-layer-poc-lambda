use super::framework::TestFramework;
use crate::circuits::polynomials::generic::testing::{create_circuit, fill_in_witness};
use crate::circuits::wires::KIMCHI_COLS;
use ark_ff::Zero;
use mina_curves::pasta::{Fp, Vesta, VestaParameters};
use mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    sponge::{DefaultFqSponge, DefaultFrSponge},
};
use std::array;

type SpongeParams = PlonkSpongeConstantsKimchi;
type BaseSponge = DefaultFqSponge<VestaParameters, SpongeParams>;
type ScalarSponge = DefaultFrSponge<Fp, SpongeParams>;

#[test]
fn test_generic_gate() {
    let gates = create_circuit::<Fp, KIMCHI_COLS>(0, 0);

    // create witness
    let mut witness: [Vec<Fp>; KIMCHI_COLS] = array::from_fn(|_| vec![Fp::zero(); gates.len()]);
    fill_in_witness(0, &mut witness, &[]);

    // create and verify proof based on the witness
    TestFramework::<Vesta>::default()
        .gates(gates)
        .witness(witness)
        .setup()
        .prove_and_verify::<BaseSponge, ScalarSponge>()
        .unwrap();
}

#[test]
fn test_generic_gate_pub() {
    let public = vec![Fp::from(3u8); 5];
    let gates = create_circuit::<Fp, KIMCHI_COLS>(0, public.len());

    // create witness
    let mut witness: [Vec<Fp>; KIMCHI_COLS] = array::from_fn(|_| vec![Fp::zero(); gates.len()]);
    fill_in_witness(0, &mut witness, &public);

    // create and verify proof based on the witness
    TestFramework::<Vesta>::default()
        .gates(gates)
        .witness(witness)
        .public_inputs(public)
        .setup()
        .prove_and_verify::<BaseSponge, ScalarSponge>()
        .unwrap();
}

#[test]
fn test_generic_gate_pub_all_zeros() {
    let public = vec![Fp::from(0u8); 5];
    let gates = create_circuit::<Fp, KIMCHI_COLS>(0, public.len());

    // create witness
    let mut witness: [Vec<Fp>; KIMCHI_COLS] = array::from_fn(|_| vec![Fp::zero(); gates.len()]);
    fill_in_witness(0, &mut witness, &public);

    // create and verify proof based on the witness
    TestFramework::<Vesta, KIMCHI_COLS>::default()
        .gates(gates)
        .witness(witness)
        .public_inputs(public)
        .setup()
        .prove_and_verify::<BaseSponge, ScalarSponge>()
        .unwrap();
}

#[test]
fn test_generic_gate_pub_empty() {
    let public = vec![];
    let gates = create_circuit::<Fp, KIMCHI_COLS>(0, public.len());

    // create witness
    let mut witness: [Vec<Fp>; KIMCHI_COLS] = array::from_fn(|_| vec![Fp::zero(); gates.len()]);
    fill_in_witness(0, &mut witness, &public);

    // create and verify proof based on the witness
    TestFramework::<Vesta, KIMCHI_COLS>::default()
        .gates(gates)
        .witness(witness)
        .public_inputs(public)
        .setup()
        .prove_and_verify::<BaseSponge, ScalarSponge>()
        .unwrap();
}

#[cfg(feature = "bn254")]
#[test]
fn test_generic_gate_pairing() {
    type Fp = ark_bn254::Fr;
    type SpongeParams = PlonkSpongeConstantsKimchi;
    type BaseSponge = DefaultFqSponge<ark_bn254::g1::Parameters, SpongeParams>;
    type ScalarSponge = DefaultFrSponge<Fp, SpongeParams>;

    use ark_ff::UniformRand;

    let public = vec![Fp::from(3u8); 5];
    let gates = create_circuit::<Fp, KIMCHI_COLS>(0, public.len());

    let rng = &mut rand::rngs::OsRng;
    let x = Fp::rand(rng);

    // create witness
    let mut witness: [Vec<Fp>; KIMCHI_COLS] = array::from_fn(|_| vec![Fp::zero(); gates.len()]);
    fill_in_witness(0, &mut witness, &public);

    // create and verify proof based on the witness
    <TestFramework<
        _,
        KIMCHI_COLS,
        poly_commitment::pairing_proof::PairingProof<ark_ec::bn::Bn<ark_bn254::Parameters>>,
    > as Default>::default()
    .gates(gates)
    .witness(witness)
    .public_inputs(public)
    .setup_with_custom_srs(|d1, usize| {
        let mut srs = poly_commitment::pairing_proof::PairingSRS::create(x, usize);
        srs.full_srs.add_lagrange_basis(d1);
        srs
    })
    .prove_and_verify::<BaseSponge, ScalarSponge>()
    .unwrap();
}
