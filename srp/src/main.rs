use tokio::net::TcpStream;
use tokio::io::{copy_bidirectional, AsyncReadExt};
use tokio::net::TcpStream as TokioTcpStream;
use std::io;
use shared;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = shared::Args::parse_args();

    let config: shared::ClientConfig = shared::parsing::parse_client_config(&args.config);

    loop {
        println!("Connecting to relay...");
        let mut tunnel = TcpStream::connect(&config.client.remote_addr).await?;

        let mut buffer = [0u8; 6];

        loop {
            if tunnel.read_exact(&mut buffer).await.is_err() {
                println!("Tunnel closed");
                break;
            }

            if &buffer == b"START\n" {
                let mut local = TokioTcpStream::connect(&config.client.endpoint_addr).await?;
                let _ = copy_bidirectional(&mut tunnel, &mut local).await;
                break;
            }
        }
    }
}
