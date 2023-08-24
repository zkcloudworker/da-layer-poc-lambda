use std::array;

use crate::circuits::{
    constraints::ConstraintSystem,
    gate::CircuitGate,
    polynomials::keccak::{self, ROT_TAB, STATE},
    wires::Wire,
};
use ark_ec::AffineCurve;
use mina_curves::pasta::{Fp, Pallas, Vesta};
use num_bigint::BigUint;
use rand::{rngs::StdRng, Rng, SeedableRng};

//use super::framework::TestFramework;
type PallasField = <Pallas as AffineCurve>::BaseField;

const RNG_SEED: [u8; 32] = [
    0, 131, 43, 175, 229, 252, 206, 26, 67, 193, 86, 160, 1, 90, 131, 86, 168, 4, 95, 50, 48, 9,
    192, 13, 250, 215, 172, 130, 24, 164, 162, 221,
];

fn create_test_constraint_system() -> ConstraintSystem<Fp> {
    let (mut next_row, mut gates) = { CircuitGate::<Fp>::create_keccak(0) };

    // Temporary workaround for lookup-table/domain-size issue
    for _ in 0..(1 << 13) {
        gates.push(CircuitGate::zero(Wire::for_row(next_row)));
        next_row += 1;
    }

    ConstraintSystem::create(gates).build().unwrap()
}

#[test]
// Test that all of the offsets in the rotation table work fine
fn test_keccak_table() {
    let cs = create_test_constraint_system();
    let state = array::from_fn(|_| {
        array::from_fn(|_| rand::thread_rng().gen_range(0..2u128.pow(64)) as u64)
    });
    let witness = keccak::create_witness_keccak_rot(state);
    for row in 0..=48 {
        assert_eq!(
            cs.gates[row].verify_witness::<Vesta>(
                row,
                &witness,
                &cs,
                &witness[0][0..cs.public].to_vec()
            ),
            Ok(())
        );
    }
    let mut rot = 0;
    for (x, row) in ROT_TAB.iter().enumerate() {
        for (y, &bits) in row.iter().enumerate() {
            if bits == 0 {
                continue;
            }
            assert_eq!(
                PallasField::from(state[x][y].rotate_left(bits)),
                witness[1][1 + 2 * rot],
            );
            rot += 1;
        }
    }
}

#[test]
// Test if converters work fine
fn test_from() {
    let rng = &mut StdRng::from_seed(RNG_SEED);
    let bytes = (0..STATE / 8)
        .map(|_| rng.gen_range(0..=255))
        .collect::<Vec<u8>>();
    let converted = keccak::from_state_to_bytes(keccak::from_bytes_to_state(&bytes));
    assert_eq!(bytes, converted);
}

#[test]
// Check that the padding is added correctly
fn test_padding() {
    let message = 0x30; // Character "0"
    let bytes = [
        0x30, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80,
    ];
    // 0x30 0x01 0x00 ... 0x00 0x80
    let padded = keccak::pad101(&[message], 1088);
    let number = BigUint::from_bytes_be(&padded);
    let desired = BigUint::from_bytes_be(&bytes);
    assert_eq!(number, desired)
}

#[test]
// Check the steps of the hash
fn test_f() {
    let message = [0x33u8, 0x0A, 0x84, 0x2A, 0xEE, 0x90, 0x73, 0xFD, 0x85, 0x21]; // Character "0"
    let hash = keccak::keccak_eth(&message);
    let desired = [
        0x36, 0x3d, 0xf8, 0xb0, 0x53, 0x90, 0x94, 0xb1, 0x3d, 0x45, 0x6a, 0x5c, 0xb4, 0xc0, 0xe2,
        0x18, 0x05, 0xc6, 0x3c, 0xc5, 0x78, 0x13, 0x06, 0x57, 0xc7, 0x61, 0x6d, 0xf5, 0xc8, 0x5a,
        0x8d, 0x52,
    ];
    assert_eq!(hash, desired);
}

#[test]
fn test_2047() {
    let message = [
        0x1Fu8, 0x42u8, 0xAD, 0xD2, 0x5C, 0x0A, 0x80, 0xA4, 0xC8, 0x2A, 0xAE, 0x3A, 0x0E, 0x30,
        0x2A, 0xBF, 0x92, 0x61, 0xDC, 0xA7, 0xE7, 0x88, 0x4F, 0xD8, 0x69, 0xD9, 0x6E, 0xD4, 0xCE,
        0x88, 0xAA, 0xAA, 0x25, 0x30, 0x4D, 0x2D, 0x79, 0xE1, 0xFA, 0x5C, 0xC1, 0xFA, 0x2C, 0x95,
        0x89, 0x92, 0x29, 0xBC, 0x87, 0x43, 0x1A, 0xD0, 0x6D, 0xA5, 0x24, 0xF2, 0x14, 0x0E, 0x70,
        0xBD, 0x05, 0x36, 0xE9, 0x68, 0x5E, 0xE7, 0x80, 0x8F, 0x59, 0x8D, 0x8A, 0x9F, 0xE1, 0x5D,
        0x40, 0xA7, 0x2A, 0xEF, 0xF4, 0x31, 0x23, 0x92, 0x92, 0xC5, 0xF6, 0x4B, 0xDB, 0x7F, 0x62,
        0x0E, 0x5D, 0x16, 0x0B, 0x32, 0x9D, 0xEB, 0x58, 0xCF, 0x6D, 0x5C, 0x06, 0x65, 0xA3, 0xDE,
        0xD6, 0x1A, 0xE4, 0xAD, 0xBC, 0xA9, 0x4D, 0xC2, 0xB7, 0xB0, 0x2C, 0xDF, 0x39, 0x92, 0xFD,
        0xF7, 0x9B, 0x3D, 0x93, 0xE5, 0x46, 0xD5, 0x82, 0x3C, 0x3A, 0x63, 0x09, 0x23, 0x06, 0x4E,
        0xD2, 0x4C, 0x3D, 0x97, 0x4C, 0x46, 0x02, 0xA4, 0x9D, 0xF7, 0x5E, 0x49, 0xCF, 0x7B, 0xD5,
        0x1E, 0xDC, 0x73, 0x82, 0x21, 0x4C, 0xBA, 0x85, 0x0C, 0x4D, 0x3D, 0x11, 0xB4, 0x0A, 0x70,
        0xB1, 0xD9, 0x26, 0xE3, 0x75, 0x5E, 0xC7, 0x96, 0x93, 0x62, 0x0C, 0x24, 0x2A, 0xB0, 0xF2,
        0x3E, 0xA2, 0x06, 0xBA, 0x33, 0x7A, 0x7E, 0xDC, 0x54, 0x21, 0xD6, 0x31, 0x26, 0xCB, 0x6C,
        0x70, 0x94, 0xF6, 0xBC, 0x1C, 0xF9, 0x94, 0x37, 0x96, 0xBE, 0x2A, 0x0D, 0x9E, 0xB7, 0x4F,
        0xC7, 0x26, 0xAA, 0x0C, 0x0D, 0x3B, 0x3D, 0x39, 0x03, 0x9D, 0xEA, 0xD3, 0x9A, 0x71, 0x69,
        0xF8, 0xC3, 0xE2, 0x36, 0x5D, 0xD3, 0x49, 0xE3, 0x58, 0xBF, 0x08, 0xC7, 0x17, 0xD2, 0xE4,
        0x36, 0xD6, 0x51, 0x72, 0xA7, 0x6E, 0xD5, 0xE1, 0xF1, 0xE6, 0x94, 0xA7, 0x5C, 0x19, 0x28,
        0x0B, 0x2A,
    ];
    let hash = keccak::keccak_eth(&message);
    let desired = [
        0x00u8, 0x87, 0x46, 0xe3, 0x74, 0xd8, 0x5d, 0xf8, 0x72, 0x74, 0xdb, 0xff, 0x81, 0xb6, 0x2b,
        0x62, 0xe4, 0xe5, 0x35, 0x5e, 0x3e, 0x80, 0x5e, 0xe0, 0xcd, 0xce, 0xdc, 0xfd, 0x8f, 0x3d,
        0xd2, 0xf7,
    ];
    assert_eq!(hash, desired);
}

#[test]
// Check the steps of NIST variant
fn test_sha() {
    let message = [0x33u8, 0x0A, 0x84, 0x2A, 0xEE, 0x90, 0x73, 0xFD, 0x85, 0x21]; // Character "0"
    let hash224 = keccak::sha3_224(&message);
    let hash256 = keccak::sha3_256(&message);
    let hash384 = keccak::sha3_384(&message);
    let hash512 = keccak::sha3_512(&message);
    let desired224 = [
        0xd2, 0x8a, 0xd0, 0xcb, 0xa3, 0x61, 0xb2, 0xcb, 0x7d, 0x47, 0xb7, 0xe0, 0xc6, 0x57, 0xf3,
        0x29, 0x7d, 0xb4, 0x73, 0x61, 0x19, 0x67, 0xbf, 0x1a, 0x79, 0xc0, 0xff, 0x9b,
    ];
    let desired256 = [
        0x19, 0x02, 0x52, 0x1f, 0x53, 0x97, 0x4c, 0x47, 0x8e, 0x9f, 0x80, 0xf1, 0xe2, 0x87, 0xce,
        0x95, 0xa3, 0xfa, 0x40, 0x76, 0x2e, 0xd1, 0x71, 0x0e, 0x4b, 0xb8, 0x44, 0x88, 0xb0, 0x46,
        0xb4, 0xa8,
    ];
    let desired384 = [
        0x33, 0xb9, 0x2d, 0xbb, 0x72, 0x36, 0x68, 0x6e, 0xca, 0x21, 0x1a, 0xec, 0xd4, 0xf9, 0x1a,
        0xd0, 0xe0, 0x4e, 0xf0, 0x13, 0xe8, 0x57, 0x07, 0x49, 0x0c, 0xb1, 0x58, 0x09, 0x54, 0x46,
        0x20, 0xe4, 0xde, 0xad, 0x50, 0x38, 0xdb, 0x59, 0x39, 0xa3, 0x1a, 0xf7, 0x61, 0x06, 0xfd,
        0x06, 0x4d, 0x84,
    ];
    let desired512 = [
        0xc1, 0x92, 0x7e, 0xd7, 0x47, 0xba, 0xfd, 0x09, 0xb3, 0xc0, 0x6d, 0xfb, 0x16, 0x47, 0x77,
        0xbf, 0xc0, 0x68, 0x1b, 0x04, 0x7e, 0xb7, 0xbf, 0x7f, 0x58, 0x72, 0xd2, 0xe1, 0xe0, 0xf6,
        0x43, 0xd4, 0x92, 0x57, 0x62, 0x89, 0x35, 0xcd, 0x4e, 0x2e, 0xdd, 0xa9, 0x91, 0x57, 0x95,
        0xe1, 0x64, 0x04, 0xab, 0x10, 0xa3, 0x79, 0x49, 0x2d, 0xd7, 0x26, 0xce, 0x36, 0xb4, 0x32,
        0x44, 0x7d, 0xf5, 0x14,
    ];
    assert_eq!(hash224, desired224);
    assert_eq!(hash256, desired256);
    assert_eq!(hash384, desired384);
    assert_eq!(hash512, desired512);
}
#[test]
// Check the steps of the hash
fn test_keccak64() {
    let message = [
        0xf8u8, 0x67, 0x80, 0x86, 0x2d, 0x79, 0x88, 0x3d, 0x20, 0x00, 0x82, 0x52, 0x08, 0x94, 0x5d,
        0xf9, 0xb8, 0x79, 0x91, 0x26, 0x2f, 0x6b, 0xa4, 0x71, 0xf0, 0x97, 0x58, 0xcd, 0xe1, 0xc0,
        0xfc, 0x1d, 0xe7, 0x34, 0x82, 0x7a, 0x69, 0x80, 0x1c, 0xa0, 0x88, 0xff, 0x6c, 0xf0, 0xfe,
        0xfd, 0x94, 0xdb, 0x46, 0x11, 0x11, 0x49, 0xae, 0x4b, 0xfc, 0x17, 0x9e, 0x9b, 0x94, 0x72,
        0x1f, 0xff, 0xd8, 0x21, 0xd3, 0x8d, 0x16, 0x46, 0x4b, 0x3f, 0x71, 0xd0, 0xa0, 0x45, 0xe0,
        0xaf, 0xf8, 0x00, 0x96, 0x1c, 0xfc, 0xe8, 0x05, 0xda, 0xef, 0x70, 0x16, 0xb9, 0xb6, 0x75,
        0xc1, 0x37, 0xa6, 0xa4, 0x1a, 0x54, 0x8f, 0x7b, 0x60, 0xa3, 0x48, 0x4c, 0x06, 0xa3, 0x3a,
    ];
    let hash = keccak::keccak_eth(&message);
    let desired = [
        0x5c, 0x50, 0x4e, 0xd4, 0x32, 0xcb, 0x51, 0x13, 0x8b, 0xcf, 0x09, 0xaa, 0x5e, 0x8a, 0x41,
        0x0d, 0xd4, 0xa1, 0xe2, 0x04, 0xef, 0x84, 0xbf, 0xed, 0x1b, 0xe1, 0x6d, 0xfb, 0xa1, 0xb2,
        0x20, 0x60,
    ];
    assert_eq!(hash, desired);

    let message = "02f9019c018201338405f5e100850cad3895d8830108949440a50cf069e992aa4536211b23f286ef88752187880b1a2bc2ec500000b90124322bba210000000000000000000000008a001303158670e284950565164933372807cd4800000000000000000000000012d220fbda92a9c8f281ea02871afa70dfde81e90000000000000000000000000000000000000000000000000afd4ea3d29472400000000000000000000000000000000000000000461c9bb5bb1c3429b25544e3f4b7bb67d63f9b432df61df28a9897e26284b370adcd7b558fa286babb0efdeb000000000000000000000000000000000000000000000000001cdd1f19bb8dc0000000000000000000000000000000000000000000000000000000006475ed380000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a8f2573c080a0893bc3facf19becba979e31d37ed1b222faab09b8c554a17072f6fbfc1e5658fa01119ef751f0fc3c1ec4d1eeb9db64c9f416ce1aa3267d7b98d8426ab35f0c422";
    let message = hex::decode(message).expect("Decoding failed");
    let hash = keccak::keccak_eth(&message);
    let desired = "9cec14aadb06b59b2646333f47efe0ee7f21fed48d93806023b8eb205aa3b161";
    let desired = hex::decode(desired).expect("Decoding failed");
    assert_eq!(hash, desired);
}
