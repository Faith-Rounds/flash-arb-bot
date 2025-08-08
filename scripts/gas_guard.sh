#!/usr/bin/env bash
set -eo pipefail
forge test --gas-report | tee /tmp/gas.txt
if grep -E 'SLOAD:[[:space:]]+([2-9][5-9][1-9]|[3-9][0-9]{2,})' /tmp/gas.txt; then
  echo "❌ Gas guard breached (>250 SLOAD)"
  exit 1
else
  echo "✅ Gas usage within limits"
fi
