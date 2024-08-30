use futures_util::stream::StreamExt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::error;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use crate::db_client::ResultOutputData;

// Struct to hold WebSocket clients  
pub struct WebsocketConnection {
    pub clients: HashMap<Uuid, tokio::sync::mpsc::UnboundedSender<Result<Message, warp::Error>>>,
}
impl WebsocketConnection {
    pub fn new() -> Self {
        WebsocketConnection {
            clients: HashMap::new(),
        }
    }
    /// Broadcasts message to all connected WebSocket clients  
    pub fn broadcast_message(&self, output_data: &ResultOutputData) {
        let message = warp::ws::Message::text(output_data.result.clone());
        for (_, tx) in self.clients.iter() {
            let _ = tx.send(Ok(message.clone()));
        }
    }
    pub fn add_client(
        &mut self,
        id: Uuid,
        tx: tokio::sync::mpsc::UnboundedSender<Result<Message, warp::Error>>,
    ) {
        self.clients.insert(id, tx);
    }
    pub fn remove_client(&mut self, client_id: &Uuid) {
        self.clients.remove(client_id);
    }
}

/// Handles individual WebSocket client connections  
pub async fn handle_client(ws: WebSocket, clients: Arc<Mutex<WebsocketConnection>>) {
    // Split the WebSocket stream into sender and receiver
    let (tx, mut rx) = ws.split();
    let (client_ws_tx, client_ws_rx) = tokio::sync::mpsc::unbounded_channel();
    let client_ws_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(client_ws_rx);

    // Spawn task to forward messages from channel to WebSocket
    tokio::task::spawn(client_ws_rx.forward(tx));

    // Generate a unique client ID and store the sender in the clients map
    let client_id = Uuid::new_v4();
    clients.lock().unwrap().add_client(client_id, client_ws_tx);

    // Process incoming WebSocket messages
    while let Some(result) = rx.next().await {
        if let Err(e) = result {
            error!("WebSocket error: {:?}", e);
            break;
        }
    }

    // Remove client from map on disconnect
    clients.lock().unwrap().remove_client(&client_id);
}

/// Sets up WebSocket route  
pub fn ws_route(
    clients: Arc<Mutex<WebsocketConnection>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::path("result"))
        .and(warp::ws())
        .and(with_clients(clients))
        .map(|ws: warp::ws::Ws, clients| {
            ws.on_upgrade(move |socket| handle_client(socket, clients))
        })
}

/// Clones the clients map for moving into filters  
fn with_clients(
    clients: Arc<Mutex<WebsocketConnection>>,
) -> impl Filter<Extract = (Arc<Mutex<WebsocketConnection>>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || clients.clone())
}
