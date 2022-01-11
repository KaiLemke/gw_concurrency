use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    #[clap(short, long, default_value = "ws://localhost:8000/opcode")]
    connect_addr: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = url::Url::parse(&args.connect_addr).unwrap();
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    thread::sleep(Duration::from_millis(10));
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        print!("> ");
        io::stdout().flush().unwrap();
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        let buf = String::from_utf8_lossy(&buf);
        tx.unbounded_send(Message::text(buf)).unwrap();
    }
}
