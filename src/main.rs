use proofs::handle_message;
use std::{
    collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::{self, protocol::Message};

mod proofs;
pub mod stacks;

type IoResult<T> = std::io::Result<T>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>;
//type PeerMap = Arc<Mutex<HashMap<SocketAddr, tokio::sync::mpsc::UnboundedSender<tungstenite::Message>>>>;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = "127.0.0.1:9001".to_string();
    let ws_state = PeerMap::new(Mutex::new(HashMap::new()));

    // Set up WebSocket listener (as before)
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind WebSocket listener");
    println!("WebSocket server listening on: {}", addr);

    // Initialize routes with the shared state
    let _http_state = ws_state.clone();
    let routes = stacks::stacks_routes();

    // Start the Warp server for HTTP endpoints concurrently with the WebSocket server
    tokio::select! {
        _ = run_websocket_server(listener, ws_state.clone()) => {},
        _ = warp::serve(routes).run(([127, 0, 0, 1], 3030)) => {},
    }

    Ok(())
}


// WebSocket server function to handle incoming connections
async fn run_websocket_server(listener: TcpListener, state: PeerMap) -> IoResult<()> {
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_ws_connection(state.clone(), stream, addr));
    }
    Ok(())
}

async fn handle_ws_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, _rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (_outgoing, incoming) = ws_stream.split();
    //let outgoing = Arc::new(Mutex::new(outgoing));  // Wrap `outgoing` in Arc<Mutex<...>> here

    // Process incoming messages and respond asynchronously
    let broadcast_incoming = incoming.try_for_each(|msg| {
        let msg_text = msg.to_text().unwrap().to_string();
        //let peer_map_clone = peer_map.clone();
        //let outgoing = Arc::clone(&outgoing);  // Clone the Arc for each message

        async move {
            // Handle the message asynchronously
            match handle_message(&msg_text).await {
                Ok(response_message) => {
                    // Lock outgoing for this async block
                    //let mut outgoing = outgoing.lock();
                    // Send the response back to the client
                    //outgoing.send(Message::Text(response_message)).await?;
                    eprintln!("response_message: {:?}", response_message);
                }
                Err(e) => {
                    eprintln!("Error handling message: {:?}", e);
                }
            }

            // Explicitly annotate the return type to satisfy try_for_each
            Ok::<(), tungstenite::Error>(())
        }
    });

    broadcast_incoming.await.expect("Error processing messages");
}
