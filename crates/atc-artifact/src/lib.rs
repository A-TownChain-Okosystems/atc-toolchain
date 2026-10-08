#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    pub schema: String,
    pub artifact: String,
    pub source_sha256: String,
    pub compiler_sha256: String,
    pub toolchain_version: String,
    pub toolchain_sha256: String,
    pub target: String,
    pub bytecode_format: String,
    pub bytecode_version: u16,
    pub deterministic: bool,
    pub gas_verified: bool,
    pub bytecode_verified: bool,
    pub capability_verified: bool,
    pub artifact_sha256: String,
}
impl Manifest {
    pub fn new(artifact: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            schema: "ATC-TOOLCHAIN-MANIFEST-1".into(),
            artifact: artifact.into(),
            source_sha256: String::new(),
            compiler_sha256: String::new(),
            toolchain_version: env!("CARGO_PKG_VERSION").into(),
            toolchain_sha256: String::new(),
            target: target.into(),
            bytecode_format: "ATCB".into(),
            bytecode_version: 1,
            deterministic: false,
            gas_verified: false,
            bytecode_verified: false,
            capability_verified: false,
            artifact_sha256: String::new(),
        }
    }
}
