use proofs::handle_message;
use std::{
    collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

mod proofs;
pub mod stacks;

type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type IoResult<T> = std::io::Result<T>;


#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = "127.0.0.1:9001".to_string();
    let ws_state = PeerMap::new(Mutex::new(HashMap::new()));

    // Set up WebSocket listener (as before)
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind WebSocket listener");
    println!("WebSocket server listening on: {}", addr);

    // Initialize routes with the shared state
    let http_state = ws_state.clone();
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

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        let msg_text = msg.to_text().unwrap();

        let response: Result<proofs::ApplicationResponseMessage, proofs::Error> = handle_message(msg_text);
        let response_message:String;

        // Now match on the `response` to handle both Ok and Err cases
        match response {
            Ok(success_response) => {
                // Handle the successful response
                response_message = serde_json::to_string(&success_response).unwrap()
            }
            Err(e) => {
                response_message = e.to_string();
            }
        }
        
        let peers = peer_map.lock().unwrap();
        let ws_message = Message::Text(response_message);

        // Send the proof and result back to the client
        for (_, ws_sink) in peers.iter() {
            ws_sink.unbounded_send(ws_message.clone()).unwrap();
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}
