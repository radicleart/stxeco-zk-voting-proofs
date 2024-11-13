use warp::Filter;
use std::convert::Infallible;
use serde::Serialize;

#[derive(Serialize)]
struct TransactionsResponse {
    transactions: Vec<Transaction>,
    total: usize,
}

async fn get_transactions(address: String) -> Result<impl warp::Reply, Infallible> {
    match fetch_all_transactions(&address).await {
        Ok(transactions) => {
            let response = TransactionsResponse {
                transactions,
                total: transactions.len(),
            };
            Ok(warp::reply::json(&response))
        }
        Err(_) => {
            // Return a 500 Internal Server Error if fetching transactions fails
            Ok(warp::reply::with_status(
                warp::reply::json(&"Failed to fetch transactions"),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}