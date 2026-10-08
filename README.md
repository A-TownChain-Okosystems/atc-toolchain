# ATC Toolchain

> Deterministic Rust-first toolchain for ATCLang, ATC-VM bytecode verification, ABI/artifacts, cross-target tooling, and exact-SHA evidence.

**Project:** `atc-toolchain`  
**Organization:** A-TownChain-Okosystems  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0`

## Purpose and Scope

`atc-toolchain` provides the deterministic execution, verification, and artifact management tooling for the A-TownChain ecosystem. Its primary purpose is to establish exact-SHA build provenance, bytecode verification adapters, and standardized evidence collection for compiler and VM artifacts.

## Ownership and boundaries

| Repository | Canonical responsibility |
|---|---|
| `atclang` | ATCLang language/compiler source of truth |
| `a-townchain/components/vm` | Canonical ATC-VM implementation |
| `atc-toolchain` | Toolchain orchestration, IR/tooling, bytecode tooling, verification adapters, ABI/artifact/evidence tooling |
| `a-townchain-ecosystem` | Integration, compliance and evidence only |

**Standalone First, Ecosystem Second.** This repository must not become a second implementation of the language or VM.

## Architecture

The toolchain architecture consists of modular Components, deterministic data flow pipelines, and verification boundaries:

```text
.atc
  -> Lexer
  -> Parser
  -> AST
  -> Semantic / Capability Analysis
  -> ATC-IR
  -> Optimization
  -> Bytecode Generation
  -> Bytecode Validation
  -> Gas Analysis
  -> ATC Artifact
```

Build provenance is bound to:

```text
Source SHA
  -> Compiler SHA
  -> Toolchain Version
  -> Compiler Configuration
  -> Input Hash
  -> Artifact Hash
```

## ATCB verification boundary

The current verifier implements the structural ATCB-1 baseline: magic, format version, instruction count, canonical instruction framing, unknown-opcode rejection, truncation rejection, and program-op limits. The canonical ATCLang repository remains the authority for bytecode semantics; this crate is an integration/verification boundary, not a replacement VM.

The canonical source currently exposes the ATCB header as:

```text
magic             = "ATCB"        4 bytes
format_version    = u16 BE        2 bytes
instruction_count = u32 BE        4 bytes
instructions      = canonical opcode stream
```

Any conflict between this integration layer and the canonical ATCLang/ATC-VM specification is a release blocker and must be resolved by Gate 0 before extending the implementation.

## CLI

Implemented baseline commands:

```text
atc version
atc toolchain info
atc doctor
```

Planned command surface:

```text
atc build
atc check
atc test
atc verify
atc lang compile|check|fmt
atc vm assemble|disassemble|verify|analyze|gas
atc abi generate|verify
atc target list|build
atc artifact inspect|hash
atc evidence collect|verify
```

## Usage

Example commands to use the toolchain CLI and verifier:

```bash
atc version
atc doctor
```

## Evidence model

A successful build is not itself verification.

```text
IMPLEMENTED
  -> EXECUTED
  -> TESTED
  -> EVIDENCE COLLECTED
  -> EXACT-SHA VERIFIED
  -> VERIFIED
```

Every verification claim must bind to an immutable commit SHA and retain run/job/step, exit code and log evidence. No green CI claim is converted into `VERIFIED` without that evidence.

## Requirements

- Rust stable toolchain
- Python 3.11+ for repository governance tooling
- Git 2.30+

## Installation

To clone and build `atc-toolchain`:

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-toolchain.git
cd atc-toolchain
cargo build --workspace
```

## Development

Local validation commands:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
```

Governance and Rust validation run independently; one must not weaken or replace the other.

## Testing

Run workspace tests:

```bash
cargo test --workspace
```

Expected result: PASS (all unit and integration tests passing).

## Repository Structure

```text
.
├── crates
│   ├── atc-cli
│   ├── atc-core
│   ├── atc-verifier
│   ├── atc-artifact
│   └── atc-evidence
├── docs
├── tests
└── .atc
```

## Standards & Compliance

| Standard | Title | Status |
|---|---|---|
| `ATC-STD-201` | Repository Structure Standard | Compliant |
| `ATC-STD-202` | Naming & Classification Standard | Compliant |
| `ATC-STD-203` | Security & Branching Standard | Compliant |

## Security

Vulnerabilities must not be disclosed publicly. Follow official ATC security reporting procedures under `ATC-STD-203`.

## Roadmap

Roadmap and release milestones are tracked in `ROADMAP.md` and canonical GitHub Issues.

## Governance

Repository governance follows the A-TownChain ecosystem standards.

## Maintainers

Maintained by ShivaCoreDev and the A-TownChain core engineering team.

## Status

R1 deterministic foundation is implemented. Full compiler integration, complete ATC-IR, semantic/capability verification, gas analysis, ABI management, deterministic packaging, cross compilation, and the complete CLI remain open roadmap work.

No production, mainnet, or full-verifier claim is made by repository status alone.

## License

Licensed under the Apache License, Version 2.0 (`Apache-2.0`).
