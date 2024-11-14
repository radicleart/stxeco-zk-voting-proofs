use base64::{engine::general_purpose, Engine};
use warp::{reject::Rejection, Filter};

use crate::{proofs::{stacks_voting::{SignatureData, StacksVotingProofGenrator}, ApplicationResponseMessage, ProofResponse, VotingProofGenerator}, stacks::transactions::TransactionFetchError};

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
    let (proof, result);
    if let Ok(transactions) = fetch_all_transactions(&public_key).await {
        (proof, result) = StacksVotingProofGenrator::generate_proof(signature_data, transactions);
    } else {
        eprintln!("Failed to retrieve transactions.");
        return Err(warp::reject::custom(TransactionFetchError {
            message: "Failed to fetch transactions".to_string(),
        }));
    }

    let b64_proof = general_purpose::STANDARD.encode(proof);
    let response = ProofResponse::StacksVotingProof {
        result: result.to_string(),
        proof: b64_proof,
    };
    let application_response = ApplicationResponseMessage::ProofGenerationResponse(response);
    println!("Message: {}", public_key);
    Ok(application_response)
}

async fn validate_proof(body: SignatureData) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Validating proof with public key: {:?}", body);
    Ok(warp::reply::json(&serde_json::json!({"status": "proof validated"})))
}
