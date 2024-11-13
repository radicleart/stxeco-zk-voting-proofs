use warp::Filter;

use crate::proofs::stacks_voting::SignatureData;

pub fn proofs_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("proof")
        .and(
            warp::path("generate")
                .and(warp::post())
                .and(warp::body::json::<SignatureData>())
                .and_then(generate_proof)
            .or(
                warp::path("validate")
                    .and(warp::post())
                    .and(warp::body::json::<SignatureData>())
                    .and_then(validate_proof)
            )
        )
}

async fn generate_proof(body: SignatureData) -> Result<impl warp::Reply, warp::Rejection> {
    let message_inputs = body.message_inputs;
    let public_key = body.public_key;
    println!("Validating proof with public key: {}", public_key);
    println!("Message: {}", message_inputs.message);
    Ok(warp::reply::json(&serde_json::json!({"status": "proof generated"})))
}

async fn validate_proof(body: SignatureData) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Validating proof with public key: {:?}", body);
    Ok(warp::reply::json(&serde_json::json!({"status": "proof validated"})))
}
