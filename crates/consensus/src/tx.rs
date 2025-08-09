#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;
use alloy_primitives::{Address, B256, U256};
use alloy_rlp::{Decodable, Encodable, RlpDecodable, RlpEncodable};
use core::fmt;
use thiserror::Error;
fn decode_option_address(buf: &mut &[u8]) -> Result<Option<Address>, alloy_rlp::Error> {
    if let Some(&first) = buf.first() {
        if first == alloy_rlp::EMPTY_STRING_CODE {
            *buf = &buf[1..];
            return Ok(None);
        }
    }
    let addr: Address = Decodable::decode(buf)?;
    Ok(Some(addr))
}

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
pub struct ArbUnsignedTx {
    pub chain_id: U256,
    pub from: Address,
    pub nonce: u64,
    pub gas_fee_cap: U256,
    pub gas: u64,
    pub to: Option<Address>,
    pub value: U256,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbContractTx {
    pub chain_id: U256,
    pub request_id: B256,
    pub from: Address,
    pub gas_fee_cap: U256,
    pub gas: u64,
    pub to: Option<Address>,
    pub value: U256,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbRetryTx {
    pub chain_id: U256,
    pub nonce: u64,
    pub from: Address,
    pub gas_fee_cap: U256,
    pub gas: u64,
    pub to: Option<Address>,
    pub value: U256,
    pub data: Vec<u8>,
    pub ticket_id: B256,
    pub refund_to: Address,
    pub max_refund: U256,
    pub submission_fee_refund: U256,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbSubmitRetryableTx {
    pub chain_id: U256,
    pub request_id: B256,
    pub from: Address,
    pub l1_base_fee: U256,
    pub deposit_value: U256,
    pub gas_fee_cap: U256,
    pub gas: u64,
    pub retry_to: Option<Address>,
    pub retry_value: U256,
    pub beneficiary: Address,
    pub max_submission_fee: U256,
    pub fee_refund_addr: Address,
    pub retry_data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct ArbDepositTx {
    pub chain_id: U256,
    pub l1_request_id: B256,
    pub from: Address,
    pub to: Address,
    pub value: U256,
}

#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct ArbInternalTx {
    pub chain_id: U256,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArbTxEnvelope {
    Deposit(ArbDepositTx),
    Unsigned(ArbUnsignedTx),
    Contract(ArbContractTx),
    Retry(ArbRetryTx),
    SubmitRetryable(ArbSubmitRetryableTx),
    Internal(ArbInternalTx),
    Legacy(Vec<u8>),
}

impl Encodable for ArbUnsignedTx {
    fn length(&self) -> usize {
        let mut payload = 0usize;
        payload += self.chain_id.length();
        payload += self.from.length();
        payload += self.nonce.length();
        payload += self.gas_fee_cap.length();
        payload += self.gas.length();
        payload += match self.to {
            Some(a) => a.length(),
            None => 1,
        };
        payload += self.value.length();
        payload += self.data.length();
        alloy_rlp::length_of_length(payload) + payload + 1
    }
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        let mut payload = 0usize;
        payload += self.chain_id.length();
        payload += self.from.length();
        payload += self.nonce.length();
        payload += self.gas_fee_cap.length();
        payload += self.gas.length();
        payload += match self.to {
            Some(a) => a.length(),
            None => 1,
        };
        payload += self.value.length();
        payload += self.data.length();
        let header = alloy_rlp::Header {
            list: true,
            payload_length: payload,
        };
        header.encode(out);
        self.chain_id.encode(out);
        self.from.encode(out);
        self.nonce.encode(out);
        self.gas_fee_cap.encode(out);
        self.gas.encode(out);
        match self.to {
            Some(a) => a.encode(out),
            None => out.put_slice(&[alloy_rlp::EMPTY_STRING_CODE]),
        }
        self.value.encode(out);
        self.data.encode(out);
    }
}
impl Decodable for ArbUnsignedTx {
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let header = alloy_rlp::Header::decode(buf)?;
        let (payload, rest) = buf.split_at(header.payload_length);
        let mut p = payload;
        let chain_id: U256 = Decodable::decode(&mut p)?;
        let from: Address = Decodable::decode(&mut p)?;
        let nonce: u64 = Decodable::decode(&mut p)?;
        let gas_fee_cap: U256 = Decodable::decode(&mut p)?;
        let gas: u64 = Decodable::decode(&mut p)?;
        let to: Option<Address> = decode_option_address(&mut p)?;
        let value: U256 = Decodable::decode(&mut p)?;
        let data: Vec<u8> = Decodable::decode(&mut p)?;
        *buf = rest;
        Ok(ArbUnsignedTx {
            chain_id,
            from,
            nonce,
            gas_fee_cap,
            gas,
            to,
            value,
            data,
        })
    }
}

impl Encodable for ArbContractTx {
    fn length(&self) -> usize {
        let mut payload = 0usize;
        payload += self.chain_id.length();
        payload += self.request_id.length();
        payload += self.from.length();
        payload += self.gas_fee_cap.length();
        payload += self.gas.length();
        payload += match self.to {
            Some(a) => a.length(),
            None => 1,
        };
        payload += self.value.length();
        payload += self.data.length();
        alloy_rlp::length_of_length(payload) + payload + 1
    }
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        let mut payload = 0usize;
        payload += self.chain_id.length();
        payload += self.request_id.length();
        payload += self.from.length();
        payload += self.gas_fee_cap.length();
        payload += self.gas.length();
        payload += match self.to {
            Some(a) => a.length(),
            None => 1,
        };
        payload += self.value.length();
        payload += self.data.length();
        let header = alloy_rlp::Header {
            list: true,
            payload_length: payload,
        };
        header.encode(out);
        self.chain_id.encode(out);
        self.request_id.encode(out);
        self.from.encode(out);
        self.gas_fee_cap.encode(out);
        self.gas.encode(out);
        match self.to {
            Some(a) => a.encode(out),
            None => out.put_slice(&[alloy_rlp::EMPTY_STRING_CODE]),
        }
        self.value.encode(out);
        self.data.encode(out);
    }
}
impl Decodable for ArbContractTx {
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let header = alloy_rlp::Header::decode(buf)?;
        let (payload, rest) = buf.split_at(header.payload_length);
        let mut p = payload;
        let chain_id: U256 = Decodable::decode(&mut p)?;
        let request_id: B256 = Decodable::decode(&mut p)?;
        let from: Address = Decodable::decode(&mut p)?;
        let gas_fee_cap: U256 = Decodable::decode(&mut p)?;
        let gas: u64 = Decodable::decode(&mut p)?;
        let to: Option<Address> = decode_option_address(&mut p)?;
        let value: U256 = Decodable::decode(&mut p)?;
        let data: Vec<u8> = Decodable::decode(&mut p)?;
        *buf = rest;
        Ok(ArbContractTx {
            chain_id,
            request_id,
            from,
            gas_fee_cap,
            gas,
            to,
            value,
            data,
        })
    }
}

impl Encodable for ArbRetryTx {
    fn length(&self) -> usize {
        let mut payload = 0usize;
        payload += self.chain_id.length();
        payload += self.nonce.length();
        payload += self.from.length();
        payload += self.gas_fee_cap.length();
        payload += self.gas.length();
        payload += match self.to {
            Some(a) => a.length(),
            None => 1,
        };
        payload += self.value.length();
        payload += self.data.length();
        payload += self.ticket_id.length();
        payload += self.refund_to.length();
        payload += self.max_refund.length();
        payload += self.submission_fee_refund.length();
        alloy_rlp::length_of_length(payload) + payload + 1
    }
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        let mut payload = 0usize;
        payload += self.chain_id.length();
        payload += self.nonce.length();
        payload += self.from.length();
        payload += self.gas_fee_cap.length();
        payload += self.gas.length();
        payload += match self.to {
            Some(a) => a.length(),
            None => 1,
        };
        payload += self.value.length();
        payload += self.data.length();
        payload += self.ticket_id.length();
        payload += self.refund_to.length();
        payload += self.max_refund.length();
        payload += self.submission_fee_refund.length();
        let header = alloy_rlp::Header {
            list: true,
            payload_length: payload,
        };
        header.encode(out);
        self.chain_id.encode(out);
        self.nonce.encode(out);
        self.from.encode(out);
        self.gas_fee_cap.encode(out);
        self.gas.encode(out);
        match self.to {
            Some(a) => a.encode(out),
            None => out.put_slice(&[alloy_rlp::EMPTY_STRING_CODE]),
        }
        self.value.encode(out);
        self.data.encode(out);
        self.ticket_id.encode(out);
        self.refund_to.encode(out);
        self.max_refund.encode(out);
        self.submission_fee_refund.encode(out);
    }
}
impl Decodable for ArbRetryTx {
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let header = alloy_rlp::Header::decode(buf)?;
        let (payload, rest) = buf.split_at(header.payload_length);
        let mut p = payload;
        let chain_id: U256 = Decodable::decode(&mut p)?;
        let nonce: u64 = Decodable::decode(&mut p)?;
        let from: Address = Decodable::decode(&mut p)?;
        let gas_fee_cap: U256 = Decodable::decode(&mut p)?;
        let gas: u64 = Decodable::decode(&mut p)?;
        let to: Option<Address> = decode_option_address(&mut p)?;
        let value: U256 = Decodable::decode(&mut p)?;
        let data: Vec<u8> = Decodable::decode(&mut p)?;
        let ticket_id: B256 = Decodable::decode(&mut p)?;
        let refund_to: Address = Decodable::decode(&mut p)?;
        let max_refund: U256 = Decodable::decode(&mut p)?;
        let submission_fee_refund: U256 = Decodable::decode(&mut p)?;
        *buf = rest;
        Ok(ArbRetryTx {
            chain_id,
            nonce,
            from,
            gas_fee_cap,
            gas,
            to,
            value,
            data,
            ticket_id,
            refund_to,
            max_refund,
            submission_fee_refund,
        })
    }
}

impl Encodable for ArbSubmitRetryableTx {
    fn length(&self) -> usize {
        let mut payload = 0usize;
        payload += self.chain_id.length();
        payload += self.request_id.length();
        payload += self.from.length();
        payload += self.l1_base_fee.length();
        payload += self.deposit_value.length();
        payload += self.gas_fee_cap.length();
        payload += self.gas.length();
        payload += match self.retry_to {
            Some(a) => a.length(),
            None => 1,
        };
        payload += self.retry_value.length();
        payload += self.beneficiary.length();
        payload += self.max_submission_fee.length();
        payload += self.fee_refund_addr.length();
        payload += self.retry_data.length();
        alloy_rlp::length_of_length(payload) + payload + 1
    }
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        let mut payload = 0usize;
        payload += self.chain_id.length();
        payload += self.request_id.length();
        payload += self.from.length();
        payload += self.l1_base_fee.length();
        payload += self.deposit_value.length();
        payload += self.gas_fee_cap.length();
        payload += self.gas.length();
        payload += match self.retry_to {
            Some(a) => a.length(),
            None => 1,
        };
        payload += self.retry_value.length();
        payload += self.beneficiary.length();
        payload += self.max_submission_fee.length();
        payload += self.fee_refund_addr.length();
        payload += self.retry_data.length();
        let header = alloy_rlp::Header {
            list: true,
            payload_length: payload,
        };
        header.encode(out);
        self.chain_id.encode(out);
        self.request_id.encode(out);
        self.from.encode(out);
        self.l1_base_fee.encode(out);
        self.deposit_value.encode(out);
        self.gas_fee_cap.encode(out);
        self.gas.encode(out);
        match self.retry_to {
            Some(a) => a.encode(out),
            None => out.put_slice(&[alloy_rlp::EMPTY_STRING_CODE]),
        }
        self.retry_value.encode(out);
        self.beneficiary.encode(out);
        self.max_submission_fee.encode(out);
        self.fee_refund_addr.encode(out);
        self.retry_data.encode(out);
    }
}
impl Decodable for ArbSubmitRetryableTx {
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let header = alloy_rlp::Header::decode(buf)?;
        let (payload, rest) = buf.split_at(header.payload_length);
        let mut p = payload;
        let chain_id: U256 = Decodable::decode(&mut p)?;
        let request_id: B256 = Decodable::decode(&mut p)?;
        let from: Address = Decodable::decode(&mut p)?;
        let l1_base_fee: U256 = Decodable::decode(&mut p)?;
        let deposit_value: U256 = Decodable::decode(&mut p)?;
        let gas_fee_cap: U256 = Decodable::decode(&mut p)?;
        let gas: u64 = Decodable::decode(&mut p)?;
        let retry_to: Option<Address> = decode_option_address(&mut p)?;
        let retry_value: U256 = Decodable::decode(&mut p)?;
        let beneficiary: Address = Decodable::decode(&mut p)?;
        let max_submission_fee: U256 = Decodable::decode(&mut p)?;
        let fee_refund_addr: Address = Decodable::decode(&mut p)?;
        let retry_data: Vec<u8> = Decodable::decode(&mut p)?;
        *buf = rest;
        Ok(ArbSubmitRetryableTx {
            chain_id,
            request_id,
            from,
            l1_base_fee,
            deposit_value,
            gas_fee_cap,
            gas,
            retry_to,
            retry_value,
            beneficiary,
            max_submission_fee,
            fee_refund_addr,
            retry_data,
        })
    }
}

impl ArbTxEnvelope {
    pub fn tx_type(&self) -> ArbTxType {
        match self {
            ArbTxEnvelope::Deposit(_) => ArbTxType::ArbitrumDepositTx,
            ArbTxEnvelope::Unsigned(_) => ArbTxType::ArbitrumUnsignedTx,
            ArbTxEnvelope::Contract(_) => ArbTxType::ArbitrumContractTx,
            ArbTxEnvelope::Retry(_) => ArbTxType::ArbitrumRetryTx,
            ArbTxEnvelope::SubmitRetryable(_) => ArbTxType::ArbitrumSubmitRetryableTx,
            ArbTxEnvelope::Internal(_) => ArbTxType::ArbitrumInternalTx,
            ArbTxEnvelope::Legacy(_) => ArbTxType::ArbitrumLegacyTx,
        }
    }

 spi    pub fn encode_typed(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(self.tx_type().as_u8());
        match self {
            ArbTxEnvelope::Deposit(p) => p.encode(&mut out),
            ArbTxEnvelope::Unsigned(p) => p.encode(&mut out),
            ArbTxEnvelope::Contract(p) => p.encode(&mut out),
            ArbTxEnvelope::Retry(p) => p.encode(&mut out),
            ArbTxEnvelope::SubmitRetryable(p) => p.encode(&mut out),
            ArbTxEnvelope::Internal(p) => p.encode(&mut out),
            ArbTxEnvelope::Legacy(payload) => out.extend_from_slice(payload),
        }
        out
    }

    pub fn decode_typed(bytes: &[u8]) -> Result<(Self, usize), TxTypeError> {
        if bytes.len() < 2 {
            return Err(TxTypeError::UnknownType(0xff));
        }
        let ty = ArbTxType::from_u8(bytes[0])?;
        let payload = &bytes[1..];
        match ty {
            ArbTxType::ArbitrumDepositTx => {
                let (val, used) = ArbDepositTx::decode_with_used(payload)?;
                return Ok((ArbTxEnvelope::Deposit(val), used + 1));
            }
            ArbTxType::ArbitrumUnsignedTx => {
                let (val, used) = ArbUnsignedTx::decode_with_used(payload)?;
                return Ok((ArbTxEnvelope::Unsigned(val), used + 1));
            }
            ArbTxType::ArbitrumContractTx => {
                let (val, used) = ArbContractTx::decode_with_used(payload)?;
                return Ok((ArbTxEnvelope::Contract(val), used + 1));
            }
            ArbTxType::ArbitrumRetryTx => {
                let (val, used) = ArbRetryTx::decode_with_used(payload)?;
                return Ok((ArbTxEnvelope::Retry(val), used + 1));
            }
            ArbTxType::ArbitrumSubmitRetryableTx => {
                let (val, used) = ArbSubmitRetryableTx::decode_with_used(payload)?;
                return Ok((ArbTxEnvelope::SubmitRetryable(val), used + 1));
            }
            ArbTxType::ArbitrumInternalTx => {
                let (val, used) = ArbInternalTx::decode_with_used(payload)?;
                return Ok((ArbTxEnvelope::Internal(val), used + 1));
            }
            ArbTxType::ArbitrumLegacyTx => {
                return Ok((ArbTxEnvelope::Legacy(payload.to_vec()), bytes.len()));
            }
        };
    }
}

trait RlpDecodeWithUsed: Sized + Decodable {
    fn decode_with_used(bytes: &[u8]) -> Result<(Self, usize), TxTypeError> {
        let mut s = bytes;
        let before = s.len();
        let val = <Self as Decodable>::decode(&mut s).map_err(|_| TxTypeError::Decode)?;
        let used = before - s.len();
        Ok((val, used))
    }
}
impl RlpDecodeWithUsed for ArbDepositTx {}
impl RlpDecodeWithUsed for ArbUnsignedTx {}
impl RlpDecodeWithUsed for ArbContractTx {}
impl RlpDecodeWithUsed for ArbRetryTx {}
impl RlpDecodeWithUsed for ArbSubmitRetryableTx {}
impl RlpDecodeWithUsed for ArbInternalTx {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TxTypeError {
    #[error("unknown Arbitrum tx type: {0:#x}")]
    UnknownType(u8),
    #[error("RLP decode error")]
    Decode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, b256, U256};

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
    #[test]
    fn exact_type_bytes_match_nitro_spec() {
        assert_eq!(ArbTxType::ArbitrumDepositTx.as_u8(), 0x64);
        assert_eq!(ArbTxType::ArbitrumUnsignedTx.as_u8(), 0x65);
        assert_eq!(ArbTxType::ArbitrumContractTx.as_u8(), 0x66);
        assert_eq!(ArbTxType::ArbitrumRetryTx.as_u8(), 0x68);
        assert_eq!(ArbTxType::ArbitrumSubmitRetryableTx.as_u8(), 0x69);
        assert_eq!(ArbTxType::ArbitrumInternalTx.as_u8(), 0x6a);
        assert_eq!(ArbTxType::ArbitrumLegacyTx.as_u8(), 0x78);

        assert_eq!(ArbTxType::from_u8(0x64).unwrap(), ArbTxType::ArbitrumDepositTx);
        assert_eq!(ArbTxType::from_u8(0x65).unwrap(), ArbTxType::ArbitrumUnsignedTx);
        assert_eq!(ArbTxType::from_u8(0x66).unwrap(), ArbTxType::ArbitrumContractTx);
        assert_eq!(ArbTxType::from_u8(0x68).unwrap(), ArbTxType::ArbitrumRetryTx);
        assert_eq!(ArbTxType::from_u8(0x69).unwrap(), ArbTxType::ArbitrumSubmitRetryableTx);
        assert_eq!(ArbTxType::from_u8(0x6a).unwrap(), ArbTxType::ArbitrumInternalTx);
        assert_eq!(ArbTxType::from_u8(0x78).unwrap(), ArbTxType::ArbitrumLegacyTx);
    }

    }

    #[test]
    fn typed_envelope_roundtrip_per_variant() {
        let dep = ArbTxEnvelope::Deposit(ArbDepositTx {
            chain_id: U256::from(42161u64),
            l1_request_id: b256!(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            ),
            from: address!("0000000000000000000000000000000000000001"),
            to: address!("0000000000000000000000000000000000000002"),
            value: U256::from(1u64),
        });
        let uns = ArbTxEnvelope::Unsigned(ArbUnsignedTx {
            chain_id: U256::from(42161u64),
            from: address!("0000000000000000000000000000000000000003"),
            nonce: 7,
            gas_fee_cap: U256::from(1000u64),
            gas: 21000,
            to: None,
            value: U256::from(0u64),
            data: Vec::new(),
        });
        let con = ArbTxEnvelope::Contract(ArbContractTx {
            chain_id: U256::from(42161u64),
            request_id: b256!("0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20"),
            from: address!("0000000000000000000000000000000000000004"),
            gas_fee_cap: U256::from(1000),
            gas: 100000,
            to: Some(address!("0000000000000000000000000000000000000005")),
            value: U256::from(123),
            data: vec![0xaa, 0xbb],
        });
        let rty = ArbTxEnvelope::Retry(ArbRetryTx {
            chain_id: U256::from(42161u64),
            nonce: 1,
            from: address!("0000000000000000000000000000000000000006"),
            gas_fee_cap: U256::from(1000),
            gas: 50000,
            to: None,
            value: U256::ZERO,
            data: vec![],
            ticket_id: b256!("2222222222222222222222222222222222222222222222222222222222222222"),
            refund_to: address!("0000000000000000000000000000000000000007"),
            max_refund: U256::from(10),
            submission_fee_refund: U256::from(2),
        });
        let srt = ArbTxEnvelope::SubmitRetryable(ArbSubmitRetryableTx {
            chain_id: U256::from(42161u64),
            request_id: b256!("3333333333333333333333333333333333333333333333333333333333333333"),
            from: address!("0000000000000000000000000000000000000008"),
            l1_base_fee: U256::from(30),
            deposit_value: U256::from(1000),
            gas_fee_cap: U256::from(100),
            gas: 60000,
            retry_to: Some(address!("0000000000000000000000000000000000000009")),
            retry_value: U256::from(123),
            beneficiary: address!("0000000000000000000000000000000000000010"),
            max_submission_fee: U256::from(55),
            fee_refund_addr: address!("0000000000000000000000000000000000000011"),
            retry_data: vec![0xde, 0xad, 0xbe, 0xef],
        });
        let itx = ArbTxEnvelope::Internal(ArbInternalTx {
            chain_id: U256::from(42161u64),
            data: vec![0x01],
        });

        for env in [dep, uns, con, rty, srt, itx] {
            let enc = env.encode_typed();
            assert_eq!(enc[0], env.tx_type().as_u8());
            let (dec, _used) = ArbTxEnvelope::decode_typed(&enc).unwrap();
            assert_eq!(dec, env);
        }
    }
    #[test]
    fn roundtrip_deposit_only() {
        let env = ArbTxEnvelope::Deposit(ArbDepositTx {
            chain_id: U256::from(42161u64),
            l1_request_id: b256!(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            ),
            from: address!("0000000000000000000000000000000000000001"),
            to: address!("0000000000000000000000000000000000000002"),
            value: U256::from(1u64),
        });
        let enc = env.encode_typed();
        let (dec, _used) = ArbTxEnvelope::decode_typed(&enc).unwrap();
        assert_eq!(dec, env);
    }

    #[test]
    fn roundtrip_unsigned_only() {
        let env = ArbTxEnvelope::Unsigned(ArbUnsignedTx {
            chain_id: U256::from(42161u64),
            from: address!("0000000000000000000000000000000000000003"),
            nonce: 7,
            gas_fee_cap: U256::from(1000u64),
            gas: 21000,
            to: None,
            value: U256::from(0u64),
            data: Vec::new(),
        });
        let enc = env.encode_typed();
        let (dec, _used) = ArbTxEnvelope::decode_typed(&enc).unwrap();
        assert_eq!(dec, env);
    }

    #[test]
    fn roundtrip_contract_only() {
        let env = ArbTxEnvelope::Contract(ArbContractTx {
            chain_id: U256::from(42161u64),
            request_id: b256!("0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20"),
            from: address!("0000000000000000000000000000000000000004"),
            gas_fee_cap: U256::from(1000),
            gas: 100000,
            to: Some(address!("0000000000000000000000000000000000000005")),
            value: U256::from(123),
            data: vec![0xaa, 0xbb],
        });
        let enc = env.encode_typed();
        let (dec, _used) = ArbTxEnvelope::decode_typed(&enc).unwrap();
        assert_eq!(dec, env);
    }

    #[test]
    fn roundtrip_retry_only() {
        let env = ArbTxEnvelope::Retry(ArbRetryTx {
            chain_id: U256::from(42161u64),
            nonce: 1,
            from: address!("0000000000000000000000000000000000000006"),
            gas_fee_cap: U256::from(1000),
            gas: 50000,
            to: None,
            value: U256::ZERO,
            data: vec![],
            ticket_id: b256!("2222222222222222222222222222222222222222222222222222222222222222"),
            refund_to: address!("0000000000000000000000000000000000000007"),
            max_refund: U256::from(10),
            submission_fee_refund: U256::from(2),
        });
        let enc = env.encode_typed();
        let (dec, _used) = ArbTxEnvelope::decode_typed(&enc).unwrap();
        assert_eq!(dec, env);
    }

    #[test]
    fn roundtrip_submit_retryable_only() {
        let env = ArbTxEnvelope::SubmitRetryable(ArbSubmitRetryableTx {
            chain_id: U256::from(42161u64),
            request_id: b256!("3333333333333333333333333333333333333333333333333333333333333333"),
            from: address!("0000000000000000000000000000000000000008"),
            l1_base_fee: U256::from(30),
            deposit_value: U256::from(1000),
            gas_fee_cap: U256::from(100),
            gas: 60000,
            retry_to: Some(address!("0000000000000000000000000000000000000009")),
            retry_value: U256::from(123),
            beneficiary: address!("0000000000000000000000000000000000000010"),
            max_submission_fee: U256::from(55),
            fee_refund_addr: address!("0000000000000000000000000000000000000011"),
            retry_data: vec![0xde, 0xad, 0xbe, 0xef],
        });
        let enc = env.encode_typed();
        let (dec, _used) = ArbTxEnvelope::decode_typed(&enc).unwrap();
        assert_eq!(dec, env);
    }

    #[test]
    fn roundtrip_internal_only() {
        let env = ArbTxEnvelope::Internal(ArbInternalTx {
            chain_id: U256::from(42161u64),
            data: vec![0x01],
        });
        let enc = env.encode_typed();
        let (dec, _used) = ArbTxEnvelope::decode_typed(&enc).unwrap();
        assert_eq!(dec, env);
    }
    #[test]
    fn decode_unsigned_direct() {
        let env = ArbTxEnvelope::Unsigned(ArbUnsignedTx {
            chain_id: U256::from(42161u64),
            from: address!("0000000000000000000000000000000000000003"),
            nonce: 7,
            gas_fee_cap: U256::from(1000u64),
            gas: 21000,
            to: None,
            value: U256::from(0u64),
            data: Vec::new(),
        });
        let enc = env.encode_typed();
        let payload = &enc[1..];
        let (dec, used) = ArbUnsignedTx::decode_with_used(payload).expect("unsigned decode");
        assert_eq!(used, payload.len());
        assert_eq!(
            dec,
            match env {
                ArbTxEnvelope::Unsigned(u) => u,
                _ => unreachable!(),
            }
        );
    }

    #[test]
    fn decode_retry_direct() {
        let env = ArbTxEnvelope::Retry(ArbRetryTx {
            chain_id: U256::from(42161u64),
            nonce: 1,
            from: address!("0000000000000000000000000000000000000006"),
            gas_fee_cap: U256::from(1000),
            gas: 50000,
            to: None,
            value: U256::ZERO,
            data: vec![],
            ticket_id: b256!("2222222222222222222222222222222222222222222222222222222222222222"),
            refund_to: address!("0000000000000000000000000000000000000007"),
            max_refund: U256::from(10),
            submission_fee_refund: U256::from(2),
        });
        let enc = env.encode_typed();
        let payload = &enc[1..];
        let (dec, used) = ArbRetryTx::decode_with_used(payload).expect("retry decode");
        assert_eq!(used, payload.len());
        assert_eq!(
            dec,
            match env {
                ArbTxEnvelope::Retry(r) => r,
                _ => unreachable!(),
            }
        );
    }
}
    #[test]
    fn decode_typed_rejects_unknown_type() {
        let mut bad = vec![0xff, 0xc0]; // invalid type byte + minimal payload
        assert!(matches!(
            ArbTxEnvelope::decode_typed(&bad),
            Err(TxTypeError::UnknownType(0xff))
        ));
        bad[0] = 0x00; // not an Arbitrum type in this module
        assert!(matches!(
            ArbTxEnvelope::decode_typed(&bad),
            Err(TxTypeError::UnknownType(0x00))
        ));
    }

    #[test]
    fn legacy_passthrough_reports_full_length() {
        let legacy_payload: Vec<u8> = alloy_rlp::encode(alloy_primitives::U256::from(7u64));
        let mut bytes = Vec::with_capacity(1 + legacy_payload.len());
        bytes.push(ArbTxType::ArbitrumLegacyTx.as_u8());
        bytes.extend_from_slice(&legacy_payload);
        let (env, used) = ArbTxEnvelope::decode_typed(&bytes).expect("decode");
        assert!(matches!(env, ArbTxEnvelope::Legacy(_)));
        assert_eq!(used, bytes.len(), "legacy decode should consume full input");
    }
#[cfg(test)]
mod proptests {
    use super::*;
    use alloc::vec::Vec;
    use proptest::prelude::*;

    fn arb_address() -> impl Strategy<Value = Address> {
        prop::array::uniform20(any::<u8>()).prop_map(Address::from)
    }
    fn arb_b256() -> impl Strategy<Value = B256> {
        prop::array::uniform32(any::<u8>()).prop_map(B256::from)
    }
    fn arb_u256() -> impl Strategy<Value = U256> {
        any::<[u8; 32]>().prop_map(U256::from_be_bytes)
    }
    fn small_bytes() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(any::<u8>(), 0..64)
    }
    fn opt_address() -> impl Strategy<Value = Option<Address>> {
        prop_oneof![Just(None), arb_address().prop_map(Some)]
    }

    proptest! {
        #[test]
        fn typed_unsigned_roundtrip(
            chain_id in arb_u256(),
            from in arb_address(),
            nonce in any::<u64>(),
            gas_fee_cap in arb_u256(),
            gas in any::<u64>(),
            to in opt_address(),
            value in arb_u256(),
            data in small_bytes(),
        ) {
            let env = ArbTxEnvelope::Unsigned(ArbUnsignedTx { chain_id, from, nonce, gas_fee_cap, gas, to, value, data });
            let enc = env.encode_typed();
            let (dec, used) = ArbTxEnvelope::decode_typed(&enc).expect("decode");
            assert_eq!(used, enc.len());
            assert_eq!(dec, env);
        }

        #[test]
        fn typed_internal_roundtrip(chain_id in arb_u256(), data in small_bytes()) {
            let env = ArbTxEnvelope::Internal(ArbInternalTx { chain_id, data });
            let enc = env.encode_typed();
            let (dec, used) = ArbTxEnvelope::decode_typed(&enc).expect("decode");
            assert_eq!(used, enc.len());
            assert_eq!(dec, env);
        }

        #[test]
        fn typed_contract_roundtrip(
            chain_id in arb_u256(),
            request_id in arb_b256(),
            from in arb_address(),
            gas_fee_cap in arb_u256(),
            gas in any::<u64>(),
            to in opt_address(),
            value in arb_u256(),
            data in small_bytes(),
        ) {
            let env = ArbTxEnvelope::Contract(ArbContractTx { chain_id, request_id, from, gas_fee_cap, gas, to, value, data });
            let enc = env.encode_typed();
            let (dec, used) = ArbTxEnvelope::decode_typed(&enc).expect("decode");
            assert_eq!(used, enc.len());
            assert_eq!(dec, env);
        }

        #[test]
        fn typed_retry_roundtrip(
            chain_id in arb_u256(),
            nonce in any::<u64>(),
            from in arb_address(),
            gas_fee_cap in arb_u256(),
            gas in any::<u64>(),
            to in opt_address(),
            value in arb_u256(),
            data in small_bytes(),
            ticket_id in arb_b256(),
            refund_to in arb_address(),
            max_refund in arb_u256(),
            submission_fee_refund in arb_u256(),
        ) {
            let env = ArbTxEnvelope::Retry(ArbRetryTx {
                chain_id, nonce, from, gas_fee_cap, gas, to, value, data, ticket_id, refund_to, max_refund, submission_fee_refund
            });
            let enc = env.encode_typed();
            let (dec, used) = ArbTxEnvelope::decode_typed(&enc).expect("decode");
            assert_eq!(used, enc.len());
            assert_eq!(dec, env);
        }

        #[test]
        fn typed_submit_retryable_roundtrip(
            chain_id in arb_u256(),
            request_id in arb_b256(),
            from in arb_address(),
            l1_base_fee in arb_u256(),
            deposit_value in arb_u256(),
            gas_fee_cap in arb_u256(),
            gas in any::<u64>(),
            retry_to in opt_address(),
            retry_value in arb_u256(),
            beneficiary in arb_address(),
            max_submission_fee in arb_u256(),
            fee_refund_addr in arb_address(),
            retry_data in small_bytes(),
        ) {
            let env = ArbTxEnvelope::SubmitRetryable(ArbSubmitRetryableTx {
                chain_id, request_id, from, l1_base_fee, deposit_value, gas_fee_cap, gas, retry_to, retry_value, beneficiary, max_submission_fee, fee_refund_addr, retry_data
            });
            let enc = env.encode_typed();
            let (dec, used) = ArbTxEnvelope::decode_typed(&enc).expect("decode");
            assert_eq!(used, enc.len());
            assert_eq!(dec, env);
        }
    }
}

#[cfg(test)]
mod golden {
    use super::*;
    #[test]
    #[ignore]
    fn golden_unsigned_matches_nitro_rlp() {
        let golden: alloc::vec::Vec<u8> = alloc::vec::Vec::new();
        let (env, used) = ArbTxEnvelope::decode_typed(&golden).expect("decode");
        assert_eq!(used, golden.len());
        let out = env.encode_typed();
        assert_eq!(out, golden);
    }
#[cfg(test)]
mod golden_more {
    use super::*;
    #[test]
    #[ignore]
    fn golden_contract_matches_nitro_rlp() {
        let golden: Vec<u8> = Vec::new();
        let (env, used) = ArbTxEnvelope::decode_typed(&golden).expect("decode");
        assert_eq!(used, golden.len());
        let out = env.encode_typed();
        assert_eq!(out, golden);
    }

    #[test]
    #[ignore]
    fn golden_retry_matches_nitro_rlp() {
        let golden: Vec<u8> = Vec::new();
        let (env, used) = ArbTxEnvelope::decode_typed(&golden).expect("decode");
        assert_eq!(used, golden.len());
        let out = env.encode_typed();
        assert_eq!(out, golden);
    }

    #[test]
    #[ignore]
    fn golden_submit_retryable_matches_nitro_rlp() {
        let golden: Vec<u8> = Vec::new();
        let (env, used) = ArbTxEnvelope::decode_typed(&golden).expect("decode");
        assert_eq!(used, golden.len());
        let out = env.encode_typed();
        assert_eq!(out, golden);
    }

    #[test]
    #[ignore]
    fn golden_internal_matches_nitro_rlp() {
        let golden: Vec<u8> = Vec::new();
        let (env, used) = ArbTxEnvelope::decode_typed(&golden).expect("decode");
        assert_eq!(used, golden.len());
        let out = env.encode_typed();
        assert_eq!(out, golden);
    }

    #[test]
    #[ignore]
    fn golden_deposit_matches_nitro_rlp() {
        let golden: Vec<u8> = Vec::new();
        let (env, used) = ArbTxEnvelope::decode_typed(&golden).expect("decode");
        assert_eq!(used, golden.len());
        let out = env.encode_typed();
        assert_eq!(out, golden);
    }
}

}
