use atc_core::{Limits, BYTECODE_MAGIC};

const FORMAT_VERSION: u16 = 1;
const HEADER_LEN: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError {
    HeaderTooShort { actual: usize },
    InvalidMagic,
    UnsupportedVersion { actual: u16 },
    ProgramOpsExceeded { actual: u32, limit: u32 },
    UnknownOpcode { pc: u32, opcode: u8 },
    TruncatedInstruction { pc: u32, opcode: u8 },
    InstructionCountMismatch { declared: u32, parsed: u32 },
    StackLimitInvalid,
}

fn read_u16_be(bytes: &[u8], offset: usize) -> Option<u16> {
    bytes
        .get(offset..offset + 2)
        .map(|v| u16::from_be_bytes([v[0], v[1]]))
}

fn instruction_width(opcode: u8) -> Option<usize> {
    match opcode {
        0x01 => Some(9),        // ConstI64
        0x02 | 0x03 => Some(3), // LoadLocal / StoreLocal
        0x10..=0x1A => Some(1), // arithmetic/comparison
        0x20 => Some(5),        // Call
        0x21 => Some(1),        // canonical Return
        0x30 | 0x31 => Some(3), // Jump / JumpIfFalse
        0x40 => Some(1),        // Pop
        _ => None,
    }
}

/// Verify the canonical structural ATCB-1 framing used by the current
/// ATCLang integration source. Semantic VM verification remains owned by
/// the canonical VM/compiler implementations.
pub fn verify(bytecode: &[u8], limits: Limits) -> Result<(), VerificationError> {
    if bytecode.len() < HEADER_LEN {
        return Err(VerificationError::HeaderTooShort {
            actual: bytecode.len(),
        });
    }
    if bytecode[..4] != BYTECODE_MAGIC {
        return Err(VerificationError::InvalidMagic);
    }

    let version = read_u16_be(bytecode, 4).expect("header length checked");
    if version != FORMAT_VERSION {
        return Err(VerificationError::UnsupportedVersion { actual: version });
    }

    let declared = u32::from_be_bytes(bytecode[6..10].try_into().expect("header length checked"));
    if declared > limits.max_program_ops {
        return Err(VerificationError::ProgramOpsExceeded {
            actual: declared,
            limit: limits.max_program_ops,
        });
    }
    if limits.max_stack_items == 0 {
        return Err(VerificationError::StackLimitInvalid);
    }

    let mut pc = 0u32;
    let mut offset = HEADER_LEN;
    while pc < declared {
        let opcode = *bytecode
            .get(offset)
            .ok_or(VerificationError::TruncatedInstruction { pc, opcode: 0 })?;
        let width =
            instruction_width(opcode).ok_or(VerificationError::UnknownOpcode { pc, opcode })?;
        if offset + width > bytecode.len() {
            return Err(VerificationError::TruncatedInstruction { pc, opcode });
        }
        offset += width;
        pc += 1;
    }

    if offset != bytecode.len() {
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

    fn header(version: u16, count: u32) -> Vec<u8> {
        let mut out = Vec::from(*b"ATCB");
        out.extend_from_slice(&version.to_be_bytes());
        out.extend_from_slice(&count.to_be_bytes());
        out
    }

    #[test]
    fn accepts_canonical_simple_program() {
        let mut bc = header(1, 2);
        bc.push(0x01);
        bc.extend_from_slice(&42i64.to_be_bytes());
        bc.push(0x21);
        assert!(verify(&bc, Limits::default()).is_ok());
    }

    #[test]
    fn rejects_invalid_magic() {
        let mut bc = header(1, 0);
        bc[..4].copy_from_slice(b"NOPE");
        assert_eq!(
            verify(&bc, Limits::default()),
            Err(VerificationError::InvalidMagic)
        );
    }

    #[test]
    fn rejects_unsupported_version() {
        let bc = header(2, 0);
        assert_eq!(
            verify(&bc, Limits::default()),
            Err(VerificationError::UnsupportedVersion { actual: 2 })
        );
    }

    #[test]
    fn rejects_unknown_opcode() {
        let mut bc = header(1, 1);
        bc.push(0xff);
        assert_eq!(
            verify(&bc, Limits::default()),
            Err(VerificationError::UnknownOpcode {
                pc: 0,
                opcode: 0xff
            })
        );
    }

    #[test]
    fn rejects_truncated_operand() {
        let mut bc = header(1, 1);
        bc.push(0x01);
        bc.extend_from_slice(&[0; 4]);
        assert!(matches!(
            verify(&bc, Limits::default()),
            Err(VerificationError::TruncatedInstruction {
                pc: 0,
                opcode: 0x01
            })
        ));
    }

    #[test]
    fn rejects_trailing_bytes() {
        let mut bc = header(1, 1);
        bc.push(0x21);
        bc.push(0x00);
        assert!(matches!(
            verify(&bc, Limits::default()),
            Err(VerificationError::InstructionCountMismatch { .. })
        ));
    }

    #[test]
    fn enforces_program_op_limit() {
        let bc = header(1, 2);
        let limits = Limits {
            max_program_ops: 1,
            ..Limits::default()
        };
        assert!(matches!(
            verify(&bc, limits),
            Err(VerificationError::ProgramOpsExceeded {
                actual: 2,
                limit: 1
            })
        ));
    }
}
