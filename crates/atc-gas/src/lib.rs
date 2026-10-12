//! Gas policy/analyzer. Consensus schedule is injected, never invented here.
//! Input is the complete canonical ATCB-v1 container, not a raw opcode payload.
use atc_core::{decode_atcb_v1, AtcbV1Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GasPolicy {
    pub gas_limit: u64,
    pub const_i64: u64,
    pub unary: u64,
    pub binary: u64,
    pub call: u64,
    pub control_flow: u64,
    pub return_cost: u64,
    pub pop_cost: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GasReport {
    pub operations: u64,
    pub total: u64,
    pub limit: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GasError {
    InvalidContainer(AtcbV1Error),
    UnknownOpcode(u8),
    TruncatedInstruction(u8),
    InstructionCountMismatch { declared: u32, parsed: u32 },
    Overflow,
    LimitExceeded { total: u64, limit: u64 },
}
impl GasPolicy {
    pub fn analyze(self, bytecode: &[u8]) -> Result<GasReport, GasError> {
        let container = decode_atcb_v1(bytecode).map_err(GasError::InvalidContainer)?;
        let instructions = container.instructions;
        let declared = container.instruction_count;
        let mut offset = 0usize;
        let mut ops: u64 = 0;
        let mut total: u64 = 0;

        while ops < u64::from(declared) {
            let op = *instructions.get(offset).ok_or(GasError::TruncatedInstruction(0))?;
            let (width, cost) = match op {
                0x01 => (9, self.const_i64),
                0x02 | 0x03 => (3, self.unary),
                0x10..=0x1a => (1, self.binary),
                0x20 => (5, self.call),
                0x21 => (1, self.return_cost),
                0x30 | 0x31 => (3, self.control_flow),
                0x40 => (1, self.pop_cost),
                _ => return Err(GasError::UnknownOpcode(op)),
            };
            if offset + width > instructions.len() {
                return Err(GasError::TruncatedInstruction(op));
            }
            ops = ops.checked_add(1).ok_or(GasError::Overflow)?;
            total = total.checked_add(cost).ok_or(GasError::Overflow)?;
            if total > self.gas_limit {
                return Err(GasError::LimitExceeded { total, limit: self.gas_limit });
            }
            offset += width;
        }

        if offset != instructions.len() {
            return Err(GasError::InstructionCountMismatch { declared, parsed: declared });
        }
        Ok(GasReport { operations: ops, total, limit: self.gas_limit })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> GasPolicy {
        GasPolicy { gas_limit: 100, const_i64: 2, unary: 1, binary: 1, call: 5, control_flow: 2, return_cost: 1, pop_cost: 1 }
    }

    fn container(count: u32, payload: &[u8]) -> Vec<u8> {
        let mut bytes = b"ATCB".to_vec();
        bytes.extend_from_slice(&1u16.to_be_bytes());
        bytes.extend_from_slice(&count.to_be_bytes());
        bytes.extend_from_slice(payload);
        bytes
    }

    #[test]
    fn analyzes_full_atcb_container_not_header_as_opcode() {
        let mut payload = vec![0x01];
        payload.extend_from_slice(&1i64.to_be_bytes());
        payload.push(0x21);
        assert_eq!(policy().analyze(&container(2, &payload)), Ok(GasReport { operations: 2, total: 3, limit: 100 }));
    }

    #[test]
    fn rejects_invalid_container_header() {
        assert_eq!(policy().analyze(b"ATCB"), Err(GasError::InvalidContainer(AtcbV1Error::HeaderTooShort { actual: 4 })));
    }

    #[test]
    fn rejects_count_larger_than_payload() {
        assert_eq!(policy().analyze(&container(u32::MAX, &[])), Err(GasError::InvalidContainer(AtcbV1Error::InstructionCountExceedsPayload { declared: u32::MAX, available: 0 })));
    }

    #[test]
    fn rejects_unknown_opcode() {
        assert_eq!(policy().analyze(&container(1, &[0xff])), Err(GasError::UnknownOpcode(0xff)));
    }

    #[test]
    fn rejects_truncated_operand() {
        assert_eq!(policy().analyze(&container(1, &[0x01, 0, 0, 0])), Err(GasError::TruncatedInstruction(0x01)));
    }

    #[test]
    fn rejects_trailing_bytes() {
        assert_eq!(policy().analyze(&container(1, &[0x21, 0x21])), Err(GasError::InstructionCountMismatch { declared: 1, parsed: 1 }));
    }
}
