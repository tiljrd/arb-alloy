#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbReceiptEnvelope {
    pub status: bool,
    pub cumulative_gas_used: u128,
    pub l1_gas_used: Option<u128>,
    pub logs_bloom: [u8; 256],
    pub logs: Vec<ArbLog>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbLog {
    pub address: [u8; 20],
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_receipt_envelope_fields() {
        let r = ArbReceiptEnvelope {
            status: true,
            cumulative_gas_used: 12345,
            l1_gas_used: None,
            logs_bloom: [0u8; 256],
            logs: Vec::new(),
        };
        assert!(r.status);
        assert_eq!(r.cumulative_gas_used, 12345);
        assert_eq!(r.l1_gas_used, None);
        assert_eq!(r.logs.len(), 0);
    }

    #[test]
    fn l1_gas_used_optional_field_works() {
        let r = ArbReceiptEnvelope {
            status: false,
            cumulative_gas_used: 1,
            l1_gas_used: Some(777),
            logs_bloom: [0u8; 256],
            logs: Vec::new(),
        };
        assert!(!r.status);
        assert_eq!(r.cumulative_gas_used, 1);
        assert_eq!(r.l1_gas_used, Some(777));
    }
}
