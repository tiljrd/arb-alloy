#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use alloy_consensus::{self as consensus, ReceiptEnvelope, TxEnvelope, TxType};
use alloy_network::Network;
use alloy_primitives::{Address, Bytes, ChainId, TxKind, U256};
use alloy_rpc_types_eth as rpc_eth;
use rpc_eth::{AccessList, TransactionInputKind};

///
#[derive(Clone, Copy, Default, Debug)]
pub struct ArbNetwork;

impl Network for ArbNetwork {
    type TxType = TxType;

    type TxEnvelope = TxEnvelope;

    type UnsignedTx = consensus::TypedTransaction;

    type ReceiptEnvelope = ReceiptEnvelope;

    type Header = consensus::Header;

    type TransactionRequest = rpc_eth::request::TransactionRequest;

    type TransactionResponse = rpc_eth::Transaction<Self::TxEnvelope>;

    type ReceiptResponse = rpc_eth::TransactionReceipt;

    type HeaderResponse = rpc_eth::Header<Self::Header>;

    type BlockResponse = rpc_eth::Block<Self::TransactionResponse, Self::HeaderResponse>;
}

impl alloy_network::TransactionBuilder<ArbNetwork> for rpc_eth::request::TransactionRequest {
    fn chain_id(&self) -> Option<ChainId> { self.chain_id }
    fn set_chain_id(&mut self, chain_id: ChainId) { self.chain_id = Some(chain_id); }

    fn nonce(&self) -> Option<u64> { self.nonce }
    fn set_nonce(&mut self, nonce: u64) { self.nonce = Some(nonce); }
    fn take_nonce(&mut self) -> Option<u64> { self.nonce.take() }

    fn input(&self) -> Option<&Bytes> { self.input.input() }
    fn set_input<T: Into<Bytes>>(&mut self, input: T) { self.input.input = Some(input.into()); }

    fn set_input_kind<T: Into<Bytes>>(&mut self, input: T, kind: TransactionInputKind) {
        match kind {
            TransactionInputKind::Input => self.input.input = Some(input.into()),
            TransactionInputKind::Data => self.input.data = Some(input.into()),
            TransactionInputKind::Both => {
                let bytes = input.into();
                self.input.input = Some(bytes.clone());
                self.input.data = Some(bytes);
            }
        }
    }

    fn from(&self) -> Option<Address> { self.from }
    fn set_from(&mut self, from: Address) { self.from = Some(from); }

    fn kind(&self) -> Option<TxKind> { self.to }
    fn clear_kind(&mut self) { self.to = None; }
    fn set_kind(&mut self, kind: TxKind) { self.to = Some(kind); }

    fn value(&self) -> Option<U256> { self.value }
    fn set_value(&mut self, value: U256) { self.value = Some(value) }

    fn gas_price(&self) -> Option<u128> { self.gas_price }
    fn set_gas_price(&mut self, gas_price: u128) { self.gas_price = Some(gas_price); }

    fn max_fee_per_gas(&self) -> Option<u128> { self.max_fee_per_gas }
    fn set_max_fee_per_gas(&mut self, max_fee_per_gas: u128) { self.max_fee_per_gas = Some(max_fee_per_gas); }

    fn max_priority_fee_per_gas(&self) -> Option<u128> { self.max_priority_fee_per_gas }
    fn set_max_priority_fee_per_gas(&mut self, max_priority_fee_per_gas: u128) {
        self.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
    }

    fn gas_limit(&self) -> Option<u64> { self.gas }
    fn set_gas_limit(&mut self, gas_limit: u64) { self.gas = Some(gas_limit); }

    fn access_list(&self) -> Option<&AccessList> { self.access_list.as_ref() }
    fn set_access_list(&mut self, access_list: AccessList) { self.access_list = Some(access_list); }

    fn complete_type(&self, ty: TxType) -> Result<(), Vec<&'static str>> {
        match ty {
            TxType::Legacy => self.complete_legacy(),
            TxType::Eip2930 => self.complete_2930(),
            TxType::Eip1559 => self.complete_1559(),
            TxType::Eip4844 => self.complete_4844(),
            TxType::Eip7702 => self.complete_7702(),
        }
    }

    fn can_submit(&self) -> bool { self.from.is_some() }

    fn can_build(&self) -> bool {
        let common = self.gas.is_some() && self.nonce.is_some();
        let legacy = self.gas_price.is_some();
        let eip2930 = legacy && self.access_list.is_some();
        let eip1559 = self.max_fee_per_gas.is_some() && self.max_priority_fee_per_gas.is_some();
        let eip4844 = eip1559 && self.sidecar.is_some() && self.to.is_some();
        let eip7702 = eip1559 && self.authorization_list.is_some();
        common && (legacy || eip2930 || eip1559 || eip4844 || eip7702)
    }

    fn output_tx_type(&self) -> TxType { self.preferred_type() }

    fn output_tx_type_checked(&self) -> Option<TxType> { self.buildable_type() }

    fn prep_for_submission(&mut self) {
        self.transaction_type = Some(self.preferred_type() as u8);
        self.trim_conflicting_keys();
        self.populate_blob_hashes();
    }

    fn build_unsigned(self) -> alloy_network::BuildResult<consensus::TypedTransaction, ArbNetwork> {
        if let Err((tx_type, missing)) = self.missing_keys() {
            return Err(
                alloy_network::TransactionBuilderError::InvalidTransactionRequest(
                    TxType::try_from(tx_type as u8).unwrap(),
                    missing,
                ).into_unbuilt(self)
            );
        }
        Ok(self.build_typed_tx().expect("checked by missing_keys"))
    }

    async fn build<W: alloy_network::NetworkWallet<ArbNetwork>>(
        self,
        wallet: &W,
    ) -> Result<<ArbNetwork as alloy_network::Network>::TxEnvelope, alloy_network::TransactionBuilderError<ArbNetwork>> {
        Ok(wallet.sign_request(self).await?)
    }
}
