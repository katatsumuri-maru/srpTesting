use tokio::net::TcpListener;
use tokio::io::{copy_bidirectional, AsyncWriteExt};
use tokio::sync::mpsc;
use std::io;
use shared;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = shared::Args::parse_args();

    let config: shared::ServerConfig = shared::parsing::parse_server_config(&args.config);

    let tunnel_listener = TcpListener::bind(&config.server.bind_addr).await?;
    let public_listener = TcpListener::bind("0.0.0.0:443").await?;

    println!("Relay listening on {} (agent) and 443 (public)", &config.server.bind_addr);

    let (tx, mut rx) = mpsc::channel::<tokio::net::TcpStream>(1);

    // Accept agent connection
    tokio::spawn(async move {
        loop {
            let (stream, addr) = tunnel_listener.accept().await.unwrap();
            println!("Agent connected: {}", addr);
            tx.send(stream).await.unwrap();
        }
    });

    loop {
        let (mut inbound, addr) = public_listener.accept().await?;
        println!("Incoming client: {}", addr);

        let mut agent = match rx.recv().await {
            Some(s) => s,
            None => {
                println!("No agent available");
                continue;
            }
        };

        tokio::spawn(async move {
            println!("Forwarding to agent");

            if agent.write_all(b"START\n").await.is_err() {
                println!("Failed to signal agent");
                return;
            }

            let _ = copy_bidirectional(&mut inbound, &mut agent).await;
        });
    }
}
