#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ArbTxType {
    ArbitrumDepositTx = 0x64,
    ArbitrumUnsignedTx = 0x65,
    ArbitrumContractTx = 0x66,
    ArbitrumRetryTx = 0x68,
    ArbitrumSubmitRetryableTx = 0x69,
    ArbitrumInternalTx = 0x6A,
    ArbitrumLegacyTx = 0x78,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArbTxEnvelope {
    Deposit,
    Unsigned,
    Contract,
    Retry,
    SubmitRetryable,
    Internal,
    Legacy,
}

impl ArbTxEnvelope {
    pub fn tx_type(&self) -> ArbTxType {
        match self {
            ArbTxEnvelope::Deposit => ArbTxType::ArbitrumDepositTx,
            ArbTxEnvelope::Unsigned => ArbTxType::ArbitrumUnsignedTx,
            ArbTxEnvelope::Contract => ArbTxType::ArbitrumContractTx,
            ArbTxEnvelope::Retry => ArbTxType::ArbitrumRetryTx,
            ArbTxEnvelope::SubmitRetryable => ArbTxType::ArbitrumSubmitRetryableTx,
            ArbTxEnvelope::Internal => ArbTxType::ArbitrumInternalTx,
            ArbTxEnvelope::Legacy => ArbTxType::ArbitrumLegacyTx,
        }
    }
}
