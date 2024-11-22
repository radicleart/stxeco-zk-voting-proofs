use warp::{filters::cors::cors, Filter};

mod transactions;
mod proofs;

// Combines all Stacks-related routes
pub fn stacks_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = cors()
        .allow_origin("http://localhost:8060") // Allow specific origin
        .allow_methods(vec!["POST", "GET"])   // Allow specific HTTP methods
        .allow_headers(vec!["Content-Type"]); // Allow specific headers
    
    warp::path("stacks").and(
        transactions::transactions_routes().with(&cors)
            .or(proofs::proofs_routes()).with(&cors)
    )
}

