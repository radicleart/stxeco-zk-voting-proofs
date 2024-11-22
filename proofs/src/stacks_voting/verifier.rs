use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin}, math::fields::f128::BaseElement, Proof
};

use crate::ProofVerifier;

use super::{PublicInputs, StacksVotingProofVerifier, WorkAir};

impl ProofVerifier for StacksVotingProofVerifier {
    fn verify_proof(balance_at_height: u128, block_proof_height: u128, proof_in: Vec<u8>) -> bool {
        // Your proof generation logic for `proof1`
        // Call the `proof1` specific function (this is just an example)
        verify_stacks_voting_proof(balance_at_height, block_proof_height, &proof_in)
    }
}

fn verify_stacks_voting_proof(balance_at_height_in: u128, block_proof_height_in: u128, proof_in: &[u8]) -> bool {
    // The verifier will accept proofs with parameters which guarantee 95 bits or more of
    // conjectured security
    let balance_at_height: BaseElement = BaseElement::new(balance_at_height_in);
    let block_proof_height: BaseElement = BaseElement::new(block_proof_height_in);
    let proof: Proof = Proof::from_bytes(proof_in).unwrap();
    let min_opts = winterfell::AcceptableOptions::MinConjecturedSecurity(95);

    // Verify the proof. The number of steps and options are encoded in the proof itself,
    // so we don't need to pass them explicitly to the verifier.
    let pub_inputs = PublicInputs { balance: balance_at_height, height: block_proof_height };
    winterfell::verify::<WorkAir,
                                Blake3_256<BaseElement>,
                                DefaultRandomCoin<Blake3_256<BaseElement>>
                                >(proof, pub_inputs, &min_opts).is_ok()
}