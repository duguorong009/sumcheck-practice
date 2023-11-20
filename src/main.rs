use ark_poly::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::univariate::SparsePolynomial as UniSparsePolynomial;
use ark_poly::polynomial::{DenseMVPolynomial, Polynomial};
use ark_std::cfg_into_iter;
use ark_ff::Field;

fn main() {
    println!("Hello, world!");
}

type ScalarField = ark_bls12_381::Fr;
type MultiPoly = SparsePolynomial<ScalarField, SparseTerm>;
type UniPoly = UniSparsePolynomial<ScalarField>;

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

    // Given polynomial g, fix Xj, evaluate over xj+1
    pub fn gen_uni_polynomial(&mut self, r: Option<ScalarField>) -> UniPoly {
        if r.is_some() {
            self.r_vec.push(r.unwrap());
        }

        let v = self.g.num_vars - self.r_vec.len();
        (0..(2_u32.pow(v as u32 - 1))).fold(
            UniPoly::from_coefficients_vec(vec![(0, 0_u32.into())]),
            |sum, n| sum + self.evaluate_gj(n_to_vec(n as usize, v)),
        )
    }

    pub fn evaluate_gj(&self, points: Vec<ScalarField>) -> UniPoly {
        cfg_into_iter!(self.g.terms()).fold(
            UniPoly::from_coefficients_vec(vec![]),
            |sum, (coeff, term)| {
                let (coeff_eval, fixed_term) = self.evaluate_term(&term, &points);
				let curr = match fixed_term {
					None => UniPoly::from_coefficients_vec(vec![(0, *coeff * coeff_eval)]),
					_ => UniPoly::from_coefficients_vec(vec![(
						fixed_term.unwrap().degree(),
						*coeff * coeff_eval,
					)]),
				};
				curr + sum
            }
        )
    }

    pub fn evaluate_term(
		&self,
		term: &SparseTerm,
		point: &Vec<ScalarField>,
	) -> (ScalarField, Option<SparseTerm>) {
		let mut fixed_term: Option<SparseTerm> = None;
		let coeff: ScalarField =
			cfg_into_iter!(term).fold(1u32.into(), |product, (var, power)| match *var {
				j if j == self.r_vec.len() => {
					fixed_term = Some(SparseTerm::new(vec![(j, *power)]));
					product
				}
				j if j < self.r_vec.len() => self.r_vec[j].pow(&[*power as u64]) * product,
				_ => point[*var - self.r_vec.len()].pow(&[*power as u64]) * product,
			});
		(coeff, fixed_term)
	}

    // Sum all evaluations of polynomial `g` over boolean hypercube
	pub fn slow_sum_g(&self) -> ScalarField {
		let v = self.g.num_vars();
		let n = 2u32.pow(v as u32);
		(0..n)
			.map(|n| self.g.evaluate(&n_to_vec(n as usize, v)))
			.sum()
	}
}


// Converts i into an index in {0,1}^v
pub fn n_to_vec(i: usize, n: usize) -> Vec<ScalarField> {
	format!("{:0>width$}", format!("{:b}", i), width = n)
		.chars()
		.map(|x| if x == '1' { 1.into() } else { 0.into() })
		.collect()
}


struct Verifier;

impl Verifier {
    pub fn verify(g: &MultiPoly, c_1: ScalarField) -> bool {
        todo!()
    }
}
