use futures::FutureExt;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use clap::Parser;

use opcode::client::{Args, ws};
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = url::Url::parse(&args.connect_addr).unwrap();

    let ws = ws::connect(url).await;
    let (ws_sender, ws_rcv) = ws.split();
    let (channel_sender, channel_rcv) = mpsc::channel(1);
    let client_rcv = ReceiverStream::new(channel_rcv);
 
    tokio::task::spawn(client_rcv.forward(ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending message: {}", e);
        }
    }));
    
    tokio::spawn(ws::read_stdin(channel_sender));
    ws::response(ws_rcv).await;
}
