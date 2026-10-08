//! Gas policy/analyzer. Consensus schedule is injected, never invented here.
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
    UnknownOpcode(u8),
    TruncatedInstruction(u8),
    Overflow,
    LimitExceeded { total: u64, limit: u64 },
}
impl GasPolicy {
    pub fn analyze(self, bc: &[u8]) -> Result<GasReport, GasError> {
        let mut o = 0;
        let (mut ops, mut total) = (0, 0);
        while o < bc.len() {
            let op = bc[o];
            let (w, c) = match op {
                1 => (9, self.const_i64),
                2 | 3 => (3, self.unary),
                0x10..=0x1a => (1, self.binary),
                0x20 => (5, self.call),
                0x21 => (1, self.return_cost),
                0x30 | 0x31 => (3, self.control_flow),
                0x40 => (1, self.pop_cost),
                _ => (return Err(GasError::UnknownOpcode(op))),
            };
            if o + w > bc.len() {
                return Err(GasError::TruncatedInstruction(op));
            }
            ops = ops.checked_add(1).ok_or(GasError::Overflow)?;
            total = total.checked_add(c).ok_or(GasError::Overflow)?;
            if total > self.gas_limit {
                return Err(GasError::LimitExceeded {
                    total,
                    limit: self.gas_limit,
                });
            }
            o += w
        }
        Ok(GasReport {
            operations: ops,
            total,
            limit: self.gas_limit,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic() {
        let p = GasPolicy {
            gas_limit: 100,
            const_i64: 2,
            unary: 1,
            binary: 1,
            call: 5,
            control_flow: 2,
            return_cost: 1,
            pop_cost: 1,
        };
        let b = [1, 0, 0, 0, 0, 0, 0, 0, 1, 0x21];
        assert_eq!(
            p.analyze(&b),
            Ok(GasReport {
                operations: 2,
                total: 3,
                limit: 100
            })
        )
    }
}
