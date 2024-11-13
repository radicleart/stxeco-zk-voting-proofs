use warp::Filter;

mod transactions;
mod proofs;
mod utils;

// Combines all Stacks-related routes
pub fn stacks_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("stacks").and(
        transactions::transactions_routes()
            .or(proofs::proofs_routes())
    )
}
