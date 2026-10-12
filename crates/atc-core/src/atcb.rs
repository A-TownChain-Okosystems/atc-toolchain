//! Shared ATCB-v1 container framing contract for toolchain consumers.
//! Normative format ownership remains in atclang (ATC-BC-001).

pub const ATCB_V1_HEADER_LEN: usize = 10;
pub const ATCB_V1_FORMAT_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtcbV1Error {
    HeaderTooShort { actual: usize },
    InvalidMagic,
    UnsupportedVersion { actual: u16 },
    InstructionCountExceedsPayload { declared: u32, available: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtcbV1<'a> {
    pub instruction_count: u32,
    pub instructions: &'a [u8],
}

/// Decode the fixed ATCB-v1 container header without allocating from the
/// untrusted instruction count. Opcode framing is checked by each consumer.
pub fn decode_atcb_v1(bytes: &[u8]) -> Result<AtcbV1<'_>, AtcbV1Error> {
    if bytes.len() < ATCB_V1_HEADER_LEN {
        return Err(AtcbV1Error::HeaderTooShort { actual: bytes.len() });
    }
    if bytes[..4] != *b"ATCB" {
        return Err(AtcbV1Error::InvalidMagic);
    }

    let version = u16::from_be_bytes([bytes[4], bytes[5]]);
    if version != ATCB_V1_FORMAT_VERSION {
        return Err(AtcbV1Error::UnsupportedVersion { actual: version });
    }

    let instruction_count =
        u32::from_be_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]);
    let instructions = &bytes[ATCB_V1_HEADER_LEN..];
    if instruction_count as u64 > instructions.len() as u64 {
        return Err(AtcbV1Error::InstructionCountExceedsPayload {
            declared: instruction_count,
            available: instructions.len(),
        });
    }

    Ok(AtcbV1 { instruction_count, instructions })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn container(version: u16, count: u32, payload: &[u8]) -> Vec<u8> {
        let mut bytes = b"ATCB".to_vec();
        bytes.extend_from_slice(&version.to_be_bytes());
        bytes.extend_from_slice(&count.to_be_bytes());
        bytes.extend_from_slice(payload);
        bytes
    }

    #[test]
    fn accepts_header_and_exposes_payload_without_copying() {
        let bytes = container(1, 1, &[0x21]);
        let decoded = decode_atcb_v1(&bytes).unwrap();
        assert_eq!(decoded.instruction_count, 1);
        assert_eq!(decoded.instructions, &[0x21]);
    }

    #[test]
    fn rejects_short_header() {
        assert_eq!(decode_atcb_v1(b"ATCB"), Err(AtcbV1Error::HeaderTooShort { actual: 4 }));
    }

    #[test]
    fn rejects_wrong_magic() {
        assert_eq!(decode_atcb_v1(&container(1, 0, &[])), Ok(AtcbV1 { instruction_count: 0, instructions: &[] }));
        let mut bytes = container(1, 0, &[]);
        bytes[..4].copy_from_slice(b"NOPE");
        assert_eq!(decode_atcb_v1(&bytes), Err(AtcbV1Error::InvalidMagic));
    }

    #[test]
    fn rejects_unsupported_version() {
        assert_eq!(decode_atcb_v1(&container(2, 0, &[])), Err(AtcbV1Error::UnsupportedVersion { actual: 2 }));
    }

    #[test]
    fn rejects_count_larger_than_payload_before_allocating() {
        assert_eq!(
            decode_atcb_v1(&container(1, u32::MAX, &[])),
            Err(AtcbV1Error::InstructionCountExceedsPayload { declared: u32::MAX, available: 0 })
        );
    }
}
