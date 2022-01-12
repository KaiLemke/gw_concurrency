use std::io::{self, Write};
use std::time::Duration;
use std::{thread, process};
use futures::stream::SplitStream;
use futures;
use futures_util::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub async fn connect(url: url::Url) -> WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    let (ws, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket connection to server established");
    ws
}

pub async fn response(mut ws_rcv: SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>)
{
    // websocket reading
    while let Some(result) = ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving message: {}", e);
                break;
            }
        };
        let mut data = msg.into_data();
        data.extend_from_slice("\n".as_bytes().as_ref());
        tokio::io::stdout().write(&data).await.unwrap();
    }
}

pub async fn read_stdin(tx: mpsc::Sender<Result<Message, Error>>) {
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
        // let buf = buf.truncate();
        if "quit\n" == buf.to_string() {
            process::exit(0);
        }

        tx.try_send(Ok(Message::text(buf))).unwrap();
    }
}
