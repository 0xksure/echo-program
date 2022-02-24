#![cfg(feature = "test-bpf")]

use borsh::{BorshDeserialize, BorshSerialize};
use std::mem;

use solana_program::{instruction, msg, pubkey::Pubkey, system_instruction, system_program};
use solana_program_test::ProgramTest;
use xbooth::instruction::EchoInstruction;
use {
    solana_program_test::*,
    solana_sdk::signature::{Keypair, Signer},
    solana_sdk::transaction::Transaction,
};

#[tokio::test]
async fn test_echo() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::default();
    program_test.add_program("xbooth", program_id, None);

    let auth = Keypair::new();
    program_test.add_account(
        auth.pubkey(),
        solana_sdk::account::Account {
            lamports: 100_000_000_000,
            data: vec![],
            owner: system_program::id(),
            ..solana_sdk::account::Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let rent = banks_client.get_rent().await.unwrap();
    let echo_account = Keypair::new();
    let echo_account_space = 8;
    let echo_account_rent = rent.minimum_balance(echo_account_space);
    let create_ix = system_instruction::create_account(
        &payer.pubkey(),
        &echo_account.pubkey(),
        echo_account_rent,
        echo_account_space as u64,
        &program_id,
    );
    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[&payer, &echo_account],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();

    let transaction_ix: Vec<u8> = vec![0; mem::size_of::<u8>()];
    let mut echo_data: Vec<u8> = vec![3; 10];
    let mut echo_data_size: Vec<u8> = vec![echo_data.len() as u8, 0, 0, 0];
    let mut echo_ix_data = transaction_ix.clone();
    echo_ix_data.append(&mut echo_data_size);
    echo_ix_data.append(&mut echo_data);

    let echo_accountmeta = instruction::AccountMeta {
        pubkey: echo_account.pubkey(),
        is_signer: false,
        is_writable: true,
    };

    let accounts = vec![echo_accountmeta];
    let echo_ix = instruction::Instruction {
        program_id: program_id,
        data: echo_ix_data,
        accounts,
    };

    let tx2 = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[echo_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(tx2).await.unwrap();

    // let account = banks_client
    //     .get_account(echo_account.pubkey())
    //     .await
    //     .unwrap()
    //     .expect("could not get account");
    // println!("data from account: {:?} ", account.data);
}
