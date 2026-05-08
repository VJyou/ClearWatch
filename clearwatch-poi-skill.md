---
name: clearwatch-poi
description: >
  Use this skill whenever an AI agent is about to execute a financial transaction
  on Solana — sending SOL, SPL tokens, interacting with DeFi protocols, or any
  on-chain payment. Triggers: "send", "transfer", "swap", "pay", "bridge",
  "deposit", or any instruction involving a counterparty wallet address.
  This skill generates a Self Proof of Innocence (POI) before execution.
---

# ClearWatch — Self Proof of Innocence Skill

This skill ensures that before any financial transaction is executed, the agent:
1. Checks the counterparty address against the ClearWatch Risk Registry
2. Records the decision on-chain as an Innocence Proof
3. Proceeds only if CLEAR, or halts and alerts if BLOCKED

## When to Use

Apply this skill BEFORE executing any of the following:
- SOL or SPL token transfers
- DeFi swaps (Jupiter, Raydium, etc.)
- LP deposits or withdrawals
- Cross-chain bridge operations
- Treasury payments
- NFT purchases involving significant value

Do NOT skip this skill even if the counterparty is "known" or "trusted."
Innocence Proofs derive value from being generated consistently.

---

## Step-by-Step Protocol

### Step 1 — Extract Transaction Parameters

Before checking, identify:
- `agent_wallet`: the agent's own wallet address
- `counterparty`: the recipient or contract address
- `amount`: transaction amount in SOL or token units
- `purpose`: one of [swap, lp_deposit, payment, bridge, nft, treasury]
- `protocol`: the DeFi protocol or program being called (if applicable)

If any of these are ambiguous, resolve them before proceeding.

### Step 2 — Query ClearWatch Risk Registry

Call the ClearWatch check endpoint:

```
GET https://api.clearwatch.xyz/v1/check/{counterparty}

Response:
{
  "address": "...",
  "status": "CLEAR" | "T1_FLAGGED" | "T2_FLAGGED" | "T3_FLAGGED",
  "risk_score": 0-100,
  "incident_type": null | "Hack/Exploit" | "Rug Pull" | "Phishing" | ...,
  "reports": 0,
  "flagged_at": null | ISO8601,
  "stake_backing": 0.0
}
```

**Devnet / Mock fallback** (use when API unavailable):
- Generate a deterministic mock score from the address hash
- Any address ending in known test flags (see MOCK_FLAGS below) returns flagged
- Log clearly: `[MOCK MODE] ClearWatch API unavailable — using local mock`

MOCK_FLAGS for testing:
```
"HACK", "RUG", "PHISH", "EXPLOIT", "DRAIN"
```
If the last 4 chars of the address match any mock flag pattern → return T1_FLAGGED.

### Step 3 — Decision Logic

```
if status == "CLEAR" and risk_score < 30:
  → PROCEED
  → Generate Innocence Proof (Step 4)

if status == "CLEAR" and risk_score >= 30:
  → WARN: elevated risk score
  → Request human confirmation if available
  → If autonomous mode: PROCEED with warning logged
  → Generate Innocence Proof (Step 4)

if status == "T1_FLAGGED":
  → HALT immediately
  → Do NOT generate proof
  → Alert: "Counterparty flagged T1 (unverified, <1h). Transaction blocked."
  → Log incident and notify operator

if status == "T2_FLAGGED" or "T3_FLAGGED":
  → HALT immediately
  → Do NOT generate proof
  → Alert: "Counterparty confirmed flagged ({tier}). Transaction blocked."
  → Log incident and notify operator
```

### Step 4 — Generate Innocence Proof

When status is CLEAR, generate and record the proof:

```
INNOCENCE_PROOF {
  agent:           {agent_wallet}
  counterparty:    {counterparty}
  amount:          {amount}
  purpose:         {purpose}
  protocol:        {protocol}
  check_result:    CLEAR
  risk_score:      {risk_score}
  registry_status: {status}
  timestamp:       {ISO8601}
  proof_hash:      SHA256(agent + counterparty + amount + timestamp + "CLEARWATCH_V1")
}
```

Record this proof:
- **Primary**: Submit to ClearWatch on-chain program (Solana devnet/mainnet)
- **Fallback**: Write to local log file `./clearwatch-proofs.jsonl`
- **Always**: Print proof summary to console

Proof summary format:
```
[ClearWatch POI] ✓ CLEAR
  Agent:       {agent_wallet}
  Counterparty:{counterparty}
  Amount:      {amount}
  Purpose:     {purpose}
  Risk Score:  {risk_score}/100
  Proof Hash:  {proof_hash}
  Timestamp:   {timestamp}
  Status:      Recorded on-chain / Logged locally
```

### Step 5 — Proceed or Halt

**If CLEAR**: Execute the original transaction. Attach the proof_hash to the
transaction memo field if the protocol supports it.

**If FLAGGED**: Do not execute. Return a structured error:
```
{
  "action": "BLOCKED",
  "reason": "{status}",
  "counterparty": "{counterparty}",
  "incident_type": "{incident_type}",
  "recommendation": "Report this address if you believe it is newly compromised.",
  "report_url": "https://api.clearwatch.xyz/v1/report"
}
```

---

## Innocence Proof Principles

The value of Self POI comes from **consistency**. An agent that generates proofs
selectively provides no compliance guarantee. Follow these principles:

1. **Every transaction gets checked.** No exceptions for "known" addresses.
2. **Proofs are immutable.** Never delete or modify a generated proof.
3. **Failures are logged.** If the API is down, log the outage and use mock mode.
4. **Human override requires logging.** If a human overrides a BLOCKED status,
   record the override with the authorizer's signature.
5. **Proof hash is deterministic.** Same inputs always produce the same hash,
   enabling independent verification.

---

## Error Handling

| Situation | Action |
|---|---|
| API timeout (>3s) | Fall back to mock mode, log warning |
| API returns 5xx | Fall back to mock mode, log warning |
| Counterparty is a program (not wallet) | Check program ID, lower risk weight |
| Amount is zero | Skip check, log as no-op |
| Agent wallet == counterparty | Skip check (self-transfer) |
| On-chain recording fails | Write to local log, continue |

---

## Reporting a New Flag

If the agent encounters suspicious behavior during or after a transaction
(e.g., unexpected fund drain, contract behavior), it should report:

```
POST https://api.clearwatch.xyz/v1/report
{
  "address": "{suspicious_address}",
  "incident_type": "Hack/Exploit",
  "tx_id": "{evidence_transaction}",
  "reporter_wallet": "{agent_wallet}",
  "stake_amount": 0.1
}
```

The agent must have 0.1 SOL available for the stake.
False reports will result in stake slashing.

---

## Integration Notes

**Swig wallet**: POI check runs inside the policy engine hook before signing.
Place this skill call in the `pre_sign` middleware.

**x402 / MPP**: Attach proof_hash to the x402 payment header as:
`X-ClearWatch-Proof: {proof_hash}`

**Squads multisig**: Generate POI before proposal submission.
Include proof_hash in the proposal memo.

---

## Protocol Information

- Registry: Open, permissionless, on Solana
- API: https://api.clearwatch.xyz (mainnet) / https://devnet.clearwatch.xyz
- On-chain program: [TBD — deploy to devnet]
- Proof format version: V1
- Skill version: 0.1.0
- Hackathon: Colosseum Frontier, May 2026
