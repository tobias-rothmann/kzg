# Implementing the KZG according to the original paper in Rust

### Disclaimer
This is a toy/concept implementation and not for use in real-world applications.

## Source Paper 
This implementation reffers to the original paper "Polynomial Commitments" by Aniket Kate, Gregory M. Zaverucha, and Ian Goldberg. To be found at: https://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf

## Implementation
This implementation tries to mirror the original paper definition as close as possible. 
However, this is not always easily possbile, as finding elliptic curve bilinear group pairing implementations with only two groups (as described in the original paper) is basically impossible. Instead, the implementation uses the popular bls12_381 implementation with 3 groups. That is however the only notable change from the original paper. 

## The polynomial commitment scheme Trait
In order to support easy exchangable implementations of polynomial commitment schemes, for e.g. use in different plonk versions (DL-KZG-PLONK as SNARK without zero knowledge, PED-KZG-PLONK as zk-SNARK, FRI-PLONK as STARK), I define a polynomial commitment scheme trait upfront, which is implemented by every consruction-implementation. 
#### Done

## DL-Construction
Mirroring section 3.2 of the original paper, kzg_dl.rs implements the KZG construction based on the Discrete Log assumption. 
#### Done

## PED-CONSTRUCTION 
Mirroring section 3.3 of the original paper, kzg_ped implements the KZG construction based on Pedersen-Commitments, which is basically the DL-Constrcution with Pedersen-Commitment style multiplication of random values on to commitments and hence it grants a bit stronger privacy gurantees in exchange for a bit more prover overhead. 
#### Done

## Batch Opening 
Mirroring section 3.4 Batch Opening of the original paper, the batch-opening trait mirrors bacth-opening for a polynomial commitmenrt schemeand is implemented for the DL and the PED construction.

#### TODO