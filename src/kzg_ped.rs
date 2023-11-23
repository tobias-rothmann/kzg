//implement the Pedersen-construction of the KZG 
//(see section 3.3 in the original paper: https://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf)

use super::poly::Poly;
use super::polynomial_commitment_scheme::PolynomialCommitmentScheme;
use bls12_381::*;

pub struct KZG {
    //second generator h
    pub h: G1Projective,
    //public keys
    pub pk_g1: Vec<G1Projective>,
    pub pk_h1: Vec<G1Projective>,
    pub pk_g2: Vec<G2Projective>,
    pub pk_h2: Vec<G1Projective>,
}

pub type PolynomialCommitment = G1Projective;
pub type Witness = G1Projective;
pub type Index = Scalar;
pub type Evaluation = Scalar; 

impl KZG {
    fn eval_on_pk_g1(&self, poly : &Poly) -> G1Projective {
        poly.0
            .iter()
            .enumerate()
            .fold(G1Projective::identity(), |acc, (i, k)| {
                acc + self.pk_g1[i] * k
            })
    }

    fn eval_on_pk_h1(&self, poly : &Poly) -> G1Projective {
        poly.0
            .iter()
            .enumerate()
            .fold(G1Projective::identity(), |acc, (i, k)| {
                acc + self.pk_h1[i] * k
            })
    }
}

impl PolynomialCommitmentScheme<Scalar, (Poly,Poly), PolynomialCommitment, (Evaluation, Witness)> for KZG {

    /*
        The KZG as described in section 3.2 in the original paper: 
        https://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf
        implemented as poloynomial commitment scheme
     */
    
    fn commit(&self, (poly, poly_prime) : &(Poly, Poly)) -> PolynomialCommitment {
        Self::eval_on_pk_g1(&self, poly) + Self::eval_on_pk_h1(&self, poly_prime)
    }

    fn verify(&self, c: PolynomialCommitment, (poly, poly_prime) : &(Poly, Poly)) -> bool {
        c == self.commit(&(poly.clone(), poly_prime.clone()))
    }

    fn create_witness(&self, (poly, poly_prime) : &(Poly, Poly), i: &Index) ->  (Evaluation, Witness) {
        let (psi, remainder) = (poly.clone() - &Poly::new(vec![poly.eval(i)]))
                                            /Poly::new(vec![-i, Scalar::one()]);
        let (psi_prime, remainder_prime) = (poly_prime.clone() - &Poly::new(vec![poly_prime.eval(i)]))
                                            /Poly::new(vec![-i, Scalar::one()]);
        assert!(remainder.is_zero() && remainder_prime.is_zero());
        (Poly::eval(poly_prime, i), Self::eval_on_pk_g1(&self, &psi) + Self::eval_on_pk_g1(&self, &psi_prime))
    }

    fn verify_witness(&self, c: &PolynomialCommitment, i: &Index, phi_eval_i : &Evaluation, (phi_prime_eval_i, w) : &(Evaluation, Witness)) -> bool {
        let e_c_g = pairing(&c.into(), &G2Projective::generator().into());
        let e_check = pairing(&w.into(), &(self.pk_g2[1] - G2Projective::generator()*i).into()) 
                        + pairing(&(G1Projective::generator() * phi_eval_i + self.h * phi_prime_eval_i).into(), &G2Projective::generator().into());
        e_c_g == e_check
    }

}