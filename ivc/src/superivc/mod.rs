//! This module contains an implementation of a variant of the non uniform IVC
//! circuit (NIVC) described in the paper
//! [SuperNova](https://eprint.iacr.org/2022/1758) to be used with the library
//! [folding](folding) of this monorepo.
//!
//! The circuit generalizes the IVC circuit described in the paper
//! [Nova](https://eprint.iacr.org/2021/370) to support non-uniform circuit by
//! adding a list of accumulators ("running instances") and by adding a
//! polynomial-time function φ that will be used to select the function to be
//! run in the circuit.
//!
//! The circuit can be used with the `o1vm` interpreter to run a
//! zero-knowledge virtual machine.
//! To encode the instructions of the virtual machine, the module will use the
//! encoding of the 32-bit instructions.
//! Suppose that there are N instructions, and their encodings is the 32bits
//! value `{x_1, x_1, ..., x_{N}}`.
//! The function φ will be the polynomial function that results of the
//! interpolation of the points `{(i, x_i)}` for `i = 1, ..., N`.
//! In the circuit, we will constrain the evaluation to be equal to zero to
//! encode the condition that `φ((z_i, w_i)) is in [1, ..., N]`, where `(z_i,,
//! w_i)` is the witness of the current instance.
//!
//! As the degree of this function is high (N - 1), we will encode the
//! evaluation of it to degree 2 constraints. The number of columns of the
//! circuit will restrict the number of instructions we can encode. However, as
//! described below, the number of columns will be significantly high to allow
//! the encoding of a large number of instructions, at least enough for a
//! reasonable virtual machine.
//!
//! The paper SuperNova encodes the instructions by polynomial-time function
//! `F_1`, ..., `F_N`. In this module, we want to allow a parallelisation of the
//! NIVC scheme. For that, we will first suppose that all the instructions of
//! the ISA the virtual machine encodes are given by the functions `F_1`, ...,
//! `F_M`.
//! We will suppose that the execution of the function `F_i` is simply given by
//! hashing the commitments to the columns generated by the execution of the
//! function `F_i`. It will give us the value `z_i`.

pub mod columns;
pub mod constraints;
pub mod interpreter;
pub mod witness;
