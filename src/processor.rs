use std::borrow::BorrowMut;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::instruction::EchoInstruction;
use crate::state::{AuthorizedBufferHeader, AUTH_BUFFER_HEADER_SIZE};
pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EchoInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            EchoInstruction::Echo { data } => {
                msg!("Echo account");
                let account_iter = &mut accounts.iter();
                let echo_buffer = next_account_info(account_iter)?;
                let buffer = &mut (*echo_buffer.data).borrow_mut();
                msg!("data_: {:?} ", data);
                if buffer.len() == 0 {
                    msg!("Account has data length of 0. Failing. ");
                    return Err(ProgramError::AccountDataTooSmall);
                }
                let bytes_to_copy = buffer.len();
                for index in 0..bytes_to_copy {
                    buffer[index] = data[index]
                }
                msg!(
                    "Successfully wrote {} bytes to account of size {}",
                    bytes_to_copy,
                    buffer.len()
                );
            }
            EchoInstruction::InitializeAuthorizedEcho {
                buffer_seed,
                buffer_size,
            } => {
                msg!("Initialize Authorized echo");
                let account_iter = &mut accounts.iter();
                let authorized_buffer = next_account_info(account_iter)?;
                let authority = next_account_info(account_iter)?;
                let system_program = next_account_info(account_iter)?;

                if authorized_buffer.owner != authority.key {
                    msg!("authorized buffer owner is not the same as authority key");
                    return Err(ProgramError::IllegalOwner);
                }

                let auth_buffer_data = &mut (*authorized_buffer.data).borrow_mut();
                let buffer_seed_b = buffer_seed.to_le_bytes();
                let (pubkey, bump_seed) = Pubkey::find_program_address(
                    &[b"authority", authority.key.as_ref(), &buffer_seed_b],
                    &_program_id,
                );

                if pubkey != *authorized_buffer.key {
                    msg!("authorized buffer is not a correct PDA");
                    return Err(ProgramError::InvalidAccountData);
                }

                // create pda
                let create_account_ix = system_instruction::create_account(
                    &authority.key,
                    &authorized_buffer.key,
                    Rent::get()?.minimum_balance(buffer_size),
                    buffer_size as u64,
                    _program_id,
                );

                invoke_signed(
                    &create_account_ix,
                    &[
                        authorized_buffer.clone(),
                        authority.clone(),
                        system_program.clone(),
                    ],
                    &[&[
                        b"authority",
                        authority.key.as_ref(),
                        &buffer_seed.to_le_bytes(),
                        &[bump_seed],
                    ]],
                )?;

                let buffer = &mut (*authorized_buffer.data).borrow_mut();

                let buffer_header = AuthorizedBufferHeader {
                    bump_seed,
                    buffer_seed,
                };

                buffer[0..AUTH_BUFFER_HEADER_SIZE]
                    .copy_from_slice(&buffer_header.try_to_vec().unwrap());
                msg!("Authorized buffer len: {}", buffer_size);
                msg!("Bump seed: {}", bump_seed);
                msg!("Buffer seed: {}", buffer_seed);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::{borrow::Borrow, mem};

    // #[test]
    // fn test_initialize_authorize_echo() {
    //     let program_id = Pubkey::default();
    //     let authority = Pubkey::default();
    //     let mut lamports = 0;
    //     let mut authority_lamports = 10;
    //     let mut authority_data = vec![0; mem::size_of::<u32>()];
    //     let buffer_seed: u64 = 3;
    //     let buffer_size: usize = 8;
    //     let (authorized_key, bump) = Pubkey::find_program_address(
    //         &[b"authority", authority.as_ref(), &buffer_seed.to_le_bytes()],
    //         &program_id,
    //     );
    //     let mut data = vec![0; buffer_size];

    //     let authorization_echo_account = AccountInfo::new(
    //         &authorized_key,
    //         false,
    //         true,
    //         &mut lamports,
    //         &mut data,
    //         &authority,
    //         false,
    //         Epoch::default(),
    //     );

    //     let authority_account = AccountInfo::new(
    //         &authority,
    //         true,
    //         false,
    //         &mut authority_lamports,
    //         &mut authority_data,
    //         &program_id,
    //         false,
    //         Epoch::default(),
    //     );

    //     let system_program = AccountInfo::new(
    //         &system_program::id(),
    //         true,
    //         false,
    //         &mut 0,
    //         &mut [0; 1],
    //         &system_program::id(),
    //         false,
    //         Epoch::default(),
    //     );

    //     let accounts = vec![
    //         authorization_echo_account,
    //         authority_account,
    //         system_program,
    //     ];

    //     let mut instruction_data: Vec<u8> = Vec::new();
    //     let authorization_echo_instruction = EchoInstruction::InitializeAuthorizedEcho {
    //         buffer_seed,
    //         buffer_size,
    //     };
    //     authorization_echo_instruction
    //         .serialize(&mut instruction_data)
    //         .unwrap();

    //     Processor::process_instruction(&program_id, &accounts, &instruction_data).unwrap();
    //     println!("data: {:?}", (*accounts[0].data).borrow());
    // }
    // #[test]
    // fn test_sanity() {
    //     let program_id = Pubkey::default();
    //     let key = Pubkey::default();
    //     let mut lamports = 0;
    //     let mut data = vec![0; mem::size_of::<u32>()];
    //     let owner = Pubkey::default();

    //     let echo_account = AccountInfo::new(
    //         &key,
    //         false,
    //         true,
    //         &mut lamports,
    //         &mut data,
    //         &owner,
    //         false,
    //         Epoch::default(),
    //     );

    //     let mut instruction_data: Vec<u8> = Vec::new();
    //     //instruction_data.push(0);
    //     let echo_instruction = EchoInstruction::Echo {
    //         data: vec![0, 1, 1, 1, 2],
    //     };
    //     echo_instruction.serialize(&mut instruction_data).unwrap();

    //     let instruction = EchoInstruction::try_from_slice(&instruction_data);
    //     println!("{:?}", instruction);
    //     let accounts = vec![echo_account];

    //     Processor::process_instruction(&program_id, &accounts, &instruction_data).unwrap();

    //     println!("data. {:?}", (*accounts[0].data).borrow());
    //     assert!(false)
    // }
}
