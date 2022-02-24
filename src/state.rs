use std::mem::size_of;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct EchoBuffer {
    pub data: Vec<u8>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct AuthorizedBufferHeader {
    pub bump_seed: u8,
    pub buffer_seed: u64,
}

pub const AUTH_BUFFER_HEADER_SIZE: usize = size_of::<u8>() + size_of::<u64>();
