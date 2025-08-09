#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;
use alloy_primitives::keccak256;

pub const ARB_SYS: [u8; 20] = hex20(0x64);
pub const ARB_ADDRESS_TABLE: [u8; 20] = hex20(0x66);
pub const ARB_BLS: [u8; 20] = hex20(0x67);
pub const ARB_FUNCTION_TABLE: [u8; 20] = hex20(0x68);
pub const ARB_INFO: [u8; 20] = hex20(0x65);
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
pub const SIG_WITHDRAW_ETH: &str = "withdrawEth(address)";
pub const SIG_SEND_TX_TO_L1: &str = "sendTxToL1(address,bytes)";
pub const SIG_ARB_BLOCK_NUMBER: &str = "arbBlockNumber()";
pub const SIG_ARB_BLOCK_HASH: &str = "arbBlockHash(uint256)";
pub const SIG_ARB_CHAIN_ID: &str = "arbChainID()";
pub const SIG_ARB_OS_VERSION: &str = "arbOSVersion()";
pub const SIG_GET_STORAGE_GAS_AVAILABLE: &str = "getStorageGasAvailable()";
pub const SIG_IS_TOP_LEVEL_CALL: &str = "isTopLevelCall()";
pub const SIG_MAP_L1_SENDER_TO_L2_ALIAS: &str =
    "mapL1SenderContractAddressToL2Alias(address,address)";
pub const SIG_WAS_MY_CALLERS_ADDRESS_ALIASED: &str = "wasMyCallersAddressAliased()";
pub const SIG_MY_CALLERS_ADDRESS_WITHOUT_ALIASING: &str = "myCallersAddressWithoutAliasing()";
pub const SIG_SEND_MERKLE_TREE_STATE: &str = "sendMerkleTreeState()";
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
pub const SIG_GI_GET_PRICES_IN_WEI_WITH_AGG: &str = "getPricesInWeiWithAggregator(address)";
pub const SIG_GI_GET_PRICES_IN_ARBGAS: &str = "getPricesInArbGas()";
pub const SIG_GI_GET_PRICES_IN_ARBGAS_WITH_AGG: &str = "getPricesInArbGasWithAggregator(address)";
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
pub const SIG_NI_CONSTRUCT_OUTBOX_PROOF: &str = "constructOutboxProof(uint64,uint64)";
pub const SIG_NI_FIND_BATCH_CONTAINING_BLOCK: &str = "findBatchContainingBlock(uint64)";
pub const SIG_NI_GET_L1_CONFIRMATIONS: &str = "getL1Confirmations(bytes32)";
pub const SIG_NI_GAS_ESTIMATE_COMPONENTS: &str = "gasEstimateComponents(address,bool,bytes)";
pub const SIG_NI_GAS_ESTIMATE_L1_COMPONENT: &str = "gasEstimateL1Component(address,bool,bytes)";
pub const SIG_NI_LEGACY_LOOKUP_MESSAGE_BATCH_PROOF: &str =
    "legacyLookupMessageBatchProof(uint256,uint64)";
pub const SIG_NI_NITRO_GENESIS_BLOCK: &str = "nitroGenesisBlock()";
pub const SIG_NI_BLOCK_L1_NUM: &str = "blockL1Num(uint64)";
/* NodeInterfaceDebug (virtual at 0xc9) */
pub const SIG_NID_RETRYABLE_DETAILS: &str = "getRetryable(bytes32)";
pub const SIG_NID_BLOOM_GAS_USED: &str = "bloomGasUsed(uint64)";
pub const SIG_NID_BLOOM_LOGS: &str = "bloomLogs(uint64)";
pub const SIG_NID_BLOOM_SIZE: &str = "bloomSize(uint64)";
pub const SIG_NID_BLOOM_STEPS: &str = "bloomBuildSteps(uint64)";
pub const SIG_NID_L2_TO_L1_LOGS: &str = "l2ToL1LogsHash(uint64)";
pub const SIG_NID_SEND_ROOT: &str = "sendRoot(uint64)";
pub const SIG_NID_SEND_ROOT_TIME: &str = "sendRootTime(uint64)";

pub const SIG_NI_L2_BLOCK_RANGE_FOR_L1: &str = "l2BlockRangeForL1(uint64)";

/* ArbInfo */
pub const SIG_INFO_GET_BALANCE: &str = "getBalance(address)";
pub const SIG_INFO_GET_CODE: &str = "getCode(address)";

/* ArbFunctionTable */
pub const SIG_FT_UPLOAD: &str = "upload(bytes)";
pub const SIG_FT_SIZE: &str = "size(address)";
pub const SIG_FT_GET: &str = "get(address,uint256)";

/* ArbStatistics */
pub const SIG_STATS_GET_STATS: &str = "getStats()";

/* ArbWasm */
pub const SIG_WASM_ACTIVATE_PROGRAM: &str = "activateProgram(address)";
pub const SIG_WASM_STYLUS_VERSION: &str = "stylusVersion()";
pub const SIG_WASM_CODEHASH_VERSION: &str = "codehashVersion(bytes32)";
pub const SIG_WASM_CODEHASH_KEEPALIVE: &str = "codehashKeepalive(bytes32)";
pub const SIG_WASM_CODEHASH_ASM_SIZE: &str = "codehashAsmSize(bytes32)";
pub const SIG_WASM_PROGRAM_VERSION: &str = "programVersion(address)";
pub const SIG_WASM_PROGRAM_INIT_GAS: &str = "programInitGas(address)";
pub const SIG_WASM_PROGRAM_MEMORY_FOOTPRINT: &str = "programMemoryFootprint(address)";
pub const SIG_WASM_PROGRAM_TIME_LEFT: &str = "programTimeLeft(address)";
pub const SIG_WASM_INK_PRICE: &str = "inkPrice()";
pub const SIG_WASM_MAX_STACK_DEPTH: &str = "maxStackDepth()";
pub const SIG_WASM_FREE_PAGES: &str = "freePages()";
pub const SIG_WASM_PAGE_GAS: &str = "pageGas()";
pub const SIG_WASM_PAGE_RAMP: &str = "pageRamp()";
pub const SIG_WASM_PAGE_LIMIT: &str = "pageLimit()";
pub const SIG_WASM_MIN_INIT_GAS: &str = "minInitGas()";
pub const SIG_WASM_INIT_COST_SCALAR: &str = "initCostScalar()";
pub const SIG_WASM_EXPIRY_DAYS: &str = "expiryDays()";
pub const SIG_WASM_KEEPALIVE_DAYS: &str = "keepaliveDays()";
pub const SIG_WASM_BLOCK_CACHE_SIZE: &str = "blockCacheSize()";

/* ArbWasmCache */
pub const SIG_WASMC_IS_CACHE_MANAGER: &str = "isCacheManager(address)";
pub const SIG_WASMC_ALL_CACHE_MANAGERS: &str = "allCacheManagers()";
pub const SIG_WASMC_CACHE_CODEHASH: &str = "cacheCodehash(bytes32)";
pub const SIG_WASMC_CACHE_PROGRAM: &str = "cacheProgram(address)";
pub const SIG_WASMC_EVICT_CODEHASH: &str = "evictCodehash(bytes32)";
pub const SIG_WASMC_CODEHASH_IS_CACHED: &str = "codehashIsCached(bytes32)";

/* ArbNativeTokenManager */
pub const SIG_NTM_MINT_NATIVE_TOKEN: &str = "mintNativeToken(uint256)";
pub const SIG_NTM_BURN_NATIVE_TOKEN: &str = "burnNativeToken(uint256)";

/* ArbOwnerPublic */
pub const SIG_OWNER_PUB_IS_CHAIN_OWNER: &str = "isChainOwner(address)";
pub const SIG_OWNER_PUB_RECTIFY_CHAIN_OWNER: &str = "rectifyChainOwner(address)";
pub const SIG_OWNER_PUB_GET_ALL_CHAIN_OWNERS: &str = "getAllChainOwners()";
pub const SIG_OWNER_PUB_IS_NATIVE_TOKEN_OWNER: &str = "isNativeTokenOwner(address)";
pub const SIG_OWNER_PUB_GET_ALL_NATIVE_TOKEN_OWNERS: &str = "getAllNativeTokenOwners()";
pub const SIG_OWNER_PUB_GET_NETWORK_FEE_ACCOUNT: &str = "getNetworkFeeAccount()";
pub const SIG_OWNER_PUB_GET_INFRA_FEE_ACCOUNT: &str = "getInfraFeeAccount()";
pub const SIG_OWNER_PUB_GET_BROTLI_COMPRESSION_LEVEL: &str = "getBrotliCompressionLevel()";
pub const SIG_OWNER_PUB_GET_SCHEDULED_UPGRADE: &str = "getScheduledUpgrade()";
pub const SIG_OWNER_PUB_IS_CALLDATA_PRICE_INCREASE_ENABLED: &str =
    "isCalldataPriceIncreaseEnabled()";

pub const EVT_TICKET_CREATED: &str = "TicketCreated(bytes32)";
pub const EVT_LIFETIME_EXTENDED: &str = "LifetimeExtended(bytes32,uint256)";
pub const EVT_REDEEM_SCHEDULED: &str =
    "RedeemScheduled(bytes32,bytes32,uint64,uint64,address,uint256,uint256)";
pub const EVT_TICKET_CANCELED: &str = "Canceled(bytes32)";
pub const EVT_REDEEMED_DEPRECATED: &str = "Redeemed(bytes32)";

/* ArbSys events */
pub const EVT_L2_TO_L1_TX: &str =
    "L2ToL1Tx(address,address,uint256,uint256,uint256,uint256,uint256,bytes)";
pub const EVT_L2_TO_L1_TRANSACTION_DEPRECATED: &str =
    "L2ToL1Transaction(address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bytes)";
pub const EVT_SEND_MERKLE_UPDATE: &str = "SendMerkleUpdate(uint256,bytes32,uint256)";

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

        let s3 = selector(SIG_GET_STORAGE_GAS_AVAILABLE);
        let s3b = selector(SIG_GET_STORAGE_GAS_AVAILABLE);
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
