use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;
use warp::Filter;

use opcode::server::{handler::opcode_handler, with_clients, Clients};

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    println!("Configuring websocket route");
    let opcode_route = warp::path("opcode")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(opcode_handler);

    let routes = opcode_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
