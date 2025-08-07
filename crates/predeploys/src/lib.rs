#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;

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

pub const SIG_SEND_TX_TO_L1: &str = "sendTxToL1(address,bytes)";
pub const SIG_WITHDRAW_ETH: &str = "withdrawEth(address)";
pub const SIG_CREATE_RETRYABLE_TICKET: &str =
    "createRetryableTicket(address,uint256,uint256,address,address,uint256,uint256,bytes)";
pub const SIG_RETRY_REDEEM: &str = "redeem(bytes32)";
pub const SIG_CANCEL_RETRYABLE_TICKET: &str = "cancelRetryableTicket(bytes32)";

pub const EVT_TICKET_CREATED: &str =
    "TicketCreated(bytes32,address,uint256,uint256,address,address,uint256,uint256)";
pub const EVT_TICKET_REDEEMED: &str = "Redeemed(bytes32,address)";
pub const EVT_TICKET_CANCELED: &str = "Canceled(bytes32,address)";

pub fn signature_bytes(sig: &str) -> Vec<u8> {
    sig.as_bytes().to_vec()
}

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

    #[test]
    fn signature_strings_present() {
        assert!(SIG_SEND_TX_TO_L1.len() > 0);
        assert!(SIG_CREATE_RETRYABLE_TICKET.len() > 0);
        assert!(EVT_TICKET_CREATED.len() > 0);
    }
}
