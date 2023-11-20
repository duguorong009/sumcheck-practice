use ark_poly::multivariate::{SparsePolynomial, SparseTerm};

fn main() {
    println!("Hello, world!");
}

type ScalarField = ark_bls12_381::Fr;
type MultiPoly = SparsePolynomial<ScalarField, SparseTerm>;

#[derive(Debug)]
struct Prover {
    pub g: MultiPoly,
    pub r_vec: Vec<ScalarField>,
}

impl Prover {
    pub fn new(g: &MultiPoly) -> Self {
        Self {
            g: g.clone(),
            r_vec: vec![],
        }
    }
}

struct Verifier;

impl Verifier {
    pub fn verify(g: &MultiPoly, c_1: ScalarField) -> bool {
        todo!()
    }
}