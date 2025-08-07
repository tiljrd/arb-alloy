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

    pub fn encode_typed_minimal(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(2);
        out.push(self.tx_type().as_u8());
        out.push(0xC0);
        out
    }

    pub fn decode_typed_minimal(bytes: &[u8]) -> Result<(Self, usize), TxTypeError> {
        if bytes.len() < 2 {
            return Err(TxTypeError::UnknownType(0xff));
        }
        let ty = ArbTxType::from_u8(bytes[0])?;
        if bytes[1] != 0xC0 {
            return Err(TxTypeError::UnknownType(bytes[0]));
        }
        let env = match ty {
            ArbTxType::ArbitrumDepositTx => ArbTxEnvelope::Deposit,
            ArbTxType::ArbitrumUnsignedTx => ArbTxEnvelope::Unsigned,
            ArbTxType::ArbitrumContractTx => ArbTxEnvelope::Contract,
            ArbTxType::ArbitrumRetryTx => ArbTxEnvelope::Retry,
            ArbTxType::ArbitrumSubmitRetryableTx => ArbTxEnvelope::SubmitRetryable,
            ArbTxType::ArbitrumInternalTx => ArbTxEnvelope::Internal,
            ArbTxType::ArbitrumLegacyTx => ArbTxEnvelope::Legacy,
        };
        Ok((env, 2))
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

    #[test]
    fn typed_minimal_roundtrip() {
        let envs = [
            ArbTxEnvelope::Deposit,
            ArbTxEnvelope::Unsigned,
            ArbTxEnvelope::Contract,
            ArbTxEnvelope::Retry,
            ArbTxEnvelope::SubmitRetryable,
            ArbTxEnvelope::Internal,
            ArbTxEnvelope::Legacy,
        ];
        for e in envs {
            let enc = e.encode_typed_minimal();
            assert_eq!(enc.len(), 2);
            let (dec, used) = ArbTxEnvelope::decode_typed_minimal(&enc).unwrap();
            assert_eq!(used, 2);
            assert_eq!(dec, e);
        }
    }
}
