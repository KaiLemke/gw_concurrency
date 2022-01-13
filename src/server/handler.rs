//! Provides handlers if `warp::Filter`s are met

use warp::{Rejection, Reply};

use super::greeting;
use super::ws;
use super::Clients;

/// A `Result` with a `warp::Rejection` as `Err` variant
pub type Result<T> = std::result::Result<T, Rejection>;

/// Handles opcode requests
///
/// Incoming connections are upgraded to `warp::ws::WebSocket`s.
/// The `WebSocket` is handled by `ws::client_connection`.
pub async fn opcode_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("opcode_handler");
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}

/// Serves the `greeting`
pub async fn greeting_handler() -> Result<impl Reply> {
    println!("greeting_handler");
    Ok(greeting())
}
