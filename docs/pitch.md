v4[20260508]

# ClearWatch — Pitch Script (Colosseum Frontier)

**Target length:** ≤2:00 (Arena Pitch video cap)
**Delivery rate:** ~145 words per minute
**Word count:** ~290
**Format:** spoken; founder narrative video, separate from the demo video.

---

## (0:00–0:20) Self-introduction

Hi, I'm Akky. I've spent the past few years as a crypto analyst based in Japan, focused mostly on Solana and the emerging AI agent economy.

## (0:20–0:55) What I'm building

What I'm building is ClearWatch — an on-chain compliance primitive for a gap I've been watching go unsolved. AI agents are about to move serious value on-chain at machine speed, and there's no audit trail infrastructure underneath them. ClearWatch is two parts: a public stake-secured risk registry anyone can flag and read for free, plus a paid API where AI agents call check_and_prove before each transaction and write a cryptographic proof on-chain.

The frontend at clearwatch.pages.dev is the human side — victims reporting flagged addresses, builders verifying entries. AI agents integrate programmatically at the on-chain program level.

## (0:55–1:30) Why me

Why me? Across those years, the same pattern kept showing up. When something goes wrong — phishing drains, rug pulls, dust attacks — recovery mobilizes for the big losses. Everyday users get left behind. Every time. I came to see this as structural failure, not bad luck. And as AI agents transact at machine speed in small amounts, the gap multiplies by orders of magnitude. That's why I'm shipping this now.

## (1:30–1:50) On scope

What's submitted is two parts running on Solana devnet today: the registry and the API. The roadmap items — sanctions data ingestion, transitive risk propagation, ecosystem integration, privacy via Arcium — are documented but not built. I'd rather be honest than over-claim.

## (1:50–2:00) The bet

The bet: if the API succeeds, this is compliance infrastructure for the agent economy. If it doesn't, the registry stays as on-chain public infrastructure for the next builder. No path where this is wasted.

You can verify it on devnet at clearwatch.pages.dev. That's my submission.

---

## Delivery notes

- Pause two beats after each section heading. The structure carries the argument; let it breathe.
- "Why me" is the emotional center. Slow down on "phishing drains, rug pulls, dust attacks" — let each item land. "Every time" is the hook, point it.
- "I came to see this as structural failure, not bad luck" — deliberate tone shift, observation to conviction.
- "The gap multiplies by orders of magnitude" — alarm in the voice.
- Closing public URL is the call to action — don't rush.
- Total target: 1:55-2:00. The 2:00 cap is hard; over by 1 second triggers Arena auto-trim.

## What this pitch deliberately avoids

- Detailed Arcium privacy claims (Designed, not yet built).
- "Status: not implemented" verbiage out loud.
- "Integration" framing for sponsor skills.
- x402 framing.
- Frameworkless HTML defense (Q&A only).
- "Compliance solution" framing — ClearWatch is *infrastructure for compliance verification*.
- The word "layer" — in a Solana pitch, "Layer 1 / Layer 2" reads as blockchain L1/L2.
- Named incumbent AML providers — abstracted as "the established AML providers" / "the AML stack".
- Specific named integration partners — generic "any DeFi protocol or bridge" if mentioned at all.

## Optional addition (5/8 stretch, if recording with margin)

Insert at end of "What I'm building" section (~25 words, fits 2:00 cap):

  "Agents don't just consume the registry. When an agent is compromised, its runtime can flag the perpetrator using the same primitive — every other agent gets the warning in the next block."

This makes ClearWatch a "protocol agents participate in" rather than "a service agents call." Strong narrative but only include if take is clean and time permits.

## Subtitle hit list

Tech terms (Premiere Auto-Transcribe校正):
- ClearWatch (one word, capital W)
- Akky (proper noun)
- check_and_prove
- InnocenceProof
- Solana / Anchor / Phantom / Arcium
- DeFi / AML / OFAC / KYC / SOL / PDA
- "phishing drains" (often "fishing")
- "rug pulls"
- "dust attacks"
- "structural failure"

Removed from previous versions (no longer in script):
- Chainalysis / Jupiter / Wormhole
- Layer 1 / Layer 2 / Layer 3
- Sub-second finality / fractional-cent
- "outside the U.S. compliance industry"
