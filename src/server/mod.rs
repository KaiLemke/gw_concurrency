//! OpCode Server

use std::{collections::HashMap, convert::Infallible, result::Result, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter};

pub mod handler;
pub mod ws;

/// The buffer size for `mpsc::channel`s for client connections
pub const CLIENT_CONN_SIZE: usize = 42;

/// Game instructions
// We have to serve it as slice because web socket messages are whitespace trimed
// and we cannot use map in `ws::client_msg()`.
pub const HELP: [&str; 7] = [
    "You can send me an intcode, i.e. a list of integers like '1,0,0,3,2,0,3,6,99'.",
    "Index 0 is an opcode of the following: ",
    "-  1 - add     : Adds together numbers read from two positions and stores a result in a third position.",
    "-  2 - multiply: Does the same as 1 but with multiplication.",
    "- 99 - exit    : Exits the program, i.e. closes the connection immediately.",
    "Multiple opcodes can be sent in one intcode.",
    "If no exit opcode is sent, I will accept further opcodes."
];

/// The servers greeting
pub fn greeting() -> String {
    format!(
        "Welcome to opcode server!\n\n{}\n\n{}\n\n{}\n{}\n{}\n{}\n{}\n\n{}\n",
        HELP[0],
        "To do that you, please connect to ws://127.0.0.1:8000/opcode and I will give you back the modified opcode.",
        HELP[1], HELP[2], HELP[3], HELP[4], HELP[5], HELP[6]
    )
}

/// A Client
#[derive(Debug, Clone)]
pub struct Client {
    /// The client's UUID
    pub client_id: String,
    /// Communication channel sender end
    pub sender: Option<mpsc::Sender<Result<Message, warp::Error>>>,
}

/// A list of registered clients keyed by UUIDs
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;

/// `warp::Filter` for handling clients
pub fn with_clients(
    clients: Clients,
) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
