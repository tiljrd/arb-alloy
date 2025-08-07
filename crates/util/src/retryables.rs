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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submission_fee_matches_simple_formula() {
        let calldata_len = 100usize;
        let l1_base = 1_000u128;
        let expected = (1400u128 + 16u128 * (calldata_len as u128)) * l1_base;
        assert_eq!(retryable_submission_fee(calldata_len, l1_base), expected);
    }
}
