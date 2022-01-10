use warp::{Rejection, Reply};

use super::ws;
use super::Clients;

pub type Result<T> = std::result::Result<T, Rejection>;

pub async fn opcode_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("opcode_handler");
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}
