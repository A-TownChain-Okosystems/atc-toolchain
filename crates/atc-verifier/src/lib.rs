use atc_core::{Limits, BYTECODE_MAGIC};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError { InvalidMagic, ProgramOpsExceeded { actual:u32, limit:u32 }, StackLimitInvalid }
pub fn verify(bytecode:&[u8], limits:Limits)->Result<(),VerificationError>{
 if bytecode.len()<12 || bytecode[..4]!=BYTECODE_MAGIC { return Err(VerificationError::InvalidMagic); }
 let ops=u32::from_be_bytes(bytecode[6..10].try_into().unwrap());
 if ops>limits.max_program_ops { return Err(VerificationError::ProgramOpsExceeded{actual:ops,limit:limits.max_program_ops}); }
 if limits.max_stack_items==0 { return Err(VerificationError::StackLimitInvalid); }
 Ok(())
}
#[cfg(test)] mod tests { use super::*; #[test] fn rejects_invalid_magic(){assert_eq!(verify(b"invalid",Limits::default()),Err(VerificationError::InvalidMagic));} }
