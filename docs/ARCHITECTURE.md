# ATC-Toolchain Architecture
## Ownership
- atclang: ATCLang language/compiler source of truth.
- a-townchain/components/vm: canonical ATC-VM implementation.
- atc-toolchain: orchestration, IR/tooling, bytecode tooling, verification adapters, ABI/artifact/evidence tooling.
- a-townchain-ecosystem: integration, compliance, and evidence only.
Rule: **Standalone First, Ecosystem Second.**
## Deterministic pipeline
.atc -> Lexer -> Parser -> AST -> Semantic Analysis -> Capability Analysis -> ATC-IR -> Optimization -> Bytecode Generation -> Bytecode Validation -> Gas Analysis -> ATC Artifact
Optimization MUST preserve semantics. Canonical builds must not depend on ambient time, network state, filesystem ordering, locale, or nondeterministic iteration.
## Provenance
Source SHA -> Compiler SHA -> Toolchain Version -> Compiler Configuration -> Input Hash -> Artifact Hash
Manifest schema: ATC-TOOLCHAIN-MANIFEST-1.
## Verification
Fail-closed verification covers structural validity, opcode validity, bytecode version, stack constraints, gas budget, capability constraints, and deterministic encoding.
## Evidence
IMPLEMENTED -> EXECUTED -> TESTED -> EVIDENCE COLLECTED -> EXACT-SHA VERIFIED -> VERIFIED
A successful process exit is execution evidence only; Exact-SHA evidence must bind the evaluated commit.
