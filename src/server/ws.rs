//! Manages `WebSocket` connections with clients.

use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use futures::{FutureExt, StreamExt};
use uuid::Uuid;

use super::Client;
use super::Clients;

/// Registers a new client, communicates with it and removes it afterwards.
/// 
/// The actual message handling is done by `client_msg`.
///
/// TODO: Using unbounded channels may be not the best choice, because we may run out of memory.
pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("Established client connection: {:?}", ws);

    // Setup the communication channel.
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    // Keep the stream running.
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
async fn client_msg(client_id: &str, msg: Message, clients: &Clients) -> bool {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return true,
    };

    // Handle the client's wish to unregister.
    if message == "quit" || message == "quit\n" {
        return true;
    }

    let locked = clients.lock().await;
    match locked.get(client_id) {
        Some(v) => {
            if let Some(sender) = &v.sender {
                println!("sending echo message");
                let _ = sender.send(Ok(Message::text(message)));
            }
        }
        None => return true,
    }

    false
}
