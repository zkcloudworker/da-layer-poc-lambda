//! Keccak gadget
use crate::{
    auto_clone, auto_clone_array,
    circuits::{
        argument::{Argument, ArgumentEnv, ArgumentType},
        expr::{
            constraints::{boolean, ExprOps},
            Cache,
        },
        gate::GateType,
    },
};
use ark_ff::PrimeField;
use std::marker::PhantomData;

pub const DIM: usize = 5;
pub const QUARTERS: usize = 4;

#[macro_export]
macro_rules! state_from_vec {
    ($expr:expr) => {
        |i: usize, x: usize, y: usize, q: usize| {
            $expr[q + QUARTERS * (x + DIM * (y + DIM * i))].clone()
        }
    };
}

/// Creates the 5x5 table of rotation bits for Keccak modulo 64
/// | x \ y |  0 |  1 |  2 |  3 |  4 |
/// | ----- | -- | -- | -- | -- | -- |
/// | 0     |  0 | 36 |  3 | 41 | 18 |
/// | 1     |  1 | 44 | 10 | 45 |  2 |
/// | 2     | 62 |  6 | 43 | 15 | 61 |
/// | 3     | 28 | 55 | 25 | 21 | 56 |
/// | 4     | 27 | 20 | 39 |  8 | 14 |
/// Note that the order of the indexing is [y][x] to match the encoding of the witness algorithm
pub(crate) const OFF: [[u64; DIM]; DIM] = [
    [0, 1, 62, 28, 27],
    [36, 44, 6, 55, 20],
    [3, 10, 43, 25, 39],
    [41, 45, 15, 21, 8],
    [18, 2, 61, 56, 14],
];

pub const RC: [u64; 24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808a,
    0x8000000080008000,
    0x000000000000808b,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008a,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000a,
    0x000000008000808b,
    0x800000000000008b,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800a,
    0x800000008000000a,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

//~
//~ | `KeccakRound` | [0...440) | [440...1540) | [1540...2344) |
//~ | ------------- | --------- | ------------ | ------------- |
//~ | Curr          | theta     | pirho        | chi           |
//~
//~ | `KeccakRound` | [0...100) |
//~ | ------------- | --------- |
//~ | Next          | iota      |
//~
//~ -----------------------------------------------------------------------------------------------------------------------------------------------------------------------
//~
//~ | Columns  | [0...100) | [100...180) | [180...200) | [200...205) | [205...225)  | [225...245)  | [245...265)  |
//~ | -------- | --------- | ----------- | ----------- | ----------- | ------------ | ------------ | ------------ |
//~ | theta    | state_a   | shifts_c    | dense_c     | quotient_c  | remainder_c  | dense_rot_c  | expand_rot_c |
//~
//~ | Columns  | [265...665) | [665...765) | [765...865)  | [865...965) | [965...1065)  | [1065...1165) | [1165...1265) |
//~ | -------- | ----------- | ----------- | ------------ | ----------- | ------------- | ------------- | ------------- |
//~ | pirho    | shifts_e    | dense_e     | quotient_e   | remainder_e | bound_e       | dense_rot_e   | expand_rot_e  |
//~
//~ | Columns  | [1265...1665) | [1665...2065) |
//~ | -------- | ------------- | ------------- |
//~ | chi      | shifts_b      | shifts_sum    |
//~
//~ | Columns  | [0...4) | [4...100) |
//~ | -------- | ------- | --------- |
//~ | iota     | g00     | rest_g    |
//~
#[derive(Default)]
pub struct KeccakRound<F>(PhantomData<F>);

impl<F> Argument<F> for KeccakRound<F>
where
    F: PrimeField,
{
    const ARGUMENT_TYPE: ArgumentType = ArgumentType::Gate(GateType::KeccakRound);
    const CONSTRAINTS: u32 = 414;

    // Constraints for one round of the Keccak permutation function
    fn constraint_checks<T: ExprOps<F>>(env: &ArgumentEnv<F, T>, _cache: &mut Cache) -> Vec<T> {
        let mut constraints = vec![];

        // DEFINE ROUND CONSTANT
        let rc = [env.coeff(0), env.coeff(1), env.coeff(2), env.coeff(3)];

        // LOAD STATES FROM WITNESS LAYOUT
        // THETA
        let state_a = state_from_vec!(env.witness_curr_chunk(0, 100));
        let shifts_c = state_from_vec!(env.witness_curr_chunk(100, 180));
        let dense_c = state_from_vec!(env.witness_curr_chunk(180, 200));
        let quotient_c = env.witness_curr_chunk(200, 205);
        auto_clone_array!(quotient_c);
        let remainder_c = state_from_vec!(env.witness_curr_chunk(205, 225));
        let dense_rot_c = state_from_vec!(env.witness_curr_chunk(225, 245));
        let expand_rot_c = state_from_vec!(env.witness_curr_chunk(245, 265));
        // PI-RHO
        let shifts_e = state_from_vec!(env.witness_curr_chunk(265, 665));
        let dense_e = state_from_vec!(env.witness_curr_chunk(665, 765));
        let quotient_e = state_from_vec!(env.witness_curr_chunk(765, 865));
        let remainder_e = state_from_vec!(env.witness_curr_chunk(865, 965));
        let bound_e = state_from_vec!(env.witness_curr_chunk(965, 1065));
        let dense_rot_e = state_from_vec!(env.witness_curr_chunk(1065, 1165));
        let expand_rot_e = state_from_vec!(env.witness_curr_chunk(1165, 1265));
        // CHI
        let shifts_b = state_from_vec!(env.witness_curr_chunk(1265, 1665));
        let shifts_sum = state_from_vec!(env.witness_curr_chunk(1665, 2065));
        // IOTA
        let state_g = state_from_vec!(env.witness_next_chunk(0, 100));

        // Define vectors containing witness expressions which are not in the layout for efficiency
        let mut state_c: Vec<Vec<T>> = vec![vec![T::zero(); QUARTERS]; DIM];
        let mut state_d: Vec<Vec<T>> = vec![vec![T::zero(); QUARTERS]; DIM];
        let mut state_e: Vec<Vec<Vec<T>>> = vec![vec![vec![T::zero(); QUARTERS]; DIM]; DIM];
        let mut state_b: Vec<Vec<Vec<T>>> = vec![vec![vec![T::zero(); QUARTERS]; DIM]; DIM];
        let mut state_f: Vec<Vec<Vec<T>>> = vec![vec![vec![T::zero(); QUARTERS]; DIM]; DIM];

        // STEP theta: 5 * ( 3 + 4 * 1 ) = 35 constraints
        for x in 0..DIM {
            let word_c = compose_quarters(dense_c, x, 0);
            let rem_c = compose_quarters(remainder_c, x, 0);
            let rot_c = compose_quarters(dense_rot_c, x, 0);
            constraints
                .push(word_c * T::two_pow(1) - (quotient_c(x) * T::two_pow(64) + rem_c.clone()));
            constraints.push(rot_c - (quotient_c(x) + rem_c));
            constraints.push(boolean(&quotient_c(x)));

            for q in 0..QUARTERS {
                state_c[x][q] = state_a(0, x, 0, q)
                    + state_a(0, x, 1, q)
                    + state_a(0, x, 2, q)
                    + state_a(0, x, 3, q)
                    + state_a(0, x, 4, q);
                constraints.push(state_c[x][q].clone() - compose_shifts(shifts_c, x, 0, q));

                state_d[x][q] =
                    shifts_c(0, (x - 1 + DIM) % DIM, 0, q) + expand_rot_c(0, (x + 1) % DIM, 0, q);

                for y in 0..DIM {
                    state_e[y][x][q] = state_a(0, x, y, q) + state_d[x][q].clone();
                }
            }
        } // END theta

        // STEP pirho: 5 * 5 * (3 + 4 * 1) = 175 constraints
        for (y, col) in OFF.iter().enumerate() {
            for (x, off) in col.iter().enumerate() {
                let word_e = compose_quarters(dense_e, x, y);
                let quo_e = compose_quarters(quotient_e, x, y);
                let rem_e = compose_quarters(remainder_e, x, y);
                let bnd_e = compose_quarters(bound_e, x, y);
                let rot_e = compose_quarters(dense_rot_e, x, y);

                constraints.push(
                    word_e * T::two_pow(*off) - (quo_e.clone() * T::two_pow(64) + rem_e.clone()),
                );
                constraints.push(rot_e - (quo_e.clone() + rem_e));
                constraints.push(bnd_e - (quo_e + T::two_pow(64) - T::two_pow(*off)));

                for q in 0..QUARTERS {
                    constraints.push(state_e[y][x][q].clone() - compose_shifts(shifts_e, x, y, q));
                    state_b[(2 * x + 3 * y) % DIM][y][q] = expand_rot_e(0, x, y, q);
                }
            }
        } // END pirho

        // STEP chi: 4 * 5 * 5 * 2 = 200 constraints
        for q in 0..QUARTERS {
            for x in 0..DIM {
                for y in 0..DIM {
                    let not =
                        T::literal(F::from(0x1111111111111111u64)) - shifts_b(0, (x + 1) % 5, y, q);
                    let sum = not + shifts_b(1, (x + 2) % 5, y, q);
                    let and = shifts_sum(1, x, y, q);
                    constraints.push(state_b[y][x][q].clone() - compose_shifts(shifts_b, x, y, q));
                    constraints.push(sum - compose_shifts(shifts_sum, x, y, q));
                    state_f[y][x][q] = shifts_b(0, x, y, q) + and;
                }
            }
        } // END chi

        // STEP iota: 4 constraints
        for (q, c) in rc.iter().enumerate() {
            constraints.push(state_g(0, 0, 0, q) - (state_f[0][0][q].clone() + c.clone()));
        } // END iota

        constraints
    }
}

//~
//~ | `KeccakSponge` | [0...100) | [100...168) | [168...200) | [200...300) | [300...500] | [500...900) |
//~ | -------------- | --------- | ----------- | ----------- | ----------- | ----------- | ----------- |
//~ | Curr           | old_state | new_block   | zeros       | dense       | bytes       | shifts      |
//~ | Next           | xor_state |
//~
#[derive(Default)]
pub struct KeccakSponge<F>(PhantomData<F>);

impl<F> Argument<F> for KeccakSponge<F>
where
    F: PrimeField,
{
    const ARGUMENT_TYPE: ArgumentType = ArgumentType::Gate(GateType::KeccakSponge);
    const CONSTRAINTS: u32 = 448;

    // Constraints for one round of the Keccak permutation function
    fn constraint_checks<T: ExprOps<F>>(env: &ArgumentEnv<F, T>, _cache: &mut Cache) -> Vec<T> {
        let mut constraints = vec![];

        // LOAD WITNESS
        let old_state = env.witness_curr_chunk(0, 100);
        let mut new_block = env.witness_curr_chunk(100, 168);
        let mut zeros = env.witness_curr_chunk(168, 200);
        new_block.append(&mut zeros);
        let xor_state = env.witness_next_chunk(0, 100);
        let dense = env.witness_curr_chunk(200, 300);
        let bytes = env.witness_curr_chunk(300, 500);
        let shifts = env.witness_curr_chunk(500, 900);
        auto_clone_array!(old_state);
        auto_clone_array!(new_block);
        auto_clone_array!(xor_state);
        auto_clone_array!(dense);
        auto_clone_array!(bytes);
        auto_clone_array!(shifts);

        // LOAD COEFFICIENTS
        let root = env.coeff(0);
        let absorb = env.coeff(1);
        let squeeze = env.coeff(2);
        auto_clone!(root);
        auto_clone!(absorb);
        auto_clone!(squeeze);

        // STEP absorb: 32 + 100 * 4 = 432
        for z in zeros {
            // Absorb phase pads with zeros the new state
            constraints.push(absorb() * z);
        }
        for i in 0..QUARTERS * DIM * DIM {
            // In first absorb, root state is all zeros
            constraints.push(root() * old_state(i));
            // Absorbs the new block by performing XOR with the old state
            constraints.push(absorb() * (xor_state(i) - (old_state(i) + new_block(i))));
            // Check shifts correspond to the decomposition of the new state
            constraints.push(absorb() * (new_block(i) - compose_shifts_from_vec(shifts, i)));
            // Both phases: check correctness of each dense term (16 bits) by composing two bytes
            // TODO: maybe this can be passed to the lookup table composing values from two cells
            constraints.push(dense(i) - (bytes(2 * i) + T::two_pow(8) * bytes(2 * i + 1)));
        }

        // STEP squeeze: 16 constraints
        for i in 0..16 {
            // Check shifts correspond to the 256-bit prefix digest of the old state (current)
            constraints.push(squeeze() * (old_state(i) - compose_shifts_from_vec(shifts, i)));
        }

        constraints
    }
}

fn compose_quarters<F: PrimeField, T: ExprOps<F>>(
    quarters: impl Fn(usize, usize, usize, usize) -> T,
    x: usize,
    y: usize,
) -> T {
    quarters(0, x, y, 0)
        + T::two_pow(16) * quarters(0, x, y, 1)
        + T::two_pow(32) * quarters(0, x, y, 2)
        + T::two_pow(48) * quarters(0, x, y, 3)
}

fn compose_shifts<F: PrimeField, T: ExprOps<F>>(
    shifts: impl Fn(usize, usize, usize, usize) -> T,
    x: usize,
    y: usize,
    q: usize,
) -> T {
    shifts(0, x, y, q)
        + T::two_pow(1) * shifts(1, x, y, q)
        + T::two_pow(2) * shifts(2, x, y, q)
        + T::two_pow(3) * shifts(3, x, y, q)
}

fn compose_shifts_from_vec<F: PrimeField, T: ExprOps<F>>(
    shifts: impl Fn(usize) -> T,
    i: usize,
) -> T {
    shifts(4 * i)
        + T::two_pow(1) * shifts(4 * i + 1)
        + T::two_pow(2) * shifts(4 * i + 2)
        + T::two_pow(3) * shifts(4 * i + 3)
}
