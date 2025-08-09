# Flash-Arb Bot

A high-performance flash loan arbitrage bot for Arbitrum One, built with Rust and Solidity.

## Project Structure

```bash
.
├── contracts/     # Solidity & Foundry tests
├── rust/          # Cargo workspace crates
│   └── executor/  # Main bot executable
├── ops/           # Production tooling
├── infra/         # Infrastructure config (systemd, etc)
├── docs/          # Documentation
├── scripts/       # Helper scripts
└── config/        # Bot configuration
```

## Infrastructure Setup

### Local Development

#### Prerequisites

- git (≥ 2.40)
- rustup (1.79.0)
- foundry (nightly-2025-07-30)
- node.js (20.16.0)
- just (1.27.0)
- Docker & Docker Compose

Run `just env` to verify your environment is properly configured.

### Running the Stack

1. **Set your RPC URL**:

   ```bash
   export QUICKNODE_RPC="https://arb-mainnet.g.alchemy.com/v2/YOUR_API_KEY"
   ```

2. **Start the Docker Compose stack**:

   ```bash
   docker compose up -d --build
   ```

3. **View logs**:

   ```bash
   docker compose logs -f bot
   ```

### Key Components

#### Anvil Fork

We use Anvil to create a local fork of Arbitrum that's always 2 blocks behind the latest block. The fork daemon automatically respawns if it crashes.

- **Performance optimization**: State is stored in RAM using tmpfs for 10x faster access
- **Production-ready**: Systemd service for automatic startup and crash recovery

#### Hot-Reload Configuration

The bot watches its configuration files and reloads without downtime when changes are detected:

```bash
# After editing config/bot.toml
docker compose exec bot kill -HUP 1
```

## Verification Checklist

| Component | Verification Command | Expected Result |
|-----------|---------------------|-----------------|  
| Anvil Fork | `curl -s -X POST localhost:8545 -d '{"jsonrpc":"2.0","method":"eth_blockNumber","id":1}'` | Valid JSON with hex block number |
| tmpfs Mount | `docker compose exec anvil df -h /mnt/anvil-fork` | Shows tmpfs, 512MB |
| Bot Health | `curl localhost:9000/health` | `{"status":"ok"}` |
| Hot-reload | Edit config/bot.toml then `docker compose exec bot kill -HUP 1` | Logs show "config changed – reloading" |
