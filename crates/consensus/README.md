# arb-alloy-consensus

Arbitrum-specific consensus primitives built on top of alloy.

This crate provides:
- Arbitrum EIP-2718 transaction envelope type identifiers
- Receipt variants and RLP encoding/decoding rules matching Nitro
- Types and helpers used by arb-reth for execution and storage

Highlights:
- no_std friendly with `alloc` feature (default)
- Parity with Nitro/go-ethereum encodings for deterministic execution

Install:
Add the following to your Cargo.toml:
arb-alloy-consensus = "0.1.1"

Feature flags:
- alloc: enable allocation support (default)

Links:
- Repository: https://github.com/tiljrd/arb-alloy
- Docs: https://docs.rs/arb-alloy-consensus
- License: MIT OR Apache-2.0
