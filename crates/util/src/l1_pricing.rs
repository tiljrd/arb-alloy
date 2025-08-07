#![allow(dead_code)]

pub struct L1PricingState {
    pub l1_base_fee_wei: u128,
}

impl L1PricingState {
    pub fn poster_data_cost(&self, data_gas: u128) -> u128 {
        self.l1_base_fee_wei.saturating_mul(data_gas)
    }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poster_data_cost_multiplies_base_fee_by_data_gas() {
        let state = L1PricingState { l1_base_fee_wei: 1_000 };
        assert_eq!(state.poster_data_cost(0), 0);
        assert_eq!(state.poster_data_cost(1), 1_000);
        assert_eq!(state.poster_data_cost(10), 10_000);
        assert_eq!(state.poster_data_cost(123456789), 123_456_789_000);
    }
}

}
