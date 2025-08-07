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

const fn hex20(last: u128) -> [u8; 20] {
    let b = last.to_be_bytes();
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]
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

#![no_std]
#![allow(dead_code)]

pub const ARB_SYS: [u8; 20] = hex20(0x0000000000000000000000000000000000000064);
pub const ARB_ADDRESS_TABLE: [u8; 20] = hex20(0x0000000000000000000000000000000000000066);
pub const ARB_BLS: [u8; 20] = hex20(0x0000000000000000000000000000000000000067);
pub const ARB_FUNCTION_TABLE: [u8; 20] = hex20(0x0000000000000000000000000000000000000068);
pub const ARB_GAS_INFO: [u8; 20] = hex20(0x000000000000000000000000000000000000006c);
pub const ARB_OWNER_PUBLIC: [u8; 20] = hex20(0x000000000000000000000000000000000000006b);
pub const ARB_RETRYABLE_TX: [u8; 20] = hex20(0x000000000000000000000000000000000000006e);
pub const ARB_STATISTICS: [u8; 20] = hex20(0x000000000000000000000000000000000000006f);
pub const ARB_OWNER: [u8; 20] = hex20(0x0000000000000000000000000000000000000070);
pub const ARB_WASM: [u8; 20] = hex20(0x0000000000000000000000000000000000000071);
pub const ARB_WASM_CACHE: [u8; 20] = hex20(0x0000000000000000000000000000000000000072);
pub const ARB_NATIVE_TOKEN_MANAGER: [u8; 20] = hex20(0x0000000000000000000000000000000000000073);
pub const NODE_INTERFACE: [u8; 20] = hex20(0x00000000000000000000000000000000000000c8);
pub const NODE_INTERFACE_DEBUG: [u8; 20] = hex20(0x00000000000000000000000000000000000000c9);
pub const ARB_DEBUG: [u8; 20] = hex20(0x00000000000000000000000000000000000000ff);

const fn hex20(x: u128) -> [u8; 20] {
    let b = x.to_be_bytes();
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]
}
