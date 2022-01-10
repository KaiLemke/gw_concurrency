use warp::ws::WebSocket;

pub async fn client_connection(ws: WebSocket) {
    println!("Established client connection: {:?}", ws);
}
