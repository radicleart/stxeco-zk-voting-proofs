use warp::Filter;
use warp::reject::Reject;
use std::fmt;

mod transactions;
pub mod proofs;
pub mod utils;

// Combines all Stacks-related routes
pub fn stacks_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("stacks").and(
        transactions::transactions_routes()
            .or(proofs::proofs_routes())
    )
}

#[derive(Debug)]
pub struct ProofError {
    message: String,
}

impl ProofError {
    fn new(msg: &str) -> Self {
        ProofError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for ProofError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Make `ProofError` implement `Reject`
impl Reject for ProofError {}
