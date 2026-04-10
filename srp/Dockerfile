FROM rust:1.85

WORKDIR /app
COPY . .

RUN cargo build --release -p srp

CMD ["./target/release/srp" , "-c", "/app/client.toml"]
