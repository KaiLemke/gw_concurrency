use warp::{Reply, ws::WebSocket, Rejection};

pub const GREETING: &str = r#"Welcome to opcode server!

You can send me an intcode, i.e. a list of integers like '(1,0,0,3,99)'.

Index 0 is an opcode of the following:
    -  1 - add     : Adds together numbers read from two positions and stores a result in a third position.
    -  2 - multiply: Does the same as 1 but with multiplication.
    - 99 - exit    : Exits the program, i.e. closes the connection immediately.

If no exit opcode is sent, I will accept further opcodes.
"#;

pub type Result<T> = std::result::Result<T, Rejection>;

pub async fn opcode_handler(ws: warp::ws::Ws) -> Result<impl Reply> {
    println!("opcode_handler");
    Ok(ws.on_upgrade(move |socket| client_connection(socket)))
}

pub async fn client_connection(ws: WebSocket) {
    println!("Established client connection: {:?}", ws);
}