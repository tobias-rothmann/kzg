//implement the Pederesen-construction of the KZG 
//(see section 3.3 in the original paper: https://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf)

use super::poly::Poly;
use bls12_381::*;

pub struct KZG {
    pub pk_g1: Vec<G1Projective>,
    pub pk_g2: Vec<G2Projective>,
}

pub type PolynomialCommitment = G1Projective;
pub type Witness = G1Projective;
pub type Index = Scalar;
pub type Evaluation = Scalar; 