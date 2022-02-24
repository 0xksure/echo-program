use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum EchoInstruction {
    /// Accounts:
    ///
    /// echo_buffer: writable: true
    ///               signer: false
    Echo { data: Vec<u8> },
    /// Initialize Authorized Echo
    ///
    /// input accounts:
    /// 1. authorized_buffer:
    ///     - signer: false
    ///     - writable: true
    /// 2. authority:
    ///     - signer: true,
    ///     - writable: false
    /// 3. system_program
    ///     - signer: false
    ///     - writeable: false
    ///
    InitializeAuthorizedEcho {
        buffer_seed: u64,
        buffer_size: usize,
    },
}
