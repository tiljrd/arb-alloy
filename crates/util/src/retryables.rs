#![allow(dead_code)]

extern crate alloc;
pub const RETRYABLE_LIFETIME_SECONDS: u64 = 7 * 24 * 60 * 60;
pub const RETRYABLE_REAP_PRICE_UNITS: u64 = 58_000;

use alloc::vec::Vec;
use alloy_primitives::{keccak256, B256};

pub fn retryable_submission_fee(calldata_len: usize, l1_base_fee_wei: u128) -> u128 {
    let overhead: u128 = 1400;
    let per_byte: u128 = 6;
    let bytes = calldata_len as u128;
    (overhead + per_byte * bytes) * l1_base_fee_wei
}

pub fn escrow_address_from_ticket(ticket_id: [u8; 32]) -> [u8; 20] {
    let mut input = Vec::with_capacity(16 + 32);
    input.extend_from_slice(b"retryable escrow");
    input.extend_from_slice(&ticket_id);
    let hash: B256 = keccak256(input);
    let mut out = [0u8; 20];
    out.copy_from_slice(&hash.as_slice()[12..32]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{keccak256, B256};

    #[test]
    fn retryable_constants_match_nitro() {
        assert_eq!(RETRYABLE_LIFETIME_SECONDS, 7 * 24 * 60 * 60);
        assert_eq!(RETRYABLE_REAP_PRICE_UNITS, 58_000);
    }
    #[test]
    fn submission_fee_matches_nitro_formula() {
        let calldata_len = 100usize;
        let l1_base = 1_000u128;
        let expected = (1400u128 + 6u128 * (calldata_len as u128)) * l1_base;
        assert_eq!(retryable_submission_fee(calldata_len, l1_base), expected);
    }

    #[test]
    fn escrow_address_matches_nitro_derivation_zero_ticket() {
        let ticket = [0u8; 32];
        let mut input = Vec::with_capacity(16 + 32);
        input.extend_from_slice(b"retryable escrow");
        input.extend_from_slice(&ticket);
        let hash: B256 = keccak256(input);
        let mut expected = [0u8; 20];
        expected.copy_from_slice(&hash.as_slice()[12..32]);
        assert_eq!(escrow_address_from_ticket(ticket), expected);
    }

    #[test]
    fn escrow_address_matches_nitro_derivation_nonzero_ticket() {
        let mut ticket = [0u8; 32];
        ticket[0] = 0x12;
        ticket[15] = 0x34;
        ticket[31] = 0x56;
        let mut input = Vec::with_capacity(16 + 32);
        input.extend_from_slice(b"retryable escrow");
        input.extend_from_slice(&ticket);
        let hash: B256 = keccak256(input);
        let mut expected = [0u8; 20];
        expected.copy_from_slice(&hash.as_slice()[12..32]);
        assert_eq!(escrow_address_from_ticket(ticket), expected);
    }
}
