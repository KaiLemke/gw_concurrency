//! OpCode Server

use std::{collections::HashMap, convert::Infallible, result::Result, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter};

pub mod handler;
pub mod ws;

/// Game instructions
pub const HELP: &str = r#"Welcome to opcode server!

You can send me an intcode, i.e. a list of integers like '1,0,0,3,2,0,3,6,99'.

To do that you, please connect to ws://127.0.0.1:8000/opcode
and I will give you back the modified opcode.

Index 0 is an opcode of the following:
    -  1 - add     : Adds together numbers read from two positions
                     and stores a result in a third position.
    -  2 - multiply: Does the same as 1 but with multiplication.
    - 99 - exit    : Exits the program, i.e. closes the connection immediately.

Multiple opcodes can be sent in one intcode.

If no exit opcode is sent, I will accept further opcodes.
"#;

/// A Client
#[derive(Debug, Clone)]
pub struct Client {
    /// The client's UUID
    pub client_id: String,
    /// Communication channel sender end
    pub sender: Option<mpsc::UnboundedSender<Result<Message, warp::Error>>>,
}

/// A list of registered clients keyed by UUIDs
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;

/// `warp::Filter` for handling clients
pub fn with_clients(
    clients: Clients,
) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
