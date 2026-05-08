v2[20260508]

# ClearWatch — Pitch Script (Colosseum Frontier)

**Target length:** 2:30–2:45 (3-minute hard cap)
**Delivery rate:** ~145 words per minute
**Word count:** ~386
**Format:** spoken; no slides required, but section headers map to recommended on-screen overlays.

---

## (0:00–0:35) Problem — the AI agent compliance gap

Every week, millions are stolen in DeFi exploits. Investigations follow days later. Blacklist updates land days after that.

The entire AML stack — sanctions screening, transaction monitoring, KYC — was built around humans moving at human speed. AI agents transact in milliseconds. When an agent sends funds to a compromised address, the company running that agent owns the liability — and there's no audit trail proving the agent followed any rule, because no rule was ever attached to it.

## (0:35–1:05) Two-part architecture

ClearWatch is two parts.

The first is a public, stake-secured risk registry on Solana — anyone can flag a compromised address with a 0.1 SOL stake, anyone can read free of charge.

The second is Self Proof of Innocence — a paid API that reads the registry and writes a cryptographic Innocence Proof on-chain before each agent transaction. Sub-second finality and fractional-cent proof writes mean per-transaction compliance doesn't break the agent's unit economics. The enterprise gets the audit trail.

## (1:05–1:25) The asymmetric bet

This is an asymmetric bet.

If the API succeeds, ClearWatch becomes the compliance backbone for the entire AI agent economy.

If it fails — the registry is still there. Open. Permanent. On-chain. The infrastructure exists either way. That's the point.

## (1:25–2:15) Where this goes next

The registry is designed to ingest authoritative sources — OFAC, sanctions lists, public exploit databases — as Tier-3 entries with source attribution. A Risk Graph handles transitive flagging when exploits fan out funds across multiple wallets. We're not replacing the established AML providers; we're filling the structural gap where smart contracts need a compliance primitive they can compose with, not an API contract they have to negotiate.

Today AI agents call check_and_prove. Tomorrow, Jupiter, Wormhole, and Phantom can CPI into the same primitive. The InnocenceProof primitive is composable. We're not building a service for one user category — we're building a primitive that becomes infrastructure when the agent economy and the human economy converge on Solana.

## (2:15–2:40) Public Goods close

Most compliance infrastructure today is owned. ClearWatch is owned by no one.

It's a public good for an economy that doesn't yet exist — one where every company asks a question humans never had to:

"How do I prove my agent followed the rules — when no human was watching?"

ClearWatch answers that question. On Solana. Today.

---

## Delivery notes

- Pause two beats after each section heading. The structure carries the argument; let it breathe.
- The line "owned by no one" should land hard — slow it down and drop pitch.
- The closing question is the hook. Read it like a question, not a recital.
- "Where this goes next" is the only forward-looking section — keep tense consistent. "is designed to" / "handles" / "tomorrow" frame everything as roadmap; don't slip into past or perfect tense and don't add long "Status: ..." caveats. The README carries the explicit status lines.
- Avoid filler ("you know", "kind of"). Each sentence is loadbearing at this density.
- If a take runs long, cut from the Problem section first — the second sentence is the most compressible.
- "two parts" not "two layers" — never use "Layer 1 / Layer 2" out loud. In a Solana pitch the term collides with blockchain L1/L2 terminology.

## What this pitch deliberately avoids

- Detailed Arcium privacy claims (status: Designed, not yet built — not in the verbal pitch beyond what's already on-screen via the README).
- "Status: not implemented in this hackathon submission" verbiage — too long for spoken delivery; the README carries the explicit status lines.
- Any "integration" framing for sponsor skills (they accelerate development; they are not protocol integrations).
- x402 framing (if added as a stretch goal, it stays in the README/demo, not in this pitch).
- Frameworkless HTML defense (only mention if asked in Q&A — frame as auditability, not a corner cut).
- "Compliance solution" framing — ClearWatch is *infrastructure for compliance verification*, not a compliance solution. The latter creates regulatory liability we're not in business to accept.
- **The word "layer"** — in a Solana pitch, "Layer 1 / Layer 2" reads as L1/L2 blockchain terminology and creates ambiguity. Spoken delivery uses "part", "side", or function-named description ("the registry", "the API", "the Risk Graph"). The README still uses the layer-numbering convention internally; the verbal pitch does not.
- **Named incumbent AML providers** — Chainalysis, TRM, Elliptic, and similar are deliberately abstracted as "the established AML providers" / "the AML stack" in spoken delivery. The structural-gap framing is the message; naming the incumbents reframes the relationship as competitive when it isn't.

## Arena category mapping

- Primary: **Agents + Tokenization**
- Public Goods Award: yes (the closing argument is built for this)
- Privacy + Confidential Compute: **do not select** until Arcium circuit is shipped
