use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::instruction::Instruction,
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

fn program_id() -> Pubkey {
    clearwatch::id()
}

fn send_tx(
    svm: &mut LiteSVM,
    payer: &Keypair,
    instructions: Vec<Instruction>,
) -> Result<(), String> {
    let blockhash = svm.latest_blockhash();
    let msg =
        Message::new_with_blockhash(&instructions, Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer])
        .map_err(|e| e.to_string())?;
    svm.send_transaction(tx).map_err(|e| format!("{:?}", e))?;
    Ok(())
}

fn pda_risk_entry(flagged: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"risk_entry", flagged.as_ref()], &program_id()).0
}

fn pda_stake_vault(flagged: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"stake_vault", flagged.as_ref()], &program_id()).0
}

fn pda_innocence_proof(agent: &Pubkey, counterparty: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"innocence_proof", agent.as_ref(), counterparty.as_ref()],
        &program_id(),
    )
    .0
}

#[test]
fn test_report_address() {
    let program_id = program_id();
    let reporter = Keypair::new();
    let flagged = Keypair::new().pubkey();

    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/clearwatch.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&reporter.pubkey(), 2_000_000_000).unwrap();

    let risk_entry = pda_risk_entry(&flagged);
    let stake_vault = pda_stake_vault(&flagged);

    let ix = Instruction::new_with_bytes(
        program_id,
        &clearwatch::instruction::ReportAddress {
            flagged_address: flagged,
            incident_type: "Hack/Exploit".to_string(),
        }
        .data(),
        clearwatch::accounts::ReportAddress {
            reporter: reporter.pubkey(),
            risk_entry,
            stake_vault,
            system_program: anchor_lang::solana_program::system_program::id(),
        }
        .to_account_metas(None),
    );

    let result = send_tx(&mut svm, &reporter, vec![ix]);
    assert!(result.is_ok(), "report_address failed: {:?}", result);

    // Verify risk entry state
    let account = svm.get_account(&risk_entry).unwrap();
    let entry = clearwatch::RiskEntry::try_deserialize(&mut account.data.as_slice()).unwrap();
    assert_eq!(entry.address, flagged);
    assert_eq!(entry.tier, 1);
    assert_eq!(entry.incident_type, "Hack/Exploit");
    assert_eq!(entry.report_count, 1);

    println!("✓ report_address: address flagged at Tier 1");
}

#[test]
fn test_check_and_prove_clear() {
    let program_id = program_id();
    let agent = Keypair::new();
    let counterparty = Keypair::new().pubkey();

    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/clearwatch.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&agent.pubkey(), 2_000_000_000).unwrap();

    let proof_pda = pda_innocence_proof(&agent.pubkey(), &counterparty);

    let ix = Instruction::new_with_bytes(
        program_id,
        &clearwatch::instruction::CheckAndProve {
            counterparty,
            amount: 500_000_000,
            purpose: "Pay for service".to_string(),
        }
        .data(),
        clearwatch::accounts::CheckAndProve {
            agent: agent.pubkey(),
            risk_entry: None,
            innocence_proof: proof_pda,
            system_program: anchor_lang::solana_program::system_program::id(),
        }
        .to_account_metas(None),
    );

    let result = send_tx(&mut svm, &agent, vec![ix]);
    assert!(result.is_ok(), "check_and_prove CLEAR failed: {:?}", result);

    let account = svm.get_account(&proof_pda).unwrap();
    let proof =
        clearwatch::InnocenceProof::try_deserialize(&mut account.data.as_slice()).unwrap();
    assert!(proof.is_clear, "expected CLEAR result");
    assert_eq!(proof.risk_score, 0);

    println!("✓ check_and_prove: CLEAR result, proof_hash={:?}", proof.proof_hash);
}

#[test]
fn test_check_and_prove_blocked() {
    let program_id = program_id();
    let reporter = Keypair::new();
    let agent = Keypair::new();
    let flagged = Keypair::new().pubkey();

    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/clearwatch.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&reporter.pubkey(), 2_000_000_000).unwrap();
    svm.airdrop(&agent.pubkey(), 2_000_000_000).unwrap();

    // Step 1: flag the address
    let ix_report = Instruction::new_with_bytes(
        program_id,
        &clearwatch::instruction::ReportAddress {
            flagged_address: flagged,
            incident_type: "Rug Pull".to_string(),
        }
        .data(),
        clearwatch::accounts::ReportAddress {
            reporter: reporter.pubkey(),
            risk_entry: pda_risk_entry(&flagged),
            stake_vault: pda_stake_vault(&flagged),
            system_program: anchor_lang::solana_program::system_program::id(),
        }
        .to_account_metas(None),
    );
    send_tx(&mut svm, &reporter, vec![ix_report]).expect("report_address failed");

    // Step 2: agent tries to transact with flagged address → BLOCKED
    let proof_pda = pda_innocence_proof(&agent.pubkey(), &flagged);
    let risk_entry_pda = pda_risk_entry(&flagged);

    let ix_check = Instruction::new_with_bytes(
        program_id,
        &clearwatch::instruction::CheckAndProve {
            counterparty: flagged,
            amount: 1_000_000,
            purpose: "Transfer".to_string(),
        }
        .data(),
        clearwatch::accounts::CheckAndProve {
            agent: agent.pubkey(),
            risk_entry: Some(risk_entry_pda),
            innocence_proof: proof_pda,
            system_program: anchor_lang::solana_program::system_program::id(),
        }
        .to_account_metas(None),
    );

    let result = send_tx(&mut svm, &agent, vec![ix_check]);
    assert!(result.is_ok(), "check_and_prove BLOCKED failed: {:?}", result);

    let account = svm.get_account(&proof_pda).unwrap();
    let proof =
        clearwatch::InnocenceProof::try_deserialize(&mut account.data.as_slice()).unwrap();
    assert!(!proof.is_clear, "expected BLOCKED result");
    assert_eq!(proof.risk_tier_at_check, 1);
    assert_eq!(proof.risk_score, 50);

    println!("✓ check_and_prove: BLOCKED result, tier={}", proof.risk_tier_at_check);
}
