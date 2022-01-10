use std::{net::TcpListener, process};

use opcode::server::handle_connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap_or_else(|e| {
        eprintln!("Could not start server: {:#?}", e);
        process::exit(1)
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap_or_else(|e| {
            eprintln!("Could not get stream: {:#?}", e);
            process::exit(1);
        });
        handle_connection(stream).unwrap_or_else(|e| {
            eprintln!("Reading the stream failed: {:#?}", e);
            process::exit(1);
        });
    }
}
