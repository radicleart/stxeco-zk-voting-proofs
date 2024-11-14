use crate::proofs::ProofVerifier;
use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin}, math::{fields::f128::BaseElement, FieldElement, ToElements}, Proof, ProofOptions, Prover, TraceTable
};

use super::{PublicInputs, WorkAir};

pub struct StacksVotingProofVerifier;

impl ProofVerifier for StacksVotingProofVerifier {
    fn verify_proof(balance_at_height: u128, burn_start_height: u128, proof_in: Vec<u8>) -> bool {
        // Your proof generation logic for `proof1`
        // Call the `proof1` specific function (this is just an example)
        verify_stacks_voting_proof(balance_at_height, burn_start_height, &proof_in)
    }
}

fn verify_stacks_voting_proof(balance_at_height_in: u128, burn_start_height_in: u128, proof_in: &[u8]) -> bool {
    // The verifier will accept proofs with parameters which guarantee 95 bits or more of
    // conjectured security
    let balance_at_height: BaseElement = BaseElement::new(balance_at_height_in);
    let burn_start_height: BaseElement = BaseElement::new(burn_start_height_in);
    let proof: Proof = Proof::from_bytes(proof_in).unwrap();
    let min_opts = winterfell::AcceptableOptions::MinConjecturedSecurity(95);

    // Verify the proof. The number of steps and options are encoded in the proof itself,
    // so we don't need to pass them explicitly to the verifier.
    let pub_inputs = PublicInputs { start: burn_start_height, result: balance_at_height };
    winterfell::verify::<WorkAir,
                                Blake3_256<BaseElement>,
                                DefaultRandomCoin<Blake3_256<BaseElement>>
                                >(proof, pub_inputs, &min_opts).is_ok()
}