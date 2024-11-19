use base64::{engine::general_purpose, Engine};
use warp::{reject::Rejection, Filter};

use crate::{proofs::{stacks_voting::{SignatureData, StacksVotingProofGenrator}, ApplicationResponseMessage, ProofResponse, VotingProofGenerator}, stacks::{transactions::TransactionFetchError, utils::public_key_to_stacks_address, ProofError}};

use super::utils::fetch_all_transactions;

pub fn proofs_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("proof")
        .and(
            warp::path("generate")
                .and(warp::post())
                .and(warp::body::json::<SignatureData>())
                .and_then(|signature_data: SignatureData| async move {
                    match generate_proof(signature_data).await {
                        Ok(response) => Ok(warp::reply::json(&response)),
                        Err(e) => Err(e), // Pass along rejections
                    }
                })
            .or(
                warp::path("validate")
                    .and(warp::post())
                    .and(warp::body::json::<SignatureData>())
                    .and_then(validate_proof)
            )
        )
}

pub async fn generate_proof(signature_data: SignatureData) -> Result<ApplicationResponseMessage, Rejection> {
    let public_key = signature_data.public_key.clone();
    
    let stacks_address = public_key_to_stacks_address(public_key)
        .map_err(|e| warp::reject::custom(ProofError::new(&format!("Address conversion error: {:?}", e))))?;

    let transactions = fetch_all_transactions(&stacks_address).await
        .map_err(|e| warp::reject::custom(ProofError::new(&format!("Transaction fetch error: {}", e))))?;

    let (proof, result) = StacksVotingProofGenrator::generate_proof(signature_data, transactions);

    let b64_proof = general_purpose::STANDARD.encode(proof);

    // Prepare response
    let response = ProofResponse::StacksVotingProof {
        result: result.to_string(),
        proof: b64_proof,
    };
    let application_response = ApplicationResponseMessage::ProofGenerationResponse(response);
    println!("Message: {}", stacks_address);
    Ok(application_response)
}

async fn validate_proof(body: SignatureData) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Validating proof with public key: {:?}", body);
    Ok(warp::reply::json(&serde_json::json!({"status": "proof validated"})))
}
