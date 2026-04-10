# rly

## Idea

Expose self-hosted services to the internet via an external relay (hosted on a VPS).

- Expose anything from a basic HTTP endpoint to a game severs using UDP
- Fully transparent relay server -> TLS connections only terminate at the agents
- Automatic TLS certificates
- Easy configuration and deployment via Docker or Linux binary
- Minimal resources required by relay server
- Monitor service health
- Build open source with self-hosting at heart

## Architecture

client <-> relay <-> agent <-> your service

## Tech Stack

- Rust

## Deploy yourself

## Contribute

## Related projects

- https://github.com/rathole-org/rathole
- https://github.com/fosrl/pangolin
- https://github.com/fatedier/frp
