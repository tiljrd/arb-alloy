#![allow(unused)]
#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;
use alloy_primitives::{Address, B256, U256};
use alloy_rlp::{Decodable, Encodable, Header};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbReceiptEnvelope {
    pub status: bool,
    pub cumulative_gas_used: u128,
    pub logs_bloom: [u8; 256],
    pub logs: Vec<ArbLog>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbLog {
    pub address: [u8; 20],
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}

impl Encodable for ArbLog {
    fn length(&self) -> usize {
        let mut tmp = Vec::new();

        let addr = Address::from_slice(&self.address);
        addr.encode(&mut tmp);

        let mut topics_bytes = Vec::new();
        for t in &self.topics {
            let b = B256::from_slice(t);
            b.encode(&mut topics_bytes);
        }
        Header {
            list: true,
            payload_length: topics_bytes.len(),
        }
        .encode(&mut tmp);
        tmp.extend_from_slice(&topics_bytes);

        self.data[..].encode(&mut tmp);

        let header_len = alloy_rlp::length_of_length(tmp.len()) + 1;
        header_len + tmp.len()
    }

    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        let mut tmp = Vec::new();

        let addr = Address::from_slice(&self.address);
        addr.encode(&mut tmp);

        let mut topics_bytes = Vec::new();
        for t in &self.topics {
            let b = B256::from_slice(t);
            b.encode(&mut topics_bytes);
        }
        Header {
            list: true,
            payload_length: topics_bytes.len(),
        }
        .encode(&mut tmp);
        tmp.extend_from_slice(&topics_bytes);

        self.data[..].encode(&mut tmp);

        Header {
            list: true,
            payload_length: tmp.len(),
        }
        .encode(out);
        out.put_slice(&tmp);
    }
}

impl Decodable for ArbLog {
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let header = Header::decode(buf)?;
        let (payload, rest) = buf.split_at(header.payload_length);
        let mut p = payload;

        let addr: Address = Decodable::decode(&mut p)?;

        let topics_header = Header::decode(&mut p)?;
        if !topics_header.list {
            return Err(alloy_rlp::Error::Custom("topics not a list"));
        }
        let (topics_payload, topics_rest) = p.split_at(topics_header.payload_length);
        let mut tp = topics_payload;

        let mut topics: Vec<[u8; 32]> = Vec::new();
        while !tp.is_empty() {
            let b: B256 = Decodable::decode(&mut tp)?;
            let mut t = [0u8; 32];
            t.copy_from_slice(b.as_slice());
            topics.push(t);
        }
        p = topics_rest;

        let data_hdr = Header::decode(&mut p)?;
        if data_hdr.list {
            return Err(alloy_rlp::Error::UnexpectedString);
        }
        if p.len() < data_hdr.payload_length {
            return Err(alloy_rlp::Error::InputTooShort);
        }
        let (data_payload, rest_after_data) = p.split_at(data_hdr.payload_length);
        let data: Vec<u8> = data_payload.to_vec();
        p = rest_after_data;

        if !p.is_empty() {
            return Err(alloy_rlp::Error::Custom("log payload not fully consumed"));
        }

        let mut address = [0u8; 20];
        address.copy_from_slice(addr.as_slice());

        *buf = rest;
        Ok(ArbLog {
            address,
            topics,
            data,
        })
    }
}

impl Encodable for ArbReceiptEnvelope {
    fn length(&self) -> usize {
        let mut tmp = Vec::new();

        U256::from(if self.status { 1u8 } else { 0u8 }).encode(&mut tmp);
        U256::from(self.cumulative_gas_used).encode(&mut tmp);
        self.logs_bloom[..].encode(&mut tmp);

        let mut logs_bytes = Vec::new();
        for log in &self.logs {
            log.encode(&mut logs_bytes);
        }
        Header {
            list: true,
            payload_length: logs_bytes.len(),
        }
        .encode(&mut tmp);
        tmp.extend_from_slice(&logs_bytes);

        let header_len = alloy_rlp::length_of_length(tmp.len()) + 1;
        header_len + tmp.len()
    }

    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        let mut tmp = Vec::new();

        U256::from(if self.status { 1u8 } else { 0u8 }).encode(&mut tmp);
        U256::from(self.cumulative_gas_used).encode(&mut tmp);
        self.logs_bloom[..].encode(&mut tmp);

        let mut logs_bytes = Vec::new();
        for log in &self.logs {
            log.encode(&mut logs_bytes);
        }
        Header {
            list: true,
            payload_length: logs_bytes.len(),
        }
        .encode(&mut tmp);
        tmp.extend_from_slice(&logs_bytes);

        Header {
            list: true,
            payload_length: tmp.len(),
        }
        .encode(out);
        out.put_slice(&tmp);
    }
}

impl Decodable for ArbReceiptEnvelope {
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let header = Header::decode(buf)?;
        let (payload, rest) = buf.split_at(header.payload_length);
        let mut p = payload;

        let status_u: U256 = Decodable::decode(&mut p)?;
        let status = !status_u.is_zero();

        let cg_u256: U256 = Decodable::decode(&mut p)?;
        let cumulative_gas_used = cg_u256.to::<u128>();

        let bloom_header = Header::decode(&mut p)?;
        if bloom_header.list {
            return Err(alloy_rlp::Error::Custom("logs_bloom must be a string"));
        }
        if bloom_header.payload_length != 256 {
            return Err(alloy_rlp::Error::Custom("logs_bloom must be 256 bytes"));
        }
        if p.len() < 256 {
            return Err(alloy_rlp::Error::InputTooShort);
        }
        let (bloom_payload, rest_after_bloom) = p.split_at(256);
        let mut logs_bloom = [0u8; 256];
        logs_bloom.copy_from_slice(bloom_payload);
        p = rest_after_bloom;

        let logs_header = Header::decode(&mut p)?;
        if !logs_header.list {
            return Err(alloy_rlp::Error::Custom("logs not a list"));
        }
        let (logs_payload, logs_rest) = p.split_at(logs_header.payload_length);
        let mut lp = logs_payload;

        let mut logs: Vec<ArbLog> = Vec::new();
        while !lp.is_empty() {
            let log: ArbLog = Decodable::decode(&mut lp)?;
            logs.push(log);
        }
        if !logs_rest.is_empty() {
            return Err(alloy_rlp::Error::Custom("logs overconsumed"));
        }
        p = logs_rest;

        if !p.is_empty() {
            return Err(alloy_rlp::Error::Custom(
                "receipt payload not fully consumed",
            ));
        }

        *buf = rest;
        Ok(ArbReceiptEnvelope {
            status,
            cumulative_gas_used,
            logs_bloom,
            logs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn sample_log() -> ArbLog {
        ArbLog {
            address: {
                let a = Address::from_slice(&[0u8; 20]);
                let mut out = [0u8; 20];
                out.copy_from_slice(a.as_slice());
                out
            },
            topics: vec![[0u8; 32], {
                let mut t = [0u8; 32];
                t[0] = 1;
                t[31] = 2;
                t
            }],
            data: vec![0xde, 0xad, 0xbe, 0xef],
        }
    }

    #[test]
    fn receipt_roundtrip_no_logs() {
        let r = ArbReceiptEnvelope {
            status: true,
            cumulative_gas_used: 12345,
            logs_bloom: [0u8; 256],
            logs: Vec::new(),
        };
        let mut out = Vec::new();
        r.encode(&mut out);
        let mut s = out.as_slice();
        let dec = <ArbReceiptEnvelope as Decodable>::decode(&mut s).unwrap();
        assert!(s.is_empty());
        assert_eq!(dec, r);
    }

    #[test]
    fn receipt_roundtrip_with_logs() {
        let r = ArbReceiptEnvelope {
            status: false,
            cumulative_gas_used: 1_000_000,
            logs_bloom: [0x11u8; 256],
            logs: vec![sample_log()],
        };
        let mut out = Vec::new();
        r.encode(&mut out);
        let mut s = out.as_slice();
        let dec = <ArbReceiptEnvelope as Decodable>::decode(&mut s).unwrap();
        assert!(s.is_empty());
        assert_eq!(dec, r);
    }

    #[test]
    fn receipt_length_header_matches_payload() {
        let r = ArbReceiptEnvelope {
            status: true,
            cumulative_gas_used: 42,
            logs_bloom: [0u8; 256],
            logs: vec![sample_log(), sample_log()],
        };
        let mut out = Vec::new();
        r.encode(&mut out);

        let mut s = out.as_slice();
        let h = Header::decode(&mut s).unwrap();
        assert_eq!(h.payload_length, s.len());
    }
}
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    fn arb_bloom() -> impl Strategy<Value = [u8; 256]> {
        prop::collection::vec(any::<u8>(), 256).prop_map(|v| {
            let mut arr = [0u8; 256];
            arr.copy_from_slice(&v);
            arr
        })
    }
    fn arb_addr20() -> impl Strategy<Value = [u8; 20]> {
        prop::array::uniform20(any::<u8>())
    }
    fn arb_topic() -> impl Strategy<Value = [u8; 32]> {
        prop::array::uniform32(any::<u8>())
    }
    fn arb_log() -> impl Strategy<Value = ArbLog> {
        (
            arb_addr20(),
            prop::collection::vec(arb_topic(), 0..4),
            prop::collection::vec(any::<u8>(), 0..64),
        )
            .prop_map(|(address, topics, data)| ArbLog {
                address,
                topics,
                data,
            })
    }
    fn arb_logs() -> impl Strategy<Value = alloc::vec::Vec<ArbLog>> {
        prop::collection::vec(arb_log(), 0..3)
    }

    proptest! {
        #[test]
        fn receipt_roundtrip_prop(
            status in any::<bool>(),
            cumulative_gas_used in any::<u128>(),
            logs_bloom in arb_bloom(),
            logs in arb_logs(),
        ) {
            let r = ArbReceiptEnvelope { status, cumulative_gas_used, logs_bloom, logs };
            let mut out = alloc::vec::Vec::new();
            r.encode(&mut out);
            let mut s = out.as_slice();
            let dec = <ArbReceiptEnvelope as Decodable>::decode(&mut s).expect("decode");
            assert!(s.is_empty());
            assert_eq!(dec, r);
        }
    }
}

#[cfg(test)]
mod golden {
    use super::*;
    #[test]
    fn golden_receipt_roundtrip_matches_nitro_rlp() {
        let hex = "f9014701823039b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f83ef83c9400000000000000000000000000000000000000aae1a0010000000000000000000000000000000000000000000000000000000000000284deadbeef";
        let golden = hex::decode(hex).expect("hex decode");
        let mut s = golden.as_slice();
        let dec = <ArbReceiptEnvelope as Decodable>::decode(&mut s).expect("decode");
        assert!(s.is_empty());
        let mut out = alloc::vec::Vec::new();
        dec.encode(&mut out);
        assert_eq!(out, golden);
    }
}
