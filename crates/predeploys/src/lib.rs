#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;
use alloy_primitives::keccak256;

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

/* ArbSys core */
pub const SIG_SEND_TX_TO_L1: &str = "sendTxToL1(address,bytes)";
pub const SIG_WITHDRAW_ETH: &str = "withdrawEth(address)";
pub const SIG_CREATE_RETRYABLE_TICKET: &str =
    "createRetryableTicket(address,uint256,uint256,address,address,uint256,uint256,bytes)";
pub const SIG_CANCEL_RETRYABLE_TICKET: &str = "cancelRetryableTicket(bytes32)";
pub const SIG_ARB_BLOCK_NUMBER: &str = "arbBlockNumber()";
pub const SIG_ARB_BLOCK_HASH: &str = "arbBlockHash(uint64)";
pub const SIG_GET_TX_CALL_VALUE: &str = "getTxCallValue()";
pub const SIG_GET_TX_ORIGIN: &str = "getTxOrigin()";
pub const SIG_GET_BLOCK_NUMBER: &str = "getBlockNumber()";
pub const SIG_GET_BLOCK_HASH: &str = "getBlockHash(uint64)";
pub const SIG_GET_STORAGE_AT: &str = "getStorageAt(address,bytes32)";
pub const SIG_ARB_CHAIN_ID: &str = "arbChainID()";
pub const SIG_ARB_OS_VERSION: &str = "arbOSVersion()";
/* ArbOwner */
pub const SIG_OWNER_ADD_CHAIN_OWNER: &str = "addChainOwner(address)";
pub const SIG_OWNER_REMOVE_CHAIN_OWNER: &str = "removeChainOwner(address)";
pub const SIG_OWNER_IS_CHAIN_OWNER: &str = "isChainOwner(address)";
pub const SIG_OWNER_GET_ALL_CHAIN_OWNERS: &str = "getAllChainOwners()";
pub const SIG_OWNER_GET_NETWORK_FEE_ACCOUNT: &str = "getNetworkFeeAccount()";
pub const SIG_OWNER_GET_INFRA_FEE_ACCOUNT: &str = "getInfraFeeAccount()";
pub const SIG_OWNER_SET_NETWORK_FEE_ACCOUNT: &str = "setNetworkFeeAccount(address)";
pub const SIG_OWNER_SET_INFRA_FEE_ACCOUNT: &str = "setInfraFeeAccount(address)";
/* ArbRetryableTx */
pub const SIG_RETRY_GET_LIFETIME: &str = "getLifetime()";
pub const SIG_RETRY_GET_TIMEOUT: &str = "getTimeout(bytes32)";
pub const SIG_RETRY_KEEPALIVE: &str = "keepalive(bytes32)";
pub const SIG_RETRY_GET_BENEFICIARY: &str = "getBeneficiary(bytes32)";
pub const SIG_RETRY_REDEEM: &str = "redeem(bytes32)";
pub const SIG_RETRY_CANCEL: &str = "cancel(bytes32)";
pub const SIG_RETRY_GET_CURRENT_REDEEMER: &str = "getCurrentRedeemer()";
/* Non-callable but present for explorers */
pub const SIG_RETRY_SUBMIT_RETRYABLE: &str = "submitRetryable(bytes32,uint256,uint256,uint256,uint256,uint64,uint256,address,address,address,bytes)";



/* ArbAddressTable */
pub const SIG_AT_ADDRESS_EXISTS: &str = "addressExists(address)";
pub const SIG_AT_COMPRESS: &str = "compress(address)";
pub const SIG_AT_DECOMPRESS: &str = "decompress(bytes,uint256)";
pub const SIG_AT_LOOKUP: &str = "lookup(address)";
pub const SIG_AT_LOOKUP_INDEX: &str = "lookupIndex(uint256)";
pub const SIG_AT_REGISTER: &str = "register(address)";
pub const SIG_AT_SIZE: &str = "size()";

/* ArbGasInfo */
pub const SIG_GI_GET_PRICES_IN_WEI: &str = "getPricesInWei()";
pub const SIG_GI_GET_PRICES_IN_WEI_WITH_AGG: &str =
    "getPricesInWeiWithAggregator(address)";
pub const SIG_GI_GET_PRICES_IN_ARBGAS: &str = "getPricesInArbGas()";
pub const SIG_GI_GET_PRICES_IN_ARBGAS_WITH_AGG: &str =
    "getPricesInArbGasWithAggregator(address)";
pub const SIG_GI_GET_MIN_GAS_PRICE: &str = "getMinimumGasPrice()";
pub const SIG_GI_GET_L1_BASEFEE_ESTIMATE: &str = "getL1BaseFeeEstimate()";
pub const SIG_GI_GET_L1_BASEFEE_INERTIA: &str = "getL1BaseFeeEstimateInertia()";
pub const SIG_GI_GET_L1_REWARD_RATE: &str = "getL1RewardRate()";
pub const SIG_GI_GET_L1_REWARD_RECIPIENT: &str = "getL1RewardRecipient()";
pub const SIG_GI_GET_L1_GAS_PRICE_ESTIMATE: &str = "getL1GasPriceEstimate()";
pub const SIG_GI_GET_CURRENT_TX_L1_FEES: &str = "getCurrentTxL1GasFees()";

/* NodeInterface (virtual at 0xc8) */
pub const SIG_NI_ESTIMATE_RETRYABLE_TICKET: &str =
    "estimateRetryableTicket(address,uint256,address,uint256,address,address,bytes)";
pub const SIG_NI_CONSTRUCT_OUTBOX_PROOF: &str =
    "constructOutboxProof(uint64,uint64)";
pub const SIG_NI_FIND_BATCH_CONTAINING_BLOCK: &str =
    "findBatchContainingBlock(uint64)";
pub const SIG_NI_GET_L1_CONFIRMATIONS: &str = "getL1Confirmations(bytes32)";
pub const SIG_NI_GAS_ESTIMATE_COMPONENTS: &str =
    "gasEstimateComponents(address,bool,bytes)";
pub const SIG_NI_GAS_ESTIMATE_L1_COMPONENT: &str =
    "gasEstimateL1Component(address,bool,bytes)";
pub const SIG_NI_LEGACY_LOOKUP_MESSAGE_BATCH_PROOF: &str =
    "legacyLookupMessageBatchProof(uint256,uint64)";
pub const SIG_NI_NITRO_GENESIS_BLOCK: &str = "nitroGenesisBlock()";
pub const SIG_NI_BLOCK_L1_NUM: &str = "blockL1Num(uint64)";
pub const SIG_NI_L2_BLOCK_RANGE_FOR_L1: &str = "l2BlockRangeForL1(uint64)";

pub const EVT_TICKET_CREATED: &str =
    "TicketCreated(bytes32,address,uint256,uint256,address,address,uint256,uint256)";
pub const EVT_TICKET_REDEEMED: &str = "Redeemed(bytes32,address)";
pub const EVT_TICKET_CANCELED: &str = "Canceled(bytes32,address)";
pub const EVT_L2_TO_L1_TX: &str =
    "L2ToL1Transaction(address,address,uint256,uint256,uint256,uint256,bytes)";

pub fn signature_bytes(sig: &str) -> Vec<u8> {
    sig.as_bytes().to_vec()
}

pub const fn selector_for(sig_hash: [u8; 32]) -> [u8; 4] {
    [sig_hash[0], sig_hash[1], sig_hash[2], sig_hash[3]]
}

pub const fn topic_for(sig_hash: [u8; 32]) -> [u8; 32] {
    sig_hash
}

pub fn selector(sig: &str) -> [u8; 4] {
    selector_for(keccak256(sig.as_bytes()).0)
}

pub fn topic(sig: &str) -> [u8; 32] {
    topic_for(keccak256(sig.as_bytes()).0)
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
        assert_eq!(&ARB_SYS[12..], &[0, 0, 0, 0, 0, 0, 0, 0x64]);
        assert_eq!(&ARB_RETRYABLE_TX[12..], &[0, 0, 0, 0, 0, 0, 0, 0x6e]);
        assert_eq!(&ARB_OWNER[12..], &[0, 0, 0, 0, 0, 0, 0, 0x70]);
        assert_eq!(&NODE_INTERFACE[12..], &[0, 0, 0, 0, 0, 0, 0, 0xc8]);
        assert_eq!(&ARB_DEBUG[12..], &[0, 0, 0, 0, 0, 0, 0, 0xff]);
    }

    #[test]
    fn selectors_and_topics_are_derived() {
        let s = selector(SIG_SEND_TX_TO_L1);
        assert_eq!(s.len(), 4);
        let t = topic(EVT_TICKET_CREATED);
        assert_eq!(t.len(), 32);
        let sel_again = selector(SIG_SEND_TX_TO_L1);
        assert_eq!(s, sel_again);
        let topic_again = topic(EVT_TICKET_CREATED);
        assert_eq!(t, topic_again);
    }

    #[test]
    fn more_selectors_and_topics_are_derived_consistently() {
        let s1 = selector(SIG_ARB_BLOCK_NUMBER);
        let s1b = selector(SIG_ARB_BLOCK_NUMBER);
        assert_eq!(s1, s1b);
        let s2 = selector(SIG_ARB_BLOCK_HASH);
        let s2b = selector(SIG_ARB_BLOCK_HASH);
        assert_eq!(s2, s2b);

        let s3 = selector(SIG_GET_TX_CALL_VALUE);
        let s3b = selector(SIG_GET_TX_CALL_VALUE);
        assert_eq!(s3, s3b);

        let t1 = topic(EVT_L2_TO_L1_TX);
        let t1b = topic(EVT_L2_TO_L1_TX);
        assert_eq!(t1, t1b);
    }

    #[test]
    fn node_interface_and_gasinfo_selectors_compile() {
        for sig in [
            SIG_NI_ESTIMATE_RETRYABLE_TICKET,
            SIG_NI_GAS_ESTIMATE_COMPONENTS,
            SIG_NI_GAS_ESTIMATE_L1_COMPONENT,
            SIG_GI_GET_L1_BASEFEE_ESTIMATE,
            SIG_AT_REGISTER,
        ] {
            let sel = selector(sig);
            assert_eq!(sel.len(), 4);
        }
    }

    #[test]
    fn owner_selectors_compile() {
        for sig in [
            SIG_OWNER_ADD_CHAIN_OWNER,
            SIG_OWNER_REMOVE_CHAIN_OWNER,
            SIG_OWNER_IS_CHAIN_OWNER,
            SIG_OWNER_GET_ALL_CHAIN_OWNERS,
            SIG_OWNER_GET_NETWORK_FEE_ACCOUNT,
            SIG_OWNER_GET_INFRA_FEE_ACCOUNT,
            SIG_OWNER_SET_NETWORK_FEE_ACCOUNT,
            SIG_OWNER_SET_INFRA_FEE_ACCOUNT,
        ] {
            let sel = selector(sig);
            assert_eq!(sel.len(), 4);
        }
    }
}
