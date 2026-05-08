v1[20260508]

# ClearWatch — Tech Walkthrough Script (Colosseum Frontier)

**Target length:** 2:00–2:30 (3-minute hard cap)
**Delivery rate:** ~145 words per minute
**Word count:** ~285 spoken (leaves ~30s of silent on-screen action)
**Format:** screen recording with narration; cuts between IDE / Solana Explorer / browser console.

The walkthrough has one job: prove that the on-chain `proof_hash` is reproducible from its inputs. Everything else (program structure, PDA seeds) is scaffolding for that single demonstration.

---

## (0:00–0:15) Anchor program structure

[Screen: `programs/clearwatch/src/lib.rs` in editor, four `#[program]` entry points visible.]

ClearWatch is one Anchor program on Solana devnet. Four instructions: `report_address`, `check_and_prove`, `upgrade_tier`, `slash_reporter`. Two account types: `RiskEntry` and `InnocenceProof`. That's the whole program surface.

## (0:15–0:35) PDA seed design

[Screen: editor showing the seeds in `instructions/check_and_prove.rs` lines 13 and 22.]

Each address gets at most one `RiskEntry` PDA, derived from the seeds `"risk_entry"` plus the flagged pubkey. Anyone reading the registry computes the same PDA from the same address.

The `InnocenceProof` seed is `"innocence_proof"` plus the agent pubkey plus the counterparty pubkey. Every agent–counterparty pair has exactly one proof slot. A second check overwrites the first.

## (0:35–1:00) The deterministic commit

[Screen: editor scrolled to `compute_proof_hash` in `check_and_prove.rs:108-124`. Cursor on the `extend_from_slice` lines.]

The `proof_hash` is the verifiability primitive. Inside `check_and_prove`, the program concatenates six values — agent pubkey, counterparty pubkey, amount as little-endian u64, purpose hash, `is_clear` as a single byte, timestamp as little-endian i64 — and writes the SHA-256 over that one-hundred-and-thirteen-byte buffer into the PDA.

Order is fixed. Encoding is fixed. Same inputs, same hash. Always.

## (1:00–2:10) Verifying it on-chain

[Screen: Solana Explorer, cluster=devnet, InnocenceProof PDA `7rmV5Pn1oF9pjV5XQDmYQ3tcGomagm9XAAMwsDP3EyoP` loaded. Anchor decoder rendered.]

This is the `InnocenceProof` PDA from a CLEAR check we wrote earlier. Devnet cluster. The Anchor decoder unpacks the fields — agent, counterparty, amount, purpose hash, `is_clear` true, tier-at-check zero meaning no risk entry, timestamp, and the `proof_hash` at the bottom. Thirty-two bytes.

[Cursor on the proof_hash row for ~2s.]

Now I'll recompute that hash from the same inputs, byte for byte, in the browser console.

[Cut to: clearwatch.pages.dev open in a fresh tab. Dev console open. Paste the pre-prepared snippet and execute.]

```js
const proof = await window.fetchProof(
  '7rmV5Pn1oF9pjV5XQDmYQ3tcGomagm9XAAMwsDP3EyoP'
);
const computed = await window.recomputeProofHash(proof);
console.log('on-chain :', proof.proofHashHex);
console.log('computed :', computed);
console.log('match    :', proof.proofHashHex === computed);
```

[Console output renders: matching hex strings, `match: true`.]

Match. Byte for byte.

This is what makes the Innocence Proof a primitive, not a database row. The hash is reproducible from the inputs that produced it. An auditor, a compliance team, a counterparty — anyone can verify exactly what the agent did. No API key. No trusted intermediary. No permission.

## (2:10–2:30) Where this composes

[Screen: README "Ecosystem Integration — new surface" section visible.]

Today the seed is `"innocence_proof"`, agent, counterparty. The Ecosystem Integration roadmap generalizes the agent slot to caller — any program PDA, any user wallet. Jupiter, Wormhole, Phantom each get their own proof namespace. Same hash, same verifiability, scoped per integrator. That's how this stops being one product and becomes infrastructure.

---

## Delivery notes

- The verification beat is the load-bearing moment of the entire video. Land the visual cleanly: Explorer hex → console hex → `match: true` line. If anything goes wrong here, re-record the whole take — there is no salvaging this from a partial.
- The console snippet must execute first try. Test it against the same PDA in a private-window dry-run immediately before recording. Browser caching can leave stale helpers loaded; a hard reload before the take is mandatory.
- Have the Explorer tab and the console tab pre-arranged so the cut between them is one keystroke (Cmd-Tab / Alt-Tab). Don't use the mouse — too slow.
- The "Match. Byte for byte." line reads flat. Not triumphant. The credibility is in the byte equality, not the delivery.
- Cursor discipline: in the IDE sections, cursor sits on the relevant line for the whole narration. No drift. Same for Explorer — point at the `proof_hash` row once and hold.
- The closing line "becomes infrastructure" is the resolution beat. Drop pitch on "infrastructure". Same vocal handling as the pitch video's "owned by no one".
- If recording overruns 3:00, see the **Trim order** section below.

## Pre-recording setup

This walkthrough requires two helpers exposed on `window` from inside `clearwatch.html`'s module-scoped script. **Add them before recording**, redeploy via `wrangler pages deploy dist --project-name=clearwatch --branch=main --commit-dirty=true`, hard-reload the public URL, and dry-run the snippet against the target PDA. If `match` is not `true`, do not record.

```js
// Paste inside the existing <script type="module"> block in clearwatch.html,
// after decodeInnocenceProof is defined and `connection` is in scope.

window.fetchProof = async function(pdaBase58) {
  const pda = new PublicKey(pdaBase58);
  const acc = await connection.getAccountInfo(pda);
  if (!acc) throw new Error('PDA not found on ' + RPC_URL);
  const proof = decodeInnocenceProof(acc.data);
  proof.proofHashHex = proof.proofHash
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');
  return proof;
};

window.recomputeProofHash = async function(proof) {
  // Layout must match programs/clearwatch/src/instructions/check_and_prove.rs
  // 32 agent + 32 counterparty + 8 amount(LE) + 32 purpose_hash + 1 is_clear + 8 timestamp(LE)
  const buf = new Uint8Array(32 + 32 + 8 + 32 + 1 + 8);
  buf.set(proof.agent.toBytes(), 0);
  buf.set(proof.counterparty.toBytes(), 32);
  new DataView(buf.buffer, 64, 8).setBigUint64(0, BigInt(proof.amount), true);
  buf.set(Uint8Array.from(proof.purposeHash), 72);
  buf[104] = proof.isClear ? 1 : 0;
  new DataView(buf.buffer, 105, 8).setBigInt64(0, BigInt(proof.timestamp), true);
  const digest = await crypto.subtle.digest('SHA-256', buf);
  return Array.from(new Uint8Array(digest))
    .map(b => b.toString(16).padStart(2, '0')).join('');
};
```

Pre-recording checklist:

1. Helpers added to `clearwatch.html`. Mirror change into `dist/index.html`. Redeploy via wrangler.
2. Hard-reload `clearwatch.pages.dev` and verify `typeof window.recomputeProofHash === 'function'` in console.
3. Run the dry-run snippet against PDA `7rmV5Pn1oF9pjV5XQDmYQ3tcGomagm9XAAMwsDP3EyoP`. Confirm `match: true`.
4. IDE: open `programs/clearwatch/src/instructions/check_and_prove.rs`. Jump to `compute_proof_hash`. Font ≥18pt. Same for `lib.rs` (4 entry points visible without scrolling).
5. Solana Explorer: open the InnocenceProof PDA URL with `?cluster=devnet`. Wait for the Anchor decoder to render. Pin tab.
6. Browser: clearwatch.pages.dev in a fresh tab. Dev console open (Cmd-Opt-J on macOS, F12 on Windows). Pre-paste the recording snippet so up-arrow recall is one keystroke away.
7. OBS scenes: IDE, Explorer, Browser+Console. Hotkey-switchable. 1080p / 30fps / NVENC, MKV output.
8. Mic check. Headphone monitoring on. Room quiet.

## Trim order (if take runs over 3:00)

Cut in this order:

1. Section 5 ("Where this composes") closing — drop "Same hash, same verifiability, scoped per integrator" and end on "their own proof namespace." Saves ~5s.
2. Section 1 narration tightening — drop "About 200 kilobytes compiled" if present, drop the parenthetical descriptors. Saves ~3-4s.
3. Section 2 PDA seeds — collapse into one sentence: "PDAs are derived deterministically — `risk_entry` per address, `innocence_proof` per agent–counterparty pair." Saves ~10s.
4. Last resort: cut Section 5 entirely. The verification beat is the message; the composition framing is in the README.

## What this walkthrough deliberately avoids

- Component-renaming explanation. The walkthrough uses functional descriptions (`the registry`, `the API`) consistent with the pitch script. The README carries the component-name convention (Open Registry, Self POI API, etc.); the walkthrough does not introduce them verbally.
- A second pass over the `report_address` staking flow. The pitch covers the registry side; this walkthrough is dedicated to the SHA-256 verifiability claim. One video, one point.
- Line-by-line Anchor account-validation rules. Showing `#[account(seeds=…, init, payer=…)]` in passing is enough for a technical viewer; reading constraints aloud kills pace.
- Discussion of `upgrade_tier` and `slash_reporter` beyond naming them. They exist; their semantics aren't load-bearing for verifiability.
- Confidential Checks via Arcium. The README and pitch already carry "Designed, not yet built"; introducing it here muddies the "this is what we shipped" focus.
- The word "layer" — same rule as the pitch script. In a Solana context "Layer 1 / Layer 2" reads as L1/L2 blockchain terminology.
- Named incumbent AML providers. Same rule as the pitch script.
