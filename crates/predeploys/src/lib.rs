#![no_std]
#![allow(dead_code)]

pub const ARB_SYS: [u8; 20] = hex20(0x64);
pub const ARB_ADDRESS_TABLE: [u8; 20] = hex20(0x66);
pub const ARB_BLS: [u8; 20] = hex20(0x67);
pub const ARB_FUNCTION_TABLE: [u8; 20] = hex20(0x68);
pub const ARB_GAS_INFO: [u8; 20] = hex20(0x6c);
pub const ARB_OWNER_PUBLIC: [u8; 20] = hex20(0x6b);
pub const ARB_RETRYABLE_TX: [u8; 20] = hex20(0x6e);
pub const ARB_STATISTICS: [u8; 20] = hex20(0x6f);
pub const ARB_OWNER: [u8; 20] = hex20(0x70);
pub const ARB_WASM: [u8; 20] = hex20(0x71);
pub const ARB_WASM_CACHE: [u8; 20] = hex20(0x72);
pub const ARB_NATIVE_TOKEN_MANAGER: [u8; 20] = hex20(0x73);
pub const NODE_INTERFACE: [u8; 20] = hex20(0xc8);
pub const NODE_INTERFACE_DEBUG: [u8; 20] = hex20(0xc9);
pub const ARB_DEBUG: [u8; 20] = hex20(0xff);

const fn hex20(last: u8) -> [u8; 20] {
    let mut out = [0u8; 20];
    out[19] = last;
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predeploy_addresses_match_expected_suffixes() {
        assert_eq!(&ARB_SYS[12..], &[0,0,0,0,0,0,0,0x64]);
        assert_eq!(&ARB_RETRYABLE_TX[12..], &[0,0,0,0,0,0,0,0x6e]);
        assert_eq!(&ARB_OWNER[12..], &[0,0,0,0,0,0,0,0x70]);
        assert_eq!(&NODE_INTERFACE[12..], &[0,0,0,0,0,0,0,0xc8]);
        assert_eq!(&ARB_DEBUG[12..], &[0,0,0,0,0,0,0,0xff]);
    }
}
