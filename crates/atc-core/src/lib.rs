pub const TOOLCHAIN_SCHEMA: &str = "ATC-TOOLCHAIN-1";
pub const BYTECODE_MAGIC: [u8; 4] = *b"ATCB";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    pub max_program_ops: u32,
    pub max_stack_items: u32,
    pub max_storage_slots: u32,
    pub gas_limit: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_program_ops: 65_536,
            max_stack_items: 4_096,
            max_storage_slots: 65_536,
            gas_limit: u64::MAX,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn policy_is_explicit() {
        assert_eq!(Limits::default().max_program_ops, 65_536);
    }
    #[test]
    fn magic_is_atcb() {
        assert_eq!(&BYTECODE_MAGIC, b"ATCB");
    }
}
