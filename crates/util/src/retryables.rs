#![allow(dead_code)]

pub fn retryable_submission_fee(calldata_len: usize, l1_base_fee_wei: u128) -> u128 {
    let overhead: u128 = 1400;
    let per_byte: u128 = 16;
    let bytes = calldata_len as u128;
    (overhead + per_byte * bytes) * l1_base_fee_wei
}

pub fn escrow_address_from_ticket(_ticket_id: [u8; 32]) -> [u8; 20] {
    [0u8; 20]
}
