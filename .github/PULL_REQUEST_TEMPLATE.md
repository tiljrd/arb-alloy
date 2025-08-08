Title: [Consolidated] Arbitrum (Nitro) primitives/helpers/tests

Summary
- Consolidated PR for arb-alloy Arbitrum primitives, pricing, retryables, and predeploys.
- Targets exact Nitro/geth parity for:
  - EIP-2718 Arbitrum tx variants (type bytes, field order, RLP)
  - Consensus receipt RLP that excludes L1 gas fields
  - L1 pricing helpers (PosterDataCost, data-gas translation, padding, 100 bips; EIP-2028 nonzero-byte gas = 16)
  - Retryables helpers (submission fee, lifetime/reap constants, derivations)
  - Predeploy selectors/events (ArbOwner, AddressTable, GasInfo, NodeInterface)
  - Tests: roundtrips, payload-length, vectors, property tests

References
- Nitro receipts (exclude L1 gas in consensus): nitro/go-ethereum/core/types/receipt.go
- L1 pricing: nitro/arbos/l1pricing/*
- Retryables: nitro/arbos/retryables/retryable.go
- Predeploys: nitro/precompiles/* and contracts

Requester and Session
- Requested by: Til Jordan (@tiljrd)
- Link to Devin run: https://app.devin.ai/sessions/9ec52061d809477eac1d0db0e3375897

Review Checklist
- [ ] EIP-2718 variant bytes/field order match Nitro
- [ ] Consensus receipt RLP excludes any L1 gas fields
- [ ] PosterDataCost/data-gas helpers match Nitro math (+padding/+100 bips)
- [ ] Retryables helpers: baseFee*(1400+6*len) and constants are correct
- [ ] Predeploy selectors/events match bytecode ABI
- [ ] Tests: roundtrips and property tests pass
