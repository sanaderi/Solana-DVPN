# Solana dVPN Project

This repository contains two separate projects:

1. **Solana Smart Contracts (solana-program):**
   - Smart contracts for handling payments, node registration, and access control for the decentralized VPN.
   - Located in the `solana-program/` directory.
   - Developed using the [Anchor framework](https://book.anchor-lang.com/).

2. **VPN Client/Server (dvpn-client):**
   - Rust-based client/server for the decentralized VPN service.
   - Located in the `dvpn-client/` directory.

## Getting Started

### Solana Smart Contracts (solana-program)

1. Install Rust, Solana CLI, and Anchor.
2. Build and deploy the smart contract:
   ```bash
   cd solana-program
   anchor build
   anchor deploy
