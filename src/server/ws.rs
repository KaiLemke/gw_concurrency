//! Manages `WebSocket` connections with clients.

use futures::{FutureExt, StreamExt};
use std::str::FromStr;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use super::Client;
use super::Clients;
use super::CLIENT_CONN_SIZE;
use crate::Command;

/// Registers a new client, communicates with it and removes it afterwards.
///
/// The actual message handling is done by `client_msg`.
pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("Established client connection: {:?}", ws);

    // Setup the communication channel.
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::channel(CLIENT_CONN_SIZE);
    let client_rcv = ReceiverStream::new(client_rcv);

    println!("spawning a new client connection task");
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    // Register the new client.
    let uuid = Uuid::new_v4().to_simple().to_string();
    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };
    clients.lock().await.insert(uuid.clone(), new_client);

    // Process messages arriving from the client.
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        if client_msg(&uuid, msg, &clients).await {
            break;
        }
    }

    // Unregister the client.
    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

/// Handles messages from the client.
///
/// This function currently acts as an echo server except if a 'quit' message arrives.
///
/// # Arguments
/// * `client_id` - The `Client`'s UUID
/// * `msg` - The `Message` the client sendt
/// * `clients` - The shared map list of registered clients
///
/// # Returns
/// Returns `true` if a 'quit' message arrived or an error occurred.
/// This indicates that the we don't expect further messages from the client
/// and it can be removed from `clients`.
///
/// If further messages are expected, `false` is returned.
pub async fn client_msg(client_id: &str, msg: Message, clients: &Clients) -> bool {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return true,
    }
    .trim();
    let cmd = match Command::from_str(message) {
        Ok(cmd) => cmd,
        Err(_) => return true,
    };
    let (messages, quit) = cmd.reply();

    let locked = clients.lock().await;
    match locked.get(client_id) {
        Some(v) => {
            if let Some(sender) = &v.sender {
                for message in messages {
                    println!("sending message: {:?}", message);
                    let _ = sender.send(Ok(message)).await;
                }
            }
            quit
        }
        None => true,
    }
}
