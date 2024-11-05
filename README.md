# zk-voting-proofs

Privacy enhancing and scalable solution for zk-voting-proof generation
for DAO and DeFi voting applications.

## Building

To create a production version of your app:

```bash
cargo run

Listening on: 127.0.0.1:9001
```

## Technology

Our solution leverages zero-knowledge proofs (ZKPs) to generate proofs of asset ownership that safeguard user privacy. Using Facebook's open-source Winterfell framework, a Rust-based toolkit for STARK proofs and verification of arbitrary computations, we aim to deliver an effective and scalable privacy solution. By integrating ZKP functionality with Solana programs, this project achieves the following goals;

- **Anonymous Voting**: Safeguard voter privacy by verifying voting eligibility without disclosing address details or balances.
- **Cost-Effective, Off-Chain Voting**: Enable free, off-chain vote casting while ensuring all proofs are secure and verifiable.
- **Transparent Verification of Voting Proofs**: Solana programs verify proofs publicly, supporting individual or batch processing to optimize efficiency.
- **Efficient Verification via Merkle Trees**: Batch proofs can be committed to Merkle trees, reducing on-chain workload while maintaining transparency and security.
- **Customizable Voting Criteria**: Allow DAOs and DeFi projects to define specific eligibility and voting requirements, voting windows etc, to match their governance needs.
