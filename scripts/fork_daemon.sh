#!/usr/bin/env bash
set -euo pipefail

RPC="${QUICKNODE_RPC:?need QUICKNODE_RPC env var}"
PORT="${ANVIL_PORT:-8545}"
HOST="127.0.0.1"

while true; do
  echo "[anvil] starting fork against $RPC"
  exec anvil \
    --fork-url "$RPC" \
    --fork-block-number latest-2 \
    --no-mining \
    --chain-id 42161 \
    --host $HOST \
    --port $PORT \
    --state-cache-path /mnt/anvil-fork \
    --steps-tracing \
    --gas-price 200000000 \
    --code-size-limit 0 \
    --silent
  echo "[anvil] crashed — respawning in 30 s"
  sleep 30
done
