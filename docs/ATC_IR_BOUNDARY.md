# ATC-IR Toolchain Boundary

Status: implementation baseline; canonical ATC-IR remains a draft in atclang.

`atclang` owns the canonical lexer, parser and semantic gates. `atc-toolchain` consumes a typed IR contract and provides deterministic orchestration, verification adapters, artifact handling and evidence.

Rules:
1. No untyped IR enters code generation.
2. Canonical serialization contains no host metadata.
3. Canonical IR changes are Gate-0 dependencies.
4. The toolchain does not reimplement the language frontend or canonical VM.
