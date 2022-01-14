//! OpCode Server

use std::{collections::HashMap, result::Result, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::ws::Message;

use crate::command::HELP;

pub mod filter;
pub mod handler;
pub mod ws;

/// The buffer size for `mpsc::channel`s for client connections
pub const CLIENT_CONN_SIZE: usize = 42;

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
