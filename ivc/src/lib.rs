//! This crate provides a circuit to achieve Incremental Verifiable
//! Computation (IVC) based on a variant of the folding scheme described in the
//! paper [Nova](https://eprint.iacr.org/2021/370.pdf). For the rest of the
//! document, we do suppose that the curve is BN254.
//!
//! The circuit is implemented using the generic interpreter provided by the
//! crate [kimchi_msm].
//! The particularity of the implementation is that it doesn't use a cycle of
//! curves, aims to be as generic as possible, defer the scalar
//! multiplications to compute them in bulk and rely on the Poseidon hash.
//! Our argument will be mostly based on the state of the hash after we
//! executed the whole computation.
//!
//! The IVC circuit is divided into different sections/sets of constraints
//! described by multivariate polynomials, each
//! activated by (public) selectors that are defined at setup time.
//! The number of columns of the original computation defines the shape of the
//! circuit as it will define the values of the selectors.
//!
//! First, the folding scheme we use is described in the crate
//! [folding](folding::expressions) and is based on degree-3 polynomials.
//! We do suppose that each constraint are reduced to degree 2, and the third
//! degree is used to encode the aggregation of constraints.
//!
//! In the [Nova paper](https://eprint.iacr.org/2021/370.pdf), to provide
//! incremental verifiable computation, the authors propose a folding scheme
//! where the verifier has to compute the followings:
//! ```text
//! // Accumulation of the homogeneous value `u`:
//! u'' = u + r u'
//! // Accumulation of all the PIOP challenges:
//! for each challenge c_i:
//!    c_i'' = c_i + r c_i'
//! for each alpha_i (aggregation of constraints):
//!    alpha_i'' = alpha_i + r alpha_i'
//! // Accumulation of the blinders for the commitment:
//! blinder'' = blinder + r + r^2 + r^3 blinder'
//! // Accumulation of the error terms (scalar multiplication)
//! E = E1 + r T0 + r^2 T1 + r^3 E2
//! // Randomized accumulation of the instance commitments (scalar multiplication)
//! for i in 0..N_COLUMNS
//!    (C_i)_O = (C_i)_L + r (C_i)_R
//! ```
//!
//! The accumulation of the challenges, the homogeneous value and the blinders
//! are done trivially as they are scalar field values.
//! After that, the verifier has to perform foreign field ellictic curve
//! additions and scalar multiplications `(r T0)`, `(r^2 T1)`, `(r^3 E2)` and
//! `(r (C_i)_R)` for each column.
//!
//! First, we decide to defer the computations of
//! the scalar multiplications, and we reduce the verifier work to compute only
//! the foreign field elliptic curve additions. Therefore, the verifier has
//! access already to the
//! result of `r T0`, `r^2 T1`, `r^3 E2` and `r (C_i)_R`, and must only perform the
//! addition. We call the commitments `r T0` the "scaled" commitment to `T0`, and
//! the same for the others. The commitments `(C_i)_L`, `(C_i)_R` and `(C_i)_O` are
//! called the "instance commitments".
//!
//! To perform foreign field elliptic curve addition, we split the commitments
//! into 17 chunks of 15 bits and use additive lookups as described in
//! [kimchi_msm::logup]. These 17 chunks will be used later to compute the
//! scalar multiplications using a variant of the scheme described in [the MSM
//! RFC](https://github.com/o1-labs/rfcs/blob/main/0013-efficient-msms-for-non-native-pickles-verification.md)
//!
//! The first component of our circuit is a hash function.
//! We decided to use the Poseidon hash function, and implemented a generic one
//! using the generic interpreter in [crate::poseidon_8_56_5_3_2]. The Poseidon
//! instance we decided to go with is the traditional full/partial rounds. For a
//! security of 128 bits, a substitution box of 5 and a state of 3 elements, we
//! need 8 full rounds and 56 partial rounds for the scalar field BN254.
//! Therefore, we can "absorb" 2 field elements per row (the third element is
//! kept as a buffer in the Sponge construction).
//!
//! The cost of a single Poseidon hash in terms of constraints is 432
//! constraints, and requires 435 columns, in addition to 192 round constants
//! considered as constants "selectors" in our constraints.
//! Note that having 432 constraints per Poseidon hash means that we must also
//! have 432 constraints to accumulate the challenges used to combine the
//! constraints. It is worth noting that these alphas are the same for all
//! poseidon hashes.
//! Combining constraints is done by using another selector, on a single row,
//! see below (TODO).
//! Note that the columns used by the poseidon hash must also be aggregated
//! while running the IVC.
//!
//! The Poseidon hash is used to absorb all the instance commitments, the
//! challenges and the scaled commitments.
//! In our circuit, we first start by absorbing all the elliptic curve points. A
//! particularity is that we will use different Poseidon instances for each
//! "side" of the addition. For each point, we do assume (*at the moment*) that
//! each point is encoded as 2 field elements in the field of the circuit
//! (FIXME: there is negligeable probability that it wraps over, as the
//! coordinates are in the base field).
// We will change this soon, by absorbing the coordinate x, and the sign of y.
// If we compute on one row the hash and on the next row the ECADD, the sign of
// y can be computed on the ECADD row, and accessed by the poseidon constraints.
//! For a given set of coordinates `(x, y)`, and by supposing an initial state of
//! our permutation `(s0, s1, s2)`, we will compute on a single row the absorbtion
//! of `(x, y)`, which consists of updating the state `(s0, s1, s2)` to
//! `(s0 + x, s1 + y, s2)`.
//! For each side, we will initialize a new Poseidon state, and we will keep
//! absorbing each column of the circuit.
//!
//! We end up with the following shape:
//! ```text
//! | q_poseidon | s0 | s1 | s2 | s0 + x | s1 + y | ... | s0' | s1' | s2' | side |
//! ```
//! where `(s0', s1', s2')` is the final state of the Poseidon permutation after
//! the execution of all the rounds, and `q_poseidon` will be a (public
//! selector) that will be set to `1` on the row that Poseidon will need to be
//! executed, `0` otherwise.
//!
//! ## Parallel folding/bifolding
//!
//! The library also lets the user fold in parallel.
//! Imagine you have two polynomial P_1(X1, ..., Xn) and P_2(Y1, ..., Yn) that
//! you define two different relations R_1 and R_2.
//! As usual, you can create a unique relation R that contains two additional
//! columns q_1 and q_2 consisting of "selecting" the actual relation to use on one row.
//! In addition to the columns, you will add constraints that will ensure that
//! only one of the two relations is used, as follow:
//! ```text
//! q_1 * (1 - q_1) = 0
//! q_2 * (1 - q_2) = 0
//! q_1 + q_2 = 1
//! ```
//!
//! The IVC paradigm defined in Nova lets you fold incrementally sequential
//! computation, by
//! that we mean that if you have a function f, you can incrementally, and
//! sequentially verify `f(f(f(...(f(x)))))` by encoding in a
//! single circuit the verifier of the previous step. In the case of Nova, it is
//! supposed that the goal is to prove the composition of f with itself, i.e.
//! the input of the step (i + 1) depends on the output of the step i.
//!
//! Imagine the case of a function that doesn't depend on the previous output, and
//! therefore you don't want to prove a new execution of f on the input `f(x)`,
//! but simply that at each step, you correctly executed the function f of
//! another previous step. For instance, it is the case of a zero-knowledge
//! proof virtual machine like `o1vm` because the "time" is encoded as a column,
//! and proved by a (folded) lookup argument.
//!
//! The issue we have with Nova is that the input of the function `F'` is a
//! relaxed instance (which is the circuit computing f at the previous + the
//! input of the previous step) and you have also the current step.
//! The Nova circuit will output a new relaxed instance, and does not allow to
//! "merge" two independent computations.
//! The Nova paper doesn't allow to "reduce" the number of instances that you
//! will input into the circuit. You can never go from 4 inputs to 2 inputs for
//! instance.
//!
//! We propose a solution to compress the number of input instances in a single
//! instance.
//!
//! (dw)
//! The main idea behind the parallel folding is to keep the accumulation of the
//! application circuit and the IVC circuit separately.
//! In the Nova paper, the accumulator is noted with the capital letter `U`, and
//! the current value to be folded into `U_i` is noted with a lowercase `u_i`.
//! In other words, `U_i` defines the accumulated instance of the function `F'` (i.e.
//! the function performing the IVC + the original application F).
//!
//! For the sake of simplicity, and to avoid confusion in the notation, we will
//! start using `acc_i` for the accumulator up to step `i`, and `u_i` for the
//! current instance.
//!
//! In Nova, the function `F'` is the concatenation of the application circuit
//! and the IVC circuit. In our design, we will keep the application circuit and
//! the IVC circuit separated.
//! Therefore, we will call:
//! ```text
//! U^{APP} = the accumulator of the application circuit.
//! U^{IVC} = the accumulator of the IVC circuit.
//! u_app = the current instance of the application circuit.
//! u_ivc = the current instance of the IVC circuit.
//! ```
//! As an input, our circuit will take 4 different values, of 2 different types:
//! ```text
//! - u_a^{APP} = a first instance of the application circuit
//! - u_b^{APP} = a second instance of the application circuit
//! - u_a^{IVC} = a first instance of the IVC circuit
//! - u_b^{IVC} = a second instance of the IVC circuit
//! ```
//!
//! Regarding notation, in this library (and in the [folding] library), we use
//! the following correspondances:
//! ```text
//! acc_i (or U_i) = the (folded) left instance
//! u_i = the right instance
//! acc_(i + 1) = the (folded) output instance
//! ```

pub mod ivc;
/// Poseidon hash function with 55 full rounds, 0 partial rounds, sbox 7, a
/// state of 3 elements and constraints of degree 2
pub mod poseidon_55_0_7_3_2;
/// Poseidon hash function with 55 full rounds, 0 partial rounds, sbox 7,
/// a state of 3 elements and constraints of degree 7
pub mod poseidon_55_0_7_3_7;
/// Poseidon hash function with 8 full rounds, 56 partial rounds, sbox 5, a
/// state of 3 elements and constraints of degree 2
pub mod poseidon_8_56_5_3_2;
/// Poseidon parameters for 55 full rounds, 0 partial rounds, sbox 7, a state of
/// 3 elements
pub mod poseidon_params_55_0_7_3;
