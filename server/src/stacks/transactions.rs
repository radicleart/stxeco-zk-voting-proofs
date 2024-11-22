use proofs::stacks_voting::stacks_blockchain_utils::fetch_all_transactions;
use warp::Filter;


pub fn transactions_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("transactions")
        .and(warp::path::param::<String>())
        .and_then(get_transactions)
}

pub async fn get_transactions(address: String) -> Result<impl warp::Reply, warp::Rejection> {
    match fetch_all_transactions(&address).await {
        Ok(transactions) => Ok(warp::reply::json(&transactions)),
        Err(e) => {
            eprintln!("Error fetching or parsing response from : {:?}", e);
            Err(warp::reject::not_found())
        }
    }
}