use atc_core::{decode_atcb_v1, AtcbV1Error, Limits};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError {
    HeaderTooShort { actual: usize },
    InvalidMagic,
    UnsupportedVersion { actual: u16 },
    InstructionCountExceedsPayload { declared: u32, available: usize },
    ProgramOpsExceeded { actual: u32, limit: u32 },
    UnknownOpcode { pc: u32, opcode: u8 },
    TruncatedInstruction { pc: u32, opcode: u8 },
    InstructionCountMismatch { declared: u32, parsed: u32 },
    StackLimitInvalid,
}

fn instruction_width(opcode: u8) -> Option<usize> {
    match opcode {
        0x01 => Some(9),       // ConstI64
        0x02 | 0x03 => Some(3), // LoadLocal / StoreLocal
        0x10..=0x1A => Some(1), // arithmetic/comparison
        0x20 => Some(5),       // Call
        0x21 => Some(1),       // Return
        0x30 | 0x31 => Some(3), // Jump / JumpIfFalse
        0x40 => Some(1),       // Pop
        _ => None,
    }
}

fn map_header_error(error: AtcbV1Error) -> VerificationError {
    match error {
        AtcbV1Error::HeaderTooShort { actual } => VerificationError::HeaderTooShort { actual },
        AtcbV1Error::InvalidMagic => VerificationError::InvalidMagic,
        AtcbV1Error::UnsupportedVersion { actual } => VerificationError::UnsupportedVersion { actual },
        AtcbV1Error::InstructionCountExceedsPayload { declared, available } => {
            VerificationError::InstructionCountExceedsPayload { declared, available }
        }
    }
}

/// Verify ATCB-v1 container framing. This is structural container validation,
/// not full semantic/stack/control-flow verification owned by canonical specs.
pub fn verify(bytecode: &[u8], limits: Limits) -> Result<(), VerificationError> {
    let container = decode_atcb_v1(bytecode).map_err(map_header_error)?;
    let declared = container.instruction_count;
    if declared > limits.max_program_ops {
        return Err(VerificationError::ProgramOpsExceeded {
            actual: declared,
            limit: limits.max_program_ops,
        });
    }
    if limits.max_stack_items == 0 {
        return Err(VerificationError::StackLimitInvalid);
    }

    let instructions = container.instructions;
    let mut pc = 0u32;
    let mut offset = 0usize;
    while pc < declared {
        let opcode = *instructions
            .get(offset)
            .ok_or(VerificationError::TruncatedInstruction { pc, opcode: 0 })?;
        let width = instruction_width(opcode)
            .ok_or(VerificationError::UnknownOpcode { pc, opcode })?;
        if offset + width > instructions.len() {
            return Err(VerificationError::TruncatedInstruction { pc, opcode });
        }
        offset += width;
        pc += 1;
    }

    if offset != instructions.len() {
        return Err(VerificationError::InstructionCountMismatch {
            declared,
            parsed: pc,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn container(version: u16, count: u32, payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::from(*b"ATCB");
        out.extend_from_slice(&version.to_be_bytes());
        out.extend_from_slice(&count.to_be_bytes());
        out.extend_from_slice(payload);
        out
    }

    #[test]
    fn accepts_canonical_simple_program() {
        let mut payload = vec![0x01];
        payload.extend_from_slice(&42i64.to_be_bytes());
        payload.push(0x21);
        assert!(verify(&container(1, 2, &payload), Limits::default()).is_ok());
    }

    #[test]
    fn rejects_invalid_magic() {
        let mut bytes = container(1, 0, &[]);
        bytes[..4].copy_from_slice(b"NOPE");
        assert_eq!(verify(&bytes, Limits::default()), Err(VerificationError::InvalidMagic));
    }

    #[test]
    fn rejects_unsupported_version() {
        assert_eq!(verify(&container(2, 0, &[]), Limits::default()), Err(VerificationError::UnsupportedVersion { actual: 2 }));
    }

    #[test]
    fn rejects_count_larger_than_payload() {
        assert_eq!(
            verify(&container(1, u32::MAX, &[]), Limits::default()),
            Err(VerificationError::InstructionCountExceedsPayload { declared: u32::MAX, available: 0 })
        );
    }

    #[test]
    fn rejects_unknown_opcode() {
        assert_eq!(verify(&container(1, 1, &[0xff]), Limits::default()), Err(VerificationError::UnknownOpcode { pc: 0, opcode: 0xff }));
    }

    #[test]
    fn rejects_truncated_operand() {
        assert!(matches!(verify(&container(1, 1, &[0x01, 0, 0, 0, 0]), Limits::default()), Err(VerificationError::TruncatedInstruction { pc: 0, opcode: 0x01 })));
    }

    #[test]
    fn rejects_trailing_bytes() {
        assert!(matches!(verify(&container(1, 1, &[0x21, 0x00]), Limits::default()), Err(VerificationError::InstructionCountMismatch { .. })));
    }

    #[test]
    fn enforces_program_op_limit() {
        let limits = Limits { max_program_ops: 1, ..Limits::default() };
        assert!(matches!(verify(&container(1, 2, &[0x21, 0x21]), limits), Err(VerificationError::ProgramOpsExceeded { actual: 2, limit: 1 })));
    }
}
