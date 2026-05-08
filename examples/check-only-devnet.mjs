/**
 * Devnet verification for the check_only instruction.
 *
 * 1. Submits report_address for a freshly-generated counterparty A (Tier 1, TTL 1h).
 * 2. Calls check_only(A)             — expects BLOCKED, return_data { is_clear=false, risk_score=50, tier=1 }.
 * 3. Calls check_only(WrappedSOLMint) — expects CLEAR,   return_data { is_clear=true,  risk_score=0,  tier=0 }.
 *
 * Reads return_data via getTransaction(...).meta.returnData.
 */
import {
  Connection, PublicKey, Keypair, Transaction, TransactionInstruction,
  SystemProgram, sendAndConfirmTransaction,
} from '@solana/web3.js';
import { readFileSync } from 'node:fs';
import { homedir } from 'node:os';

const PROGRAM_ID = new PublicKey('FDMGN1Gp62gK1TAnVvq2DM4HV6BhFwJ9Me5djLVKEKgB');
const WSOL_MINT = new PublicKey('So11111111111111111111111111111111111111112');

const DISC = {
  report_address: Buffer.from([218, 57, 210, 32, 75, 236, 251, 64]),
  check_only:     Buffer.from([231, 87, 157, 175, 225, 213, 255, 123]),
};

function pdaRiskEntry(addr) {
  return PublicKey.findProgramAddressSync([Buffer.from('risk_entry'), addr.toBuffer()], PROGRAM_ID)[0];
}
function pdaStakeVault(addr) {
  return PublicKey.findProgramAddressSync([Buffer.from('stake_vault'), addr.toBuffer()], PROGRAM_ID)[0];
}

function encodeReportAddress(flagged, incidentType) {
  const incidentBytes = Buffer.from(incidentType, 'utf8');
  const lenBuf = Buffer.alloc(4);
  lenBuf.writeUInt32LE(incidentBytes.length, 0);
  return Buffer.concat([DISC.report_address, flagged.toBuffer(), lenBuf, incidentBytes]);
}

function encodeCheckOnly(counterparty) {
  return Buffer.concat([DISC.check_only, counterparty.toBuffer()]);
}

function decodeCheckOnlyResult(buf) {
  return {
    is_clear: buf.readUInt8(0) === 1,
    risk_score: buf.readUInt8(1),
    risk_tier_at_check: buf.readUInt8(2),
  };
}

const conn = new Connection('https://api.devnet.solana.com', 'confirmed');

const walletPath = `${homedir()}/.config/solana/id.json`;
const wallet = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(readFileSync(walletPath, 'utf8'))));
console.log(`wallet:        ${wallet.publicKey.toBase58()}`);
console.log(`balance:       ${(await conn.getBalance(wallet.publicKey)) / 1e9} SOL\n`);

// ── 1) Flag a fresh counterparty ────────────────────────────────────────────
const flaggedKp = Keypair.generate();
const flagged = flaggedKp.publicKey;
console.log(`flagged target: ${flagged.toBase58()}`);

const ixReport = new TransactionInstruction({
  programId: PROGRAM_ID,
  keys: [
    { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
    { pubkey: pdaRiskEntry(flagged), isSigner: false, isWritable: true },
    { pubkey: pdaStakeVault(flagged), isSigner: false, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ],
  data: encodeReportAddress(flagged, 'Hack / Exploit'),
});
const sigReport = await sendAndConfirmTransaction(conn, new Transaction().add(ixReport), [wallet]);
console.log(`report_address tx: ${sigReport}`);
console.log(`  https://explorer.solana.com/tx/${sigReport}?cluster=devnet\n`);

// ── 2) check_only against the flagged address ──────────────────────────────
const ixBlocked = new TransactionInstruction({
  programId: PROGRAM_ID,
  keys: [
    { pubkey: wallet.publicKey, isSigner: true, isWritable: false },
    { pubkey: pdaRiskEntry(flagged), isSigner: false, isWritable: false },
  ],
  data: encodeCheckOnly(flagged),
});
const sigBlocked = await sendAndConfirmTransaction(conn, new Transaction().add(ixBlocked), [wallet]);
console.log(`check_only(flagged) tx: ${sigBlocked}`);

const txBlocked = await conn.getTransaction(sigBlocked, { commitment: 'confirmed', maxSupportedTransactionVersion: 0 });
const rdBlocked = txBlocked.meta.returnData;
const dataBlocked = Buffer.from(rdBlocked.data[0], rdBlocked.data[1]);
const blockedResult = decodeCheckOnlyResult(dataBlocked);
console.log(`  return_data raw: ${dataBlocked.toString('hex')}`);
console.log(`  decoded:         ${JSON.stringify(blockedResult)}`);
console.log(`  https://explorer.solana.com/tx/${sigBlocked}?cluster=devnet\n`);

// ── 3) check_only against a known-clean address (Wrapped SOL mint) ─────────
const ixClear = new TransactionInstruction({
  programId: PROGRAM_ID,
  keys: [
    { pubkey: wallet.publicKey, isSigner: true, isWritable: false },
    // For the None case, pass program_id as the optional account placeholder.
    // Anchor treats program_id at this position as "account not present".
    { pubkey: PROGRAM_ID, isSigner: false, isWritable: false },
  ],
  data: encodeCheckOnly(WSOL_MINT),
});
const sigClear = await sendAndConfirmTransaction(conn, new Transaction().add(ixClear), [wallet]);
console.log(`check_only(WSOL-mint, no risk_entry) tx: ${sigClear}`);

const txClear = await conn.getTransaction(sigClear, { commitment: 'confirmed', maxSupportedTransactionVersion: 0 });
const rdClear = txClear.meta.returnData;
const dataClear = Buffer.from(rdClear.data[0], rdClear.data[1]);
const clearResult = decodeCheckOnlyResult(dataClear);
console.log(`  return_data raw: ${dataClear.toString('hex')}`);
console.log(`  decoded:         ${JSON.stringify(clearResult)}`);
console.log(`  https://explorer.solana.com/tx/${sigClear}?cluster=devnet\n`);

// ── Assertions ─────────────────────────────────────────────────────────────
const expect = (label, actual, expected) => {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  console.log(`${ok ? 'PASS' : 'FAIL'} ${label}`);
  if (!ok) console.log(`     expected ${JSON.stringify(expected)} got ${JSON.stringify(actual)}`);
  return ok;
};

const allPass =
  expect('BLOCKED return shape', blockedResult, { is_clear: false, risk_score: 50, risk_tier_at_check: 1 }) &
  expect('CLEAR   return shape', clearResult,   { is_clear: true,  risk_score: 0,  risk_tier_at_check: 0 });

console.log(`\nbalance after: ${(await conn.getBalance(wallet.publicKey)) / 1e9} SOL`);
process.exit(allPass ? 0 : 1);
