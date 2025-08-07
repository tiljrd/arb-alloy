#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbReceiptEnvelope {
    pub status: bool,
    pub cumulative_gas_used: u128,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_receipt_envelope_fields() {
        let r = ArbReceiptEnvelope { status: true, cumulative_gas_used: 12345 };
        assert!(r.status);
        assert_eq!(r.cumulative_gas_used, 12345);
    }
}
