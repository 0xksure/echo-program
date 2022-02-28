#![cfg(feature = "test-bpf")]

use borsh::{BorshDeserialize, BorshSerialize};
use std::io::Cursor;
use std::mem;

use solana_program::{
    account_info::AccountInfo,
    config::program,
    instruction::{self, AccountMeta},
    msg,
    pubkey::Pubkey,
    system_instruction, system_program,
};
use solana_program_test::ProgramTest;
use xbooth::instruction::EchoInstruction;
use {
    solana_program_test::*,
    solana_sdk::signature::{Keypair, Signer},
    solana_sdk::transaction::Transaction,
};

// #[tokio::test]
// async fn test_echo() {
//     let program_id = Pubkey::new_unique();
//     let mut program_test = ProgramTest::default();
//     program_test.add_program("xbooth", program_id, None);

//     let auth = Keypair::new();
//     program_test.add_account(
//         auth.pubkey(),
//         solana_sdk::account::Account {
//             lamports: 100_000_000_000,
//             data: vec![],
//             owner: system_program::id(),
//             ..solana_sdk::account::Account::default()
//         },
//     );

//     let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
//     let rent = banks_client.get_rent().await.unwrap();
//     let echo_account = Keypair::new();
//     let echo_account_space = 8;
//     let echo_account_rent = rent.minimum_balance(echo_account_space);
//     let create_ix = system_instruction::create_account(
//         &payer.pubkey(),
//         &echo_account.pubkey(),
//         echo_account_rent,
//         echo_account_space as u64,
//         &program_id,
//     );
//     let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
//         &[create_ix],
//         Some(&payer.pubkey()),
//         &[&payer, &echo_account],
//         recent_blockhash,
//     );

//     banks_client.process_transaction(tx).await.unwrap();

//     let transaction_ix: Vec<u8> = vec![0; mem::size_of::<u8>()];
//     let mut echo_data: Vec<u8> = vec![3; 10];
//     let mut echo_data_size: Vec<u8> = vec![echo_data.len() as u8, 0, 0, 0];
//     let mut echo_ix_data = transaction_ix.clone();
//     echo_ix_data.append(&mut echo_data_size);
//     echo_ix_data.append(&mut echo_data);

//     let echo_accountmeta = instruction::AccountMeta {
//         pubkey: echo_account.pubkey(),
//         is_signer: false,
//         is_writable: true,
//     };

//     let accounts = vec![echo_accountmeta];
//     let echo_ix = instruction::Instruction {
//         program_id: program_id,
//         data: echo_ix_data,
//         accounts,
//     };

//     let tx2 = solana_sdk::transaction::Transaction::new_signed_with_payer(
//         &[echo_ix],
//         Some(&payer.pubkey()),
//         &[&payer],
//         recent_blockhash,
//     );

//     banks_client.process_transaction(tx2).await.unwrap();
// }

// #[tokio::test]
// async fn test_initialize_authorize_echo() {
//     let program_id = Pubkey::new_unique();
//     let mut program_test = ProgramTest::default();
//     program_test.add_program("xbooth", program_id, None);
//     let auth = Keypair::new();
//     program_test.add_account(
//         auth.pubkey(),
//         solana_sdk::account::Account {
//             lamports: 100_000_000_000,
//             data: vec![],
//             owner: system_program::id(),
//             ..solana_sdk::account::Account::default()
//         },
//     );
//     let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

//     let authority = instruction::AccountMeta {
//         pubkey: auth.pubkey(),
//         is_signer: true,
//         is_writable: false,
//     };

//     let buffer_seed = 10_u64.to_le_bytes();

//     let (authorized_buffer_key, bump) = Pubkey::find_program_address(
//         &[b"authority", auth.pubkey().as_ref(), &buffer_seed],
//         &program_id,
//     );

//     let authorized_buffer = instruction::AccountMeta {
//         pubkey: authorized_buffer_key,
//         is_signer: false,
//         is_writable: true,
//     };

//     let system_program_account = instruction::AccountMeta {
//         pubkey: system_program::id(),
//         is_signer: false,
//         is_writable: false,
//     };

//     let accounts = vec![
//         authorized_buffer.clone(),
//         authority.clone(),
//         system_program_account.clone(),
//     ];

//     let mut data_input = vec![1; mem::size_of::<u8>()];
//     let mut buffer_seed_i = vec![0; mem::size_of::<u64>()];
//     buffer_seed_i[0] = 10;
//     let mut buffer_size_i = vec![0; mem::size_of::<u64>()];
//     buffer_size_i[0] = 100;
//     data_input.append(&mut buffer_seed_i);
//     data_input.append(&mut buffer_size_i);

//     println!("data_input: {:?}", data_input);
//     let initialize_authorized_echo_ix = instruction::Instruction {
//         program_id: program_id,
//         data: data_input,
//         accounts,
//     };

//     let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
//         &[initialize_authorized_echo_ix],
//         Some(&auth.pubkey()),
//         &[&auth],
//         recent_blockhash,
//     );
//     banks_client.process_transaction(tx).await.unwrap();

//     // try to change the content of the buffer
//     let instruction_vec = vec![2; mem::size_of::<u8>()];
//     let new_buffer_data = vec![1; mem::size_of::<u64>()];
//     let new_buffer_data_length: Vec<u8> = vec![new_buffer_data.len() as u8, 0, 0, 0];
//     let buffer_data = [
//         &instruction_vec[..],
//         &new_buffer_data_length[..],
//         &new_buffer_data[..],
//     ]
//     .concat();
//     let update_accounts = vec![authorized_buffer.clone(), authority.clone()];

//     let update_buffer_ix = instruction::Instruction {
//         program_id: program_id,
//         data: buffer_data,
//         accounts: update_accounts,
//     };

//     let update_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
//         &[update_buffer_ix],
//         Some(&auth.pubkey()),
//         &[&auth],
//         recent_blockhash,
//     );

//     banks_client.process_transaction(update_tx).await.unwrap();
// }

#[tokio::test]
async fn test_vending_machine_buffer() {
    // initialize test
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new("xbooth", program_id, None);

    let auth = Keypair::new();

    let vending_machine_mint_keys = Keypair::new();
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

    // set up accounts
    let price: u64 = 101000;
    let (vending_machine_buffer_pubkey, bump) = Pubkey::find_program_address(
        &[
            b"vending_machine",
            vending_machine_mint_keys.pubkey().as_ref(),
            &price.to_le_bytes(),
        ],
        &program_id,
    );

    let vending_machine_buffer = AccountMeta {
        pubkey: vending_machine_buffer_pubkey,
        is_signer: false,
        is_writable: true,
    };
    let vending_machine_mint = AccountMeta {
        pubkey: vending_machine_mint_keys.pubkey(),
        is_signer: false,
        is_writable: false,
    };

    let payer = AccountMeta {
        pubkey: auth.pubkey(),
        is_signer: true,
        is_writable: false,
    };

    let system_program_account = AccountMeta {
        pubkey: system_program::id(),
        is_signer: false,
        is_writable: false,
    };
    let accounts = vec![
        vending_machine_buffer.clone(),
        vending_machine_mint.clone(),
        payer.clone(),
        system_program_account.clone(),
    ];

    // instruction input
    let data_input: Vec<u8> = vec![3; mem::size_of::<u8>()];
    let price_data = price.to_le_bytes().to_vec();
    let buffer_size = 100_u64.to_le_bytes().to_vec();

    let input_data: Vec<u8> = [&data_input[..], &price_data[..], &buffer_size[..]].concat();
    println!("input data {:?}", input_data);

    let mint_init_ix = instruction::Instruction {
        program_id: program_id,
        accounts: accounts,
        data: input_data,
    };
    println!("create tx");

    let init_mint_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[mint_init_ix],
        Some(&auth.pubkey()),
        &[&auth],
        recent_blockhash,
    );
    println!("bank client process transaction");
    banks_client
        .process_transaction(init_mint_tx)
        .await
        .unwrap();
}
