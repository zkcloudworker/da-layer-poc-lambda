//! This module contains the definition of the `MVPoly` trait, which is used to
//! represent multi-variate polynomials.
//!
//! Different representations are provided in the sub-modules:
//! - `monomials`: a representation based on monomials
//! - `prime`: a representation based on a mapping from variables to prime
//! numbers. This representation is unmaintained for now. We leave it
//! for interested users.
//!
//! "Expressions", as defined in the [kimchi] crate, can be converted into a
//! multi-variate polynomial using the `from_expr` method.

use ark_ff::PrimeField;
use kimchi::circuits::expr::{
    ConstantExpr, ConstantExprInner, ConstantTerm, Expr, ExprInner, Operations, Variable,
};
use rand::RngCore;
use std::collections::HashMap;

pub mod monomials;
pub mod pbt;
pub mod prime;
pub mod utils;

/// Generic trait to represent a multi-variate polynomial
pub trait MVPoly<F: PrimeField, const N: usize, const D: usize>:
    // Addition
    std::ops::Add<Self, Output = Self>
    + for<'a> std::ops::Add<&'a Self, Output = Self>
    // Mul
    + std::ops::Mul<Self, Output = Self>
    // Negation
    + std::ops::Neg<Output = Self>
    // Sub
    + std::ops::Sub<Self, Output = Self>
    + for<'a> std::ops::Sub<&'a Self, Output = Self>
    + ark_ff::One
    + ark_ff::Zero
    + std::fmt::Debug
    + Clone
    // Comparison operators
    + PartialEq
    + Eq
    // Useful conversions
    + From<F>
    + Sized
{
    /// Generate a random polynomial of maximum degree `max_degree`.
    ///
    /// If `None` is provided as the maximum degree, the polynomial will be
    /// generated with a maximum degree of `D`.
    ///
    /// # Safety
    ///
    /// Marked as unsafe to warn the user to use it with caution and to not
    /// necessarily rely on it for security/randomness in cryptographic
    /// protocols. The user is responsible for providing its own secure
    /// polynomial random generator, if needed.
    ///
    /// For now, the function is only used for testing.
    unsafe fn random<RNG: RngCore>(rng: &mut RNG, max_degree: Option<usize>) -> Self;

    fn double(&self) -> Self;

    fn is_constant(&self) -> bool;

    fn mul_by_scalar(&self, scalar: F) -> Self;

    /// Returns the degree of the polynomial.
    ///
    /// The degree of the polynomial is the maximum degree of the monomials
    /// that have a non-zero coefficient.
    ///
    /// # Safety
    ///
    /// The zero polynomial as a degree equals to 0, as the degree of the
    /// constant polynomials. We do use the `unsafe` keyword to warn the user
    /// for this specific case.
    unsafe fn degree(&self) -> usize;

    /// Evaluate the polynomial at the vector point `x`.
    ///
    /// This is a dummy implementation. A cache can be used for the monomials to
    /// speed up the computation.
    fn eval(&self, x: &[F; N]) -> F;

    /// Build the univariate polynomial `x_i` from the variable `i`.
    /// The conversion into the type `usize` is unspecified by this trait. It
    /// is left to the trait implementation.
    /// For instance, in the case of [crate::prime], the output must be a prime
    /// number, starting at `2`. [crate::utils::PrimeNumberGenerator] can be
    /// used.
    /// For [crate::monomials], the output must be the index of the variable,
    /// starting from `0`.
    ///
    /// The parameter `offset_next_row` is an optional argument that is used to
    /// support the case where the "next row" is used. In this case, the type
    /// parameter `N` must include this offset (i.e. if 4 variables are in ued,
    /// N should be at least `8 = 2 * 4`).
    fn from_variable<Column: Into<usize>>(var: Variable<Column>, offset_next_row: Option<usize>) -> Self;

    fn from_constant<ChallengeTerm: Clone>(op: Operations<ConstantExprInner<F, ChallengeTerm>>) -> Self {
        use kimchi::circuits::expr::Operations::*;
        match op {
            Atom(op_const) => {
                match op_const {
                    ConstantExprInner::Challenge(_) => {
                        unimplemented!("Challenges are not supposed to be used in this context for now")
                    }
                    ConstantExprInner::Constant(ConstantTerm::EndoCoefficient) => {
                        unimplemented!(
                            "The constant EndoCoefficient is not supposed to be used in this context"
                        )
                    }
                    ConstantExprInner::Constant(ConstantTerm::Mds {
                        row: _row,
                        col: _col,
                    }) => {
                        unimplemented!("The constant Mds is not supposed to be used in this context")
                    }
                    ConstantExprInner::Constant(ConstantTerm::Literal(c)) => Self::from(c),
                }
            }
            Add(c1, c2) => Self::from_constant(*c1) + Self::from_constant(*c2),
            Sub(c1, c2) => Self::from_constant(*c1) - Self::from_constant(*c2),
            Mul(c1, c2) => Self::from_constant(*c1) * Self::from_constant(*c2),
            Square(c) => Self::from_constant(*c.clone()) * Self::from_constant(*c),
            Double(c1) => Self::from_constant(*c1).double(),
            Pow(c, e) => {
                // FIXME: dummy implementation
                let p = Self::from_constant(*c);
                let mut result = p.clone();
                for _ in 0..e {
                    result = result.clone() * p.clone();
                }
                result
            }
            Cache(_c, _) => {
                unimplemented!("The method is supposed to be used for generic multivariate expressions, not tied to a specific use case like Kimchi with this constructor")
            }
            IfFeature(_c, _t, _f) => {
                unimplemented!("The method is supposed to be used for generic multivariate expressions, not tied to a specific use case like Kimchi with this constructor")
            }
        }
    }

    /// Build a value from an expression.
    /// This method aims to be used to be retro-compatible with what we call
    /// "the expression framework".
    /// In the near future, the "expression framework" should be moved also into
    /// this library.
    ///
    /// The mapping from variable to the user is left unspecified by this trait
    /// and is left to the implementation. The conversion of a variable into an
    /// index is done by the trait requirement `Into<usize>` on the column type.
    ///
    /// The parameter `offset_next_row` is an optional argument that is used to
    /// support the case where the "next row" is used. In this case, the type
    /// parameter `N` must include this offset (i.e. if 4 variables are in ued,
    /// N should be at least `8 = 2 * 4`).
    fn from_expr<Column: Into<usize>, ChallengeTerm: Clone>(expr: Expr<ConstantExpr<F, ChallengeTerm>, Column>, offset_next_row: Option<usize>) -> Self {
        use kimchi::circuits::expr::Operations::*;

        match expr {
            Atom(op_const) => {
                match op_const {
                    ExprInner::UnnormalizedLagrangeBasis(_) => {
                        unimplemented!("Not used in this context")
                    }
                    ExprInner::VanishesOnZeroKnowledgeAndPreviousRows => {
                        unimplemented!("Not used in this context")
                    }
                    ExprInner::Constant(c) => Self::from_constant(c),
                    ExprInner::Cell(var) => {
                        Self::from_variable::<Column>(var, offset_next_row)
                    }
                }
            }
            Add(e1, e2) => {
                let p1 = Self::from_expr::<Column, ChallengeTerm>(*e1, offset_next_row);
                let p2 = Self::from_expr::<Column, ChallengeTerm>(*e2, offset_next_row);
                p1 + p2
            }
            Sub(e1, e2) => {
                let p1 = Self::from_expr::<Column, ChallengeTerm>(*e1, offset_next_row);
                let p2 = Self::from_expr::<Column, ChallengeTerm>(*e2, offset_next_row);
                p1 - p2
            }
            Mul(e1, e2) => {
                let p1 = Self::from_expr::<Column, ChallengeTerm>(*e1, offset_next_row);
                let p2 = Self::from_expr::<Column, ChallengeTerm>(*e2, offset_next_row);
                p1 * p2
            }
            Double(p) => {
                let p = Self::from_expr::<Column, ChallengeTerm>(*p, offset_next_row);
                p.double()
            }
            Square(p) => {
                let p = Self::from_expr::<Column, ChallengeTerm>(*p, offset_next_row);
                p.clone() * p.clone()
            }
            Pow(c, e) => {
                // FIXME: dummy implementation
                let p = Self::from_expr::<Column, ChallengeTerm>(*c, offset_next_row);
                let mut result = p.clone();
                for _ in 0..e {
                    result = result.clone() * p.clone();
                }
                result
            }
            Cache(_c, _) => {
                unimplemented!("The method is supposed to be used for generic multivariate expressions, not tied to a specific use case like Kimchi with this constructor")
            }
            IfFeature(_c, _t, _f) => {
                unimplemented!("The method is supposed to be used for generic multivariate expressions, not tied to a specific use case like Kimchi with this constructor")
            }
        }
    }

    /// Returns true if the polynomial is homogeneous (of degree `D`).
    /// As a reminder, a polynomial is homogeneous if all its monomials have the
    /// same degree.
    fn is_homogeneous(&self) -> bool;

    /// Evaluate the polynomial at the vector point `x` and the extra variable
    /// `u` using its homogeneous form of degree D.
    fn homogeneous_eval(&self, x: &[F; N], u: F) -> F;

    /// Add the monomial `coeff * x_1^{e_1} * ... * x_N^{e_N}` to the
    /// polynomial, where `e_i` are the values given by the array `exponents`.
    ///
    /// For instance, to add the monomial `3 * x_1^2 * x_2^3` to the polynomial,
    /// one would call `add_monomial([2, 3], 3)`.
    fn add_monomial(&mut self, exponents: [usize; N], coeff: F);

    /// Compute the cross-terms as described in [Behind Nova: cross-terms
    /// computation for high degree
    /// gates](https://hackmd.io/@dannywillems/Syo5MBq90)
    ///
    /// The polynomial must not necessarily be homogeneous. For this reason, the
    /// values `u1` and `u2` represents the extra variable that is used to make
    /// the polynomial homogeneous.
    ///
    /// The homogeneous degree is supposed to be the one defined by the type of
    /// the polynomial, i.e. `D`.
    ///
    /// The output is a map of `D - 1` values that represents the cross-terms
    /// for each power of `r`.
    fn compute_cross_terms(
        &self,
        eval1: &[F; N],
        eval2: &[F; N],
        u1: F,
        u2: F,
    ) -> HashMap<usize, F>;

    /// Modify the monomial in the polynomial to the new value `coeff`.
    fn modify_monomial(&mut self, exponents: [usize; N], coeff: F);

    /// Return true if the multi-variate polynomial is multilinear, i.e. if each
    /// variable in each monomial is of maximum degree 1.
    fn is_multilinear(&self) -> bool;
}

/// Compute the cross terms of a list of polynomials. The polynomials are
/// linearly combined using the power of a combiner, often called `α`.
/// Powers of the combiner are considered as an additional variable of the
/// multi-variate polynomials, therefore increasing the degree of the sum of all
/// polynomials by one. In other words, if we combine (M + 1) polynomials
/// `P_1(X_1, ..., X_N)`, ..., `P_{M + 1}(X_1, ..., X_N)`, we will compute the
/// cross-terms of the polynomial:
///
/// ```text
/// P(X_1, ..., X_N, α_1, ..., α_M) =     P_1(X_1, ..., X_N)
///                                 + α_1 P_2(X_1, ..., X_N)
///                                 + ...
///                                 + α_M P_{M + 1}(X_1, ..., X_N)
/// ```
///
/// We notice that we simply do have a shifted linear combination of the
/// polynomials. Based on the property that the cross-terms of a sum of
/// polynomials is the sum of the cross-terms of the individual polynomials, and
/// that shifting a polynomial with a new variable simply requires to shift the
/// power of cross-terms by one and multiply by the evaluation at the new point,
/// we get a simple algorithm.
///
/// All polynomials are supposed to be given as multivariate polynomials of the
/// same degree and the same number of variables, even if they do not have the
/// same number of variables nor the same degree.
/// Therefore, `N` is the maximum number of variables of the polynomials and `D - 1`
/// is the maximum degree of the polynomials, `D` being the the degree of the
/// combined polynomial when including the combiner.
///
/// The number of keys in the output is always `D`.
pub fn compute_combined_cross_terms<
    F: PrimeField,
    const N: usize,
    const D: usize,
    T: MVPoly<F, N, D>,
>(
    polys: Vec<T>,
    combiner1: F,
    combiner2: F,
    eval1: [F; N],
    eval2: [F; N],
    u1: F,
    u2: F,
) -> HashMap<usize, F> {
    let mut cross_terms = HashMap::new();
    polys.into_iter().enumerate().for_each(|(i, poly)| {
        let cross_terms_poly: HashMap<usize, F> = poly.compute_cross_terms(&eval1, &eval2, u1, u2);
        cross_terms_poly.into_iter().for_each(|(p, value)| {
            // Computing α^i * value, and adding it to the same power of r
            // Handling 0^0 = 1
            let entry = cross_terms.entry(p).or_insert(F::zero());
            if combiner1 != F::zero() {
                let alpha_i = combiner1.pow([i as u64]);
                *entry += alpha_i * value;
            };
            // Computing α'^i * value, and adding it to the next power of r
            // Handling 0^0 = 1
            let entry_prime = cross_terms.entry(p + 1).or_insert_with(F::zero);
            if combiner2 != F::zero() {
                let alpha_i = combiner2.pow([i as u64]);
                *entry_prime += alpha_i * value;
            };
        });
    });
    cross_terms
}
