use tokio::net::TcpStream;
use tokio::io::{copy_bidirectional, AsyncReadExt};
use tokio::net::TcpStream as TokioTcpStream;
use std::io;
use std::env;

#[tokio::main]
async fn main() -> io::Result<()> {
    let relay_addr;
    match env::var("RELAY") {
        Ok(val) => {relay_addr=val}
        Err(_e) => panic!("No relay specified"),
    }

    let endpoint_addr;
    match env::var("ENDPOINT") {
        Ok(val) => {endpoint_addr=val}
        Err(_e) => panic!("No endpoint specified"),
    }

    loop {
        println!("Connecting to relay...");
        let mut tunnel = TcpStream::connect(&relay_addr).await?;

        let mut buffer = [0u8; 6];

        loop {
            if tunnel.read_exact(&mut buffer).await.is_err() {
                println!("Tunnel closed");
                break;
            }

            if &buffer == b"START\n" {
                let mut local = TokioTcpStream::connect(&endpoint_addr).await?;
                let _ = copy_bidirectional(&mut tunnel, &mut local).await;
                break;
            }
        }
    }
}
