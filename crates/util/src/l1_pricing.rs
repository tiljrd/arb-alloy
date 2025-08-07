#![allow(dead_code)]

pub struct L1PricingState {
    pub l1_base_fee_wei: u128,
}

impl L1PricingState {
    pub fn poster_data_cost(&self, data_gas: u128) -> u128 {
        self.l1_base_fee_wei.saturating_mul(data_gas)
    }
}
