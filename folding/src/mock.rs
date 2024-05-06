use crate::{
    checker::Alphas,
    error_term::Side,
    expressions::{extract_terms, FoldingColumnTrait},
    FoldingConfig, FoldingEnv, Instance, Sponge, Witness,
};
use ark_ec::AffineCurve;
use ark_ff::Field;
use itertools::Itertools;
use kimchi::circuits::gate::CurrOrNext::{self, Curr};
use mina_curves::pasta::Pallas;
use num_traits::Zero;
use poly_commitment::commitment::CommitmentCurve;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Mock;

impl<F: Field> From<F> for Mock {
    fn from(_x: F) -> Self {
        Mock
    }
}

impl FoldingColumnTrait for u8 {
    fn is_witness(&self) -> bool {
        true
    }
}

impl<G: CommitmentCurve> Sponge<G> for Mock {
    fn challenge(_absorbe: &[poly_commitment::PolyComm<G>; 2]) -> <G>::ScalarField {
        panic!("just for test")
    }
}

impl<G: CommitmentCurve> Instance<G> for Mock {
    fn combine(_a: Self, _b: Self, _challenge: G::ScalarField) -> Self {
        Mock
    }
    fn alphas(&self) -> &Alphas<G> {
        todo!()
    }
    fn challenges(&self) -> &[<G>::ScalarField] {
        todo!()
    }
}

impl<G: CommitmentCurve> Witness<G> for Mock {
    fn combine(_a: Self, _b: Self, _challenge: G::ScalarField) -> Self {
        Mock
    }
}

struct MockEnv<F, I, W, Col, Chal>(PhantomData<(F, I, W, Col, Chal)>);
impl<F, I, W, Col, Chal> FoldingEnv<F, I, W, Col, Chal, ()> for MockEnv<F, I, W, Col, Chal> {
    type Structure = Mock;

    fn zero_vec(&self) -> Vec<F> {
        todo!()
    }

    fn col(&self, _col: Col, _curr_or_next: CurrOrNext, _side: Side) -> &Vec<F> {
        todo!()
    }

    fn challenge(&self, _challenge: Chal, _side: Side) -> F {
        todo!()
    }

    fn new(_structure: &Self::Structure, _instances: [&I; 2], _witnesses: [&W; 2]) -> Self {
        todo!()
    }

    fn lagrange_basis(&self, _i: usize) -> &Vec<F> {
        todo!()
    }

    fn alpha(&self, _i: usize, _side: Side) -> F {
        todo!()
    }

    fn selector(&self, _s: &(), _side: Side) -> &Vec<F> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct TestConfig;
impl FoldingConfig for TestConfig {
    type Column = u8;

    type Selector = ();

    type Challenge = Mock;

    type Curve = Pallas;

    type Srs = poly_commitment::srs::SRS<Pallas>;

    type Sponge = Mock;

    type Instance = Mock;

    type Witness = Mock;

    type Structure = Mock;

    type Env = MockEnv<<Pallas as AffineCurve>::ScalarField, Mock, Mock, u8, Mock>;

    fn rows() -> usize {
        todo!()
    }
}

#[test]
//not testing much right now, just to observe what quadraticization does
fn test_term_separation() {
    use crate::{columns::ExtendedFoldingColumn, expressions::FoldingExp};
    use kimchi::circuits::expr::Variable;
    let col = |col| FoldingExp::Atom(ExtendedFoldingColumn::Inner(Variable { col, row: Curr }));
    let t1: FoldingExp<TestConfig> = (col(0) + col(1)) * (col(2) + col(3));
    let t2 = col(1).double()
        - (col(2)
            + FoldingExp::Atom(ExtendedFoldingColumn::Constant(
                <<Pallas as AffineCurve>::ScalarField>::zero(),
            )));
    let test_exp = t1 + t2;
    println!("{:#?}", test_exp);
    let terms = extract_terms(test_exp).collect_vec();
    println!("{:#?}", terms);
}
