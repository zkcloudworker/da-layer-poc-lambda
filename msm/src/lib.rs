use mina_curves::pasta::curves::pallas;
use mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    sponge::{DefaultFqSponge, DefaultFrSponge},
};
use poly_commitment::pairing_proof::PairingProof;

pub mod column;
pub mod constraint;
pub mod mvlookup;
pub mod precomputed_srs;
pub mod proof;
pub mod prover;
pub mod verifier;

pub const NUM_LIMBS: usize = 16;
pub const DOMAIN_SIZE: usize = 1 << 15;

// M in the paper of MVLookup
pub const NUM_LOOKUP_M: usize = 8;

pub type MsmBN254 = ark_ec::bn::Bn<ark_bn254::Parameters>;
pub type MsmBN254G1Affine = <MsmBN254 as ark_ec::PairingEngine>::G1Affine;
pub type MsmBN254G2Affine = <MsmBN254 as ark_ec::PairingEngine>::G2Affine;

// THe native field we are working with
pub type Fp = ark_bn254::Fr;
// The foreign field we are emulating (one of the two)
pub type Ff1 = mina_curves::pasta::Fp;
pub type Ff2 = mina_curves::pasta::Fq;

pub type SpongeParams = PlonkSpongeConstantsKimchi;
pub type BaseSponge = DefaultFqSponge<ark_bn254::g1::Parameters, SpongeParams>;
pub type ScalarSponge = DefaultFrSponge<Fp, SpongeParams>;
pub type OpeningProof = PairingProof<MsmBN254>;
