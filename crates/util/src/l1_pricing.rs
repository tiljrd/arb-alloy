#![allow(dead_code)]

pub const TX_DATA_NONZERO_GAS_EIP2028: u64 = 16;
pub const ONE_IN_BIPS: u64 = 10_000;
pub const ESTIMATION_PADDING_UNITS: u64 = 16 * TX_DATA_NONZERO_GAS_EIP2028;
pub const ESTIMATION_PADDING_BASIS_POINTS: u64 = 100;

pub struct L1PricingState {
    pub l1_base_fee_wei: u128,
}

impl L1PricingState {
    pub fn poster_data_cost_from_units(&self, units: u128) -> u128 {
        self.l1_base_fee_wei.saturating_mul(units)
    }

    pub fn poster_units_from_brotli_len(len_bytes: u64) -> u128 {
        (len_bytes as u128).saturating_mul(TX_DATA_NONZERO_GAS_EIP2028 as u128)
    }

    pub fn apply_estimation_padding(units: u128) -> u128 {
        let padded_units = units.saturating_add(ESTIMATION_PADDING_UNITS as u128);
        let bips = (ONE_IN_BIPS + ESTIMATION_PADDING_BASIS_POINTS) as u128;
        padded_units.saturating_mul(bips) / ONE_IN_BIPS as u128
    }

    pub fn poster_data_cost_estimate_from_len(&self, brotli_len_bytes: u64) -> (u128, u128) {
        let units = Self::poster_units_from_brotli_len(brotli_len_bytes);
        let padded_units = Self::apply_estimation_padding(units);
        (self.poster_data_cost_from_units(padded_units), padded_units)
    }

    pub fn poster_data_cost(&self, data_gas: u128) -> u128 {
        self.l1_base_fee_wei.saturating_mul(data_gas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tx_data_nonzero_gas_constant() {
        assert_eq!(TX_DATA_NONZERO_GAS_EIP2028, 16);
    }

    #[test]
    fn poster_units_from_brotli_len_multiplies_by_16() {
        assert_eq!(L1PricingState::poster_units_from_brotli_len(0), 0);
        assert_eq!(L1PricingState::poster_units_from_brotli_len(1), 16);
        assert_eq!(L1PricingState::poster_units_from_brotli_len(10), 160);
        assert_eq!(L1PricingState::poster_units_from_brotli_len(1234), 19_744);
    }

    #[test]
    fn apply_estimation_padding_adds_units_and_1_percent() {
        let base_units = 10_000u128;
        let padded = L1PricingState::apply_estimation_padding(base_units);
        let expected_added = base_units + ESTIMATION_PADDING_UNITS as u128;
        let expected = expected_added * (ONE_IN_BIPS as u128 + ESTIMATION_PADDING_BASIS_POINTS as u128) / ONE_IN_BIPS as u128;
        assert_eq!(padded, expected);
    }

    #[test]
    fn poster_data_cost_from_units_multiplies_by_price_per_unit() {
        let state = L1PricingState { l1_base_fee_wei: 1_000 };
        assert_eq!(state.poster_data_cost_from_units(0), 0);
        assert_eq!(state.poster_data_cost_from_units(1), 1_000);
        assert_eq!(state.poster_data_cost_from_units(10), 10_000);
    }

    #[test]
    fn poster_data_cost_estimate_from_len_pipeline() {
        let state = L1PricingState { l1_base_fee_wei: 1_000 };
        let len = 100u64;
        let (cost, padded_units) = state.poster_data_cost_estimate_from_len(len);
        let expected_units = L1PricingState::poster_units_from_brotli_len(len);
        let expected_padded = L1PricingState::apply_estimation_padding(expected_units);
        assert_eq!(padded_units, expected_padded);
        assert_eq!(cost, state.poster_data_cost_from_units(expected_padded));
    }

    #[test]
    fn poster_data_cost_multiplies_base_fee_by_data_gas() {
        let state = L1PricingState { l1_base_fee_wei: 1_000 };
        assert_eq!(state.poster_data_cost(123456789), 123_456_789_000);
    }

    #[test]
    fn poster_data_cost_is_zero_when_base_fee_zero() {
        let state = L1PricingState { l1_base_fee_wei: 0 };
        assert_eq!(state.poster_data_cost_from_units(123456), 0);
        assert_eq!(state.poster_data_cost(987654321), 0);
    }

    #[test]
    fn apply_estimation_padding_is_monotonic() {
        let a = 10_000u128;
        let b = 50_000u128;
        assert!(L1PricingState::apply_estimation_padding(a) < L1PricingState::apply_estimation_padding(b));
    }
}
