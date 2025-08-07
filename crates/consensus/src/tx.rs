#![allow(dead_code)]

use core::fmt;
use thiserror::Error;

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

impl ArbTxType {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
    pub fn from_u8(b: u8) -> Result<Self, TxTypeError> {
        match b {
            0x64 => Ok(Self::ArbitrumDepositTx),
            0x65 => Ok(Self::ArbitrumUnsignedTx),
            0x66 => Ok(Self::ArbitrumContractTx),
            0x68 => Ok(Self::ArbitrumRetryTx),
            0x69 => Ok(Self::ArbitrumSubmitRetryableTx),
            0x6A => Ok(Self::ArbitrumInternalTx),
            0x78 => Ok(Self::ArbitrumLegacyTx),
            _ => Err(TxTypeError::UnknownType(b)),
        }
    }
}

impl fmt::Display for ArbTxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ArbTxType::ArbitrumDepositTx => "ArbitrumDepositTx",
            ArbTxType::ArbitrumUnsignedTx => "ArbitrumUnsignedTx",
            ArbTxType::ArbitrumContractTx => "ArbitrumContractTx",
            ArbTxType::ArbitrumRetryTx => "ArbitrumRetryTx",
            ArbTxType::ArbitrumSubmitRetryableTx => "ArbitrumSubmitRetryableTx",
            ArbTxType::ArbitrumInternalTx => "ArbitrumInternalTx",
            ArbTxType::ArbitrumLegacyTx => "ArbitrumLegacyTx",
        };
        write!(f, "{}", name)
    }
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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TxTypeError {
    #[error("unknown Arbitrum tx type: {0:#x}")]
    UnknownType(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_tx_types() {
        let types = [
            ArbTxType::ArbitrumDepositTx,
            ArbTxType::ArbitrumUnsignedTx,
            ArbTxType::ArbitrumContractTx,
            ArbTxType::ArbitrumRetryTx,
            ArbTxType::ArbitrumSubmitRetryableTx,
            ArbTxType::ArbitrumInternalTx,
            ArbTxType::ArbitrumLegacyTx,
        ];
        for t in types {
            let b = t.as_u8();
            let back = ArbTxType::from_u8(b).unwrap();
            assert_eq!(t, back);
        }
    }

    #[test]
    fn unknown_type_errs() {
        assert!(matches!(ArbTxType::from_u8(0x01), Err(TxTypeError::UnknownType(0x01))));
    }
}
