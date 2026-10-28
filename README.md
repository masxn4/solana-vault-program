# Solana Vault Program

An educational Anchor program implementing a per-authority SOL vault backed by a program-derived address (PDA).

## Features

- deterministic vault PDA derived from the authority
- authority-constrained deposits and withdrawals
- rent-aware withdrawal checks
- checked accounting for lifetime deposits and withdrawals
- custom Anchor errors
- structured on-chain events
- TypeScript integration tests

## Architecture

Each wallet gets a vault at:

```text
PDA = ["vault", authority_pubkey]
```

The vault stores its authority, PDA bump, total deposited lamports, and total withdrawn lamports. SOL sent to the account remains controlled by the program; withdrawals require the matching authority signer.

## Instructions

### initialize

Creates the caller's vault PDA and stores the authority and bump.

### deposit(amount)

Transfers lamports from the authority into the vault through the System Program and increments the lifetime deposit counter.

### withdraw(amount)

Checks authority ownership, preserves the vault's rent-exempt balance, transfers lamports back to the authority, and increments the lifetime withdrawal counter.

## Development

Prerequisites: Rust, Solana CLI, Anchor 0.32.x, Node.js.

```bash
npm install
anchor build
anchor test
```

## Security notes

This repository is intentionally small enough to audit. It demonstrates signer constraints, PDA seeds, checked arithmetic, rent preservation, and explicit error paths. It is educational code and has not been independently audited for production custody.

## Roadmap

- SPL-token vault support
- delegated withdrawal limits
- pause/emergency controls
- property-oriented tests
- client helper library
- local validator CI

## License

MIT
