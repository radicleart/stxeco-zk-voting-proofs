use crate::{stacks_voting::utils::fetch_all_transactions, PeerMap};

pub async fn get_transactions(address: String, _state: PeerMap) -> Result<impl warp::Reply, warp::Rejection> {
    match fetch_all_transactions(&address).await {
        Ok(transactions) => Ok(warp::reply::json(&transactions)),
        Err(e) => {
            eprintln!("Error fetching or parsing response from : {:?}", e);
            Err(warp::reject::not_found())
        }
    }
}