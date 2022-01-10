use warp::Filter;

use opcode::server::GREETING;

#[tokio::main]
async fn main() {
    let greeting = warp::path!().map(|| GREETING);

    warp::serve(greeting).run(([127, 0, 0, 1], 8000)).await;
}
