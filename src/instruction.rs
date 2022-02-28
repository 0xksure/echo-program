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
    /// Authorized echo
    ///
    /// input accounts:
    /// 1. authorized_buffer:
    ///     -  signer: false
    ///     - writable: true
    ///
    /// 2. authority:
    ///     - signer: true
    ///     - writable: false
    ///
    AuthorizedEcho { data: Vec<u8> },
    /// initialize vending machine mint
    /// only holders of mint can access buffer
    ///
    /// input accounts:
    /// 1. vending_machine_buffer:pda of echo program
    ///     - signer: false
    ///     - writable: true
    /// 2. vending_machine_mint: token mint
    ///     - signer: false
    ///     - writable: true
    /// 3. payer:
    ///     - signer: true,
    ///     - writable: false
    /// 4. system_program
    ///     - signer: false
    ///     - writable: fal
    InitializeVendingMachine { price: u64, buffer_size: usize },
}
