#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbReceiptEnvelope {
    pub status: bool,
    pub cumulative_gas_used: u128,
    pub l1_gas_used: Option<u128>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_receipt_envelope_fields() {
        let r = ArbReceiptEnvelope { status: true, cumulative_gas_used: 12345, l1_gas_used: None };
        assert!(r.status);
        assert_eq!(r.cumulative_gas_used, 12345);
        assert_eq!(r.l1_gas_used, None);
    }

    #[test]
    fn l1_gas_used_optional_field_works() {
        let r = ArbReceiptEnvelope { status: false, cumulative_gas_used: 1, l1_gas_used: Some(777) };
        assert!(!r.status);
        assert_eq!(r.cumulative_gas_used, 1);
        assert_eq!(r.l1_gas_used, Some(777));
    }
}
