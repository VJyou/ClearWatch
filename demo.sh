#!/bin/bash
# ClearWatch one-command demo setup.
# Starts a local validator, deploys the program, serves the UI on :8000.
set -e

source ~/.cargo/env 2>/dev/null || true
export PATH="$HOME/.local/bin:/home/vportal/.local/share/solana/install/active_release/bin:$PATH"

PROGRAM_SO="target/deploy/clearwatch.so"
PROGRAM_KP="target/deploy/clearwatch-keypair.json"
PROGRAM_ID="FDMGN1Gp62gK1TAnVvq2DM4HV6BhFwJ9Me5djLVKEKgB"

if [ ! -f "$PROGRAM_SO" ]; then
  echo "[demo] $PROGRAM_SO not found. Run: cargo build-sbf"
  exit 1
fi

echo "[demo] Switching CLI to localhost"
solana config set --url localhost > /dev/null

# Start validator if not already running
if ! pgrep -f solana-test-validator > /dev/null; then
  echo "[demo] Starting solana-test-validator (logs: /tmp/validator.log)"
  rm -rf test-ledger
  nohup solana-test-validator --quiet > /tmp/validator.log 2>&1 &
  sleep 5
else
  echo "[demo] Validator already running"
fi

# Wait for RPC to respond
for i in 1 2 3 4 5; do
  if solana cluster-version > /dev/null 2>&1; then break; fi
  sleep 1
done

# Deploy if program account is missing
if ! solana program show "$PROGRAM_ID" > /dev/null 2>&1; then
  echo "[demo] Deploying program"
  solana program deploy "$PROGRAM_SO" --program-id "$PROGRAM_KP" | tail -2
else
  echo "[demo] Program $PROGRAM_ID already deployed"
fi

WALLET=$(solana address)
BALANCE=$(solana balance | awk '{print $1}')
echo "[demo] Wallet $WALLET balance: $BALANCE SOL"

echo ""
echo "[demo] ===================================================="
echo "[demo]   Open http://localhost:8000/clearwatch.html"
echo "[demo]   Click LOAD KEYPAIR → ~/.config/solana/id.json"
echo "[demo]   Click DEMO to pre-fill addresses, then submit."
echo "[demo]   Ctrl-C here to stop the HTTP server."
echo "[demo] ===================================================="
echo ""

cd "$(dirname "$0")"
python3 -m http.server 8000
