//! Keccak gadget
use crate::circuits::{
    argument::{Argument, ArgumentEnv, ArgumentType},
    expr::{constraints::ExprOps, Cache},
    gate::GateType,
};
use ark_ff::PrimeField;
use std::marker::PhantomData;

pub const DIM: usize = 5;
pub const QUARTERS: usize = 4;

#[macro_export]
macro_rules! state_from_layout {
    ($var:ident, $expr:expr) => {
        let $var = $expr;
        let $var = |i: usize, x: usize, y: usize, q: usize| {
            $var[q + QUARTERS * (x + DIM * (y + DIM * i))].clone()
        };
    };
    ($var:ident) => {
        let $var = |i: usize, x: usize, y: usize, q: usize| {
            $var[q + QUARTERS * (x + DIM * (y + DIM * i))].clone()
        };
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
pub const ROT_TAB: [[u32; 5]; 5] = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14],
];

//~
//~ | Columns  | [0...440) | [440...1540) | [1540...2440) | 2440 |
//~ | -------- | --------- | ------------ | ------------- | ---- |
//~ | `Keccak` | theta     | pirho        | chi           | iota |
//~
//~ | Columns  | [0...100) | [100...120) | [120...200) | [200...220) | [220...240) | [240...260)  | [260...280) | [280...300)  | 300...320)   | [320...340) | [340...440) |
//~ | -------- | --------- | ----------- | ----------- | ----------- | ----------- | ------------ | ----------- | ------------ | ------------ | ----------- | ----------- |
//~ | theta    | state_a   | state_c     | reset_c     | dense_c     | quotient_c  | remainder_c  | bound_c     | dense_rot_c  | expand_rot_c | state_d     | state_e     |
//~
//~ | Columns  | [440...840) | [840...940) | [940...1040) | [1040...1140) | [1140...1240) | [1240...1340) | [1440...1540) |
//~ | -------- | ----------- | ----------- | ------------ | ------------- | ------------- | ------------- | ------------- |
//~ | pirho    | reset_e     | dense_e     | quotient_e   | remainder_e   | bound_e       | dense_rot_e   | expand_rot_e  |
//~
//~ | Columns  | [1540...1940) | [1940...2340) | [2340...2440) |
//~ | -------- | ------------- | ------------- | ------------- |
//~ | chi      | reset_b       | reset_sum     | state_f       |
//~
//~ | Columns  | 2440 |
//~ | -------- | ---- |
//~ | iota     | g00  |
//~
#[derive(Default)]
pub struct Keccak<F>(PhantomData<F>);

impl<F> Argument<F> for Keccak<F>
where
    F: PrimeField,
{
    const ARGUMENT_TYPE: ArgumentType = ArgumentType::Gate(GateType::Keccak);
    const CONSTRAINTS: u32 = 20 + 55 + 100 + 125 + 200 + 4;

    // Constraints for one round of the Keccak permutation function
    fn constraint_checks<T: ExprOps<F>>(env: &ArgumentEnv<F, T>, _cache: &mut Cache) -> Vec<T> {
        let mut constraints = vec![];

        // LOAD WITNESS LAYOUT
        // THETA
        let state_a = env.witness_curr_chunk(0, 100);
        let state_c = env.witness_curr_chunk(100, 120);
        let reset_c = env.witness_curr_chunk(120, 200);
        let dense_c = env.witness_curr_chunk(200, 220);
        let quotient_c = env.witness_curr_chunk(220, 240);
        let remainder_c = env.witness_curr_chunk(240, 260);
        let bound_c = env.witness_curr_chunk(260, 280);
        let dense_rot_c = env.witness_curr_chunk(280, 300);
        let expand_rot_c = env.witness_curr_chunk(300, 320);
        let state_d = env.witness_curr_chunk(320, 340);
        let state_e = env.witness_curr_chunk(340, 440);
        // PI-RHO
        let reset_e = env.witness_curr_chunk(440, 840);
        let dense_e = env.witness_curr_chunk(840, 940);
        let quotient_e = env.witness_curr_chunk(940, 1040);
        let remainder_e = env.witness_curr_chunk(1040, 1140);
        let bound_e = env.witness_curr_chunk(1140, 1240);
        let dense_rot_e = env.witness_curr_chunk(1240, 1340);
        let expand_rot_e = env.witness_curr_chunk(1340, 1440);
        let state_b = env.witness_curr_chunk(1440, 1540);
        // CHI
        let reset_b = env.witness_curr_chunk(1540, 1940);
        let reset_sum = env.witness_curr_chunk(1940, 2340);
        let state_f = env.witness_curr_chunk(2340, 2440);
        // IOTA
        let g00 = env.witness_curr_chunk(2440, 2444);

        // LOAD STATES FROM LAYOUT
        state_from_layout!(state_a);
        state_from_layout!(state_c);
        state_from_layout!(reset_c);
        state_from_layout!(dense_c);
        state_from_layout!(quotient_c);
        state_from_layout!(remainder_c);
        state_from_layout!(bound_c);
        state_from_layout!(dense_rot_c);
        state_from_layout!(expand_rot_c);
        state_from_layout!(state_d);
        state_from_layout!(state_e);
        state_from_layout!(reset_e);
        state_from_layout!(dense_e);
        state_from_layout!(quotient_e);
        state_from_layout!(remainder_e);
        state_from_layout!(bound_e);
        state_from_layout!(dense_rot_e);
        state_from_layout!(expand_rot_e);
        state_from_layout!(state_b);
        state_from_layout!(reset_b);
        state_from_layout!(reset_sum);
        state_from_layout!(state_f);
        state_from_layout!(g00);

        // STEP theta: 5 * ( 3 + 4 * (3 + 5 * 1) ) = 175 constraints
        for x in 0..DIM {
            let word_c = compose_quarters(dense_c, x, 0);
            let quo_c = compose_quarters(quotient_c, x, 0);
            let rem_c = compose_quarters(remainder_c, x, 0);
            let bnd_c = compose_quarters(bound_c, x, 0);
            let rot_c = compose_quarters(dense_rot_c, x, 0);

            constraints
                .push(word_c * T::two_pow(1) - (quo_c.clone() * T::two_pow(64) + rem_c.clone()));
            constraints.push(rot_c - (quo_c.clone() + rem_c));
            constraints.push(bnd_c - (quo_c + T::two_pow(64) - T::two_pow(1)));

            for q in 0..QUARTERS {
                constraints.push(
                    state_c(0, x, 0, q)
                        - (state_a(0, x, 0, q)
                            + state_a(0, x, 1, q)
                            + state_a(0, x, 2, q)
                            + state_a(0, x, 3, q)
                            + state_a(0, x, 4, q)),
                );
                constraints.push(
                    state_c(0, x, 0, q)
                        - (reset_c(0, x, 0, q)
                            + T::two_pow(1) * reset_c(1, x, 0, q)
                            + T::two_pow(2) * reset_c(2, x, 0, q)
                            + T::two_pow(3) * reset_c(3, x, 0, q)),
                );
                constraints.push(
                    state_d(0, x, 0, q)
                        - (reset_c(0, (x - 1 + DIM) % DIM, 0, q)
                            + expand_rot_c(0, (x + 1) % DIM, 0, q)),
                );

                for y in 0..DIM {
                    constraints
                        .push(state_e(0, x, y, q) - (state_a(0, x, y, q) + state_d(0, x, 0, q)));
                }
            }
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
