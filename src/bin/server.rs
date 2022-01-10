use warp::Filter;

use opcode::server::handler::opcode_handler;

#[tokio::main]
async fn main() {
    println!("Configuring websocket route");
    let opcode_route = warp::path("opcode").and(warp::ws()).and_then(opcode_handler);

    let routes = opcode_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
