use ark_poly::{multivariate::{SparsePolynomial, SparseTerm, Term}, DenseMVPolynomial};
use sumcheck::ScalarField;

mod sumcheck;

fn main() {
    // g = 2(x_1)^3 + (x_1)(x_3) + (x_2)(x_3)
    let G_0: sumcheck::MultiPoly = SparsePolynomial::from_coefficients_vec(
		3,
		vec![
			(2u32.into(), SparseTerm::new(vec![(0, 3)])),
			(1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
			(1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
		],
	);
    let G_0_SUM: ScalarField = sumcheck::Prover::new(&G_0).slow_sum_g();
    assert!(sumcheck::verify(&G_0, G_0_SUM));

	// Test with a larger g
	let G_1: sumcheck::MultiPoly = SparsePolynomial::from_coefficients_vec(
		4,
		vec![
			(2u32.into(), SparseTerm::new(vec![(0, 3)])),
			(1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
			(1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
			(1u32.into(), SparseTerm::new(vec![(3, 1), (2, 1)])),
		],
	);
	let G_1_SUM: ScalarField = sumcheck::Prover::new(&G_1).slow_sum_g();
    assert!(sumcheck::verify(&G_1, G_1_SUM));
}
