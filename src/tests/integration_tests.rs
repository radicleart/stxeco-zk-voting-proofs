use reqwest::Client;
use tokio::task;
use warp::Filter;
use serde_json::json;
use std::net::SocketAddr;

// Re-import your app's main route setup here

#[tokio::test]
async fn test_generate_proof_endpoint() {
    // Define the address for the server
    let addr: SocketAddr = "127.0.0.1:3030".parse().unwrap();

    // Start the Warp server in a background task
    let routes = stacks_routes();  // Load the stacks routes
    task::spawn(async move {
        warp::serve(routes).run(addr).await;
    });

    // Wait briefly for the server to start up
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Define the client and request data
    let client = Client::new();
    let payload = json!({
        "message_inputs": {
            "message": "I vote in favor",
            "vote": "for",
            "proposal": "SIP-028",
            "balance_at_height": 100,
            "burn_start_height": 50,
            "burn_end_height": 60
        },
        "public_key": "02abcd...",
        "hash": "245172c...",
        "signature": "db6eac1...",
        "message": "Some signed message"
    });

    // Make the POST request to the `/stacks/proof/generate` endpoint
    let res = client
        .post("http://127.0.0.1:3030/stacks/proof/generate")
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    // Check that the response is successful and contains the expected status
    assert_eq!(res.status(), 200);

    // Parse the JSON response
    let response_json: serde_json::Value = res.json().await.expect("Invalid JSON response");
    assert_eq!(response_json["status"], "proof generated");
}
