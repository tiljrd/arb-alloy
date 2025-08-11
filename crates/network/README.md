# arb-alloy-network

Arbitrum-specific marker Network type for reth integration.

This crate intentionally contains only the `ArbNetwork` marker to avoid bringing in conflicting `alloy-*` versions in downstream trees. Reth implements `RpcTypes` for this marker to select Arbitrum RPC types at compile time.
