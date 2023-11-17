
pub trait PolynomialCommitmentScheme<FiniteField, Poly, PolynomialCommitment, Witness>{

    fn commit(&self, poly : &Poly) -> PolynomialCommitment;

    fn verify(&self, c: PolynomialCommitment, poly: &Poly) -> bool;

    fn create_witness(&self, poly : &Poly, i: &FiniteField) -> Witness;

    fn verify_witness(&self, c: &PolynomialCommitment, i: &FiniteField, phi_eval_i : &FiniteField, w : &Witness) -> bool;
}