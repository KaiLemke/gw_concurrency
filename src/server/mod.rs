//! OpCode Server

use std::{collections::HashMap, convert::Infallible, result::Result, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter};

pub mod handler;
pub mod ws;

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
