//implement the DL-construction of the KZG 
//(see section 3.2 in the original paper: https://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf)

use super::poly::Poly;
use super::polynomial_commitment_scheme::PolynomialCommitmentScheme;
use bls12_381::*;

pub struct KZG {
    pub pk_g1: Vec<G1Projective>,
    pub pk_g2: Vec<G2Projective>,
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
}

impl PolynomialCommitmentScheme<Scalar, Poly, PolynomialCommitment, Witness> for KZG {

    /*
        The KZG as descirbed in section 3.2 in the original paper: 
        https://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf
        implemented as poloynomial commitment scheme
     */
    

    fn commit(&self, poly : &Poly) -> PolynomialCommitment {
       Self::eval_on_pk_g1(&self, poly)
    }

    fn verify(&self, c: PolynomialCommitment, poly: &Poly) -> bool {
        c == self.commit(poly)
    }

    fn create_witness(&self, poly : &Poly, i: &Index) -> Witness {
        let x_minus_i_poly = Poly::new(vec![-i, Scalar::one()]);
        let mut psi_upper_part = poly.clone();
        psi_upper_part -= &Poly::new(vec![poly.eval(i)]);
        let (psi, remainder) = psi_upper_part/x_minus_i_poly;
        assert!(remainder.is_zero());
        Self::eval_on_pk_g1(&self, &psi)
    }

    fn verify_witness(&self, c: &PolynomialCommitment, i: &Index, phi_eval_i : &Evaluation, w : &Witness) -> bool {
        let e_c_g = pairing(&c.into(), &G2Projective::generator().into());
        let e_check = pairing(&w.into(), &(self.pk_g2[1] - G2Projective::generator()*i).into()) 
                        + pairing(&G1Projective::generator().into(), &G2Projective::generator().into()) * phi_eval_i;
        e_c_g == e_check
    }

}