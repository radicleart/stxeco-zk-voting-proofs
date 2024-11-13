use warp::Filter;
use crate::{stacks_voting::transactions::get_transactions, PeerMap};

// Define a function to set up the transactions route
pub fn transactions_routes(state: PeerMap) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("transactions" / String)
        .and(with_state(state))
        .and_then(|address, state| async move { get_transactions(address, state).await })
}

// A helper function to clone and share the state with each request
fn with_state(state: PeerMap) -> impl Filter<Extract = (PeerMap,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
