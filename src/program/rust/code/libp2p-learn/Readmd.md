## relay

cargo run --bin relay_v2 -- --port 4001 --secret-key-seed 0

## Setting up the listening client

RUST_LOG=info cargo run --bin client -- --secret-key-seed 1 --mode listen --relay-address /ip4/$SERVER_IP/tcp/4001/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN

## client0

RUST_LOG=info cargo run --bin client -- --secret-key-seed 2 --mode dial --relay-address /ip4/$SERVER_IP/tcp/4001/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN --remote-peer-id 12D3KooWPjceQrSwdWXPyLLeABRXmuqt69Rg3sBYbU1Nft9HyQ6X
