//! Zero gate
//!
//! This gate implements nothing

use std::marker::PhantomData;

use ark_ff::PrimeField;

use crate::{
    circuits::{
        argument::ArgumentEnv,
        expr::{constraints::ExprOps, Cache},
        gate::Gate,
    },
    define_gate,
};

define_gate!(
    Zero<F: PrimeField>,
    "Implements the empty gate without any constraints"
);

impl<F: PrimeField, T: ExprOps<F>> Gate<F, T> for Zero<F> {
    fn constraint_checks(&self, env: &ArgumentEnv<F, T>, cache: &mut Cache) -> Vec<T> {
        vec![]
    }
}
