use std::io;
use std::io::Read;
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer)?;

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    Ok(())
}
