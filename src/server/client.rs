//! Tracking clients

use std::{collections::HashMap, result::Result, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::ws::Message;

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
