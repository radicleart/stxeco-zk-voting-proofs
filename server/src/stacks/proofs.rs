use bytes::Bytes;
use proofs::{handle_message, ProofsError};
use warp::Filter;


#[derive(Debug)]
struct Utf8Error;

impl warp::reject::Reject for Utf8Error {}

#[derive(Debug)]
pub struct RejectionError(pub ProofsError);

impl warp::reject::Reject for RejectionError {}


pub fn proofs_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("proof")
        .and(
            warp::path("generate")
                .and(warp::post())
                .and(warp::body::bytes())
                .and_then(|body: Bytes| async move {
                    let body_str = match std::str::from_utf8(&body) {
                        Ok(msg) => msg,
                        Err(_) => return Err(warp::reject::custom(Utf8Error)),
                    };

                    // Call the core logic and convert the result
                    handle_message(body_str)
                        .await
                        .map(|response| warp::reply::json(&response)) // Convert to warp::Reply
                        .map_err(|e| warp::reject::custom(RejectionError(e)))
                })
            .or(
                warp::path("validate")
                    .and(warp::post())
                    .and(warp::body::bytes())
                    .and_then(|body: Bytes| async move {
                        let body_str = match std::str::from_utf8(&body) {
                            Ok(msg) => msg,
                            Err(_) => return Err(warp::reject::custom(Utf8Error)),
                        };

                        handle_message(body_str)
                            .await
                            .map(|response| warp::reply::json(&response))
                            .map_err(|e| warp::reject::custom(RejectionError(e)))
                    })
            )
        )
}

