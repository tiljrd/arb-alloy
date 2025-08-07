#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbReceiptEnvelope {
    pub status: bool,
    pub cumulative_gas_used: u128,
}
