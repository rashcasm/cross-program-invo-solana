# CPI Solana Example

## Structure

- `programs/`
  - `program-a/` and `program-b/`: Rust source code for Solana programs.
- `migrations/`: Deployment scripts (e.g., `deploy.ts`).
- `tests/`: TypeScript tests for the programs.
- `target/`: Build artifacts, deployed binaries, and IDLs.
- `.anchor/`: Anchor logs and test ledger files.

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://book.anchor-lang.com/chapter_1/installation.html)
- [Node.js](https://nodejs.org/)

### Build Programs
```sh
anchor build
```

### Run Tests
```sh
anchor test
```

## Key Files
- `programs/program-a/src/lib.rs`: Main logic for Program A
- `programs/program-b/src/lib.rs`: Main logic for Program B
- `migrations/deploy.ts`: Deployment script
- `tests/program-a.ts`: Example test

