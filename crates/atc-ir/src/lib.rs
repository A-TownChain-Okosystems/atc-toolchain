//! Toolchain-facing ATC-IR model. Canonical language semantics remain in atclang.
pub const IR_SCHEMA: &str = "ATC-IR-001";
pub const IR_VERSION: u16 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    I64,
    Bool,
    Unit,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    ConstI64,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Return,
    Pop,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    ConstI64 {
        value: i64,
        ty: Type,
    },
    Unary {
        op: Opcode,
        operand: u32,
        ty: Type,
    },
    Binary {
        op: Opcode,
        lhs: u32,
        rhs: u32,
        ty: Type,
    },
    Return {
        value: u32,
        ty: Type,
    },
    Pop {
        value: u32,
        ty: Type,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub version: u16,
    pub nodes: Vec<Node>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrError {
    UnsupportedVersion(u16),
    UntypedNode,
    InvalidNodeReference { node: u32 },
    InvalidOperandType { node: u32 },
}
impl Module {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            version: IR_VERSION,
            nodes,
        }
    }
    pub fn validate(&self) -> Result<(), IrError> {
        if self.version != IR_VERSION {
            return Err(IrError::UnsupportedVersion(self.version));
        }
        for (i, n) in self.nodes.iter().enumerate() {
            let i = i as u32;
            match n {
                Node::ConstI64 { ty, .. } if *ty != Type::I64 => {
                    return Err(IrError::InvalidOperandType { node: i })
                }
                Node::Unary { operand, ty, .. } => {
                    self.reference(*operand)?;
                    if *ty == Type::Unit {
                        return Err(IrError::UntypedNode);
                    }
                }
                Node::Binary { lhs, rhs, ty, .. } => {
                    self.reference(*lhs)?;
                    self.reference(*rhs)?;
                    if *ty == Type::Unit {
                        return Err(IrError::UntypedNode);
                    }
                }
                Node::Return { value, ty } | Node::Pop { value, ty } => {
                    self.reference(*value)?;
                    if *ty == Type::Unit {
                        return Err(IrError::UntypedNode);
                    }
                }
                Node::ConstI64 { .. } => {}
            }
        }
        Ok(())
    }
    fn reference(&self, r: u32) -> Result<(), IrError> {
        if (r as usize) < self.nodes.len() {
            Ok(())
        } else {
            Err(IrError::InvalidNodeReference { node: r })
        }
    }
    pub fn encode_canonical(&self) -> Result<Vec<u8>, IrError> {
        self.validate()?;
        let mut o = Vec::new();
        o.extend_from_slice(&self.version.to_le_bytes());
        o.extend_from_slice(&(self.nodes.len() as u32).to_le_bytes());
        for n in &self.nodes {
            match n {
                Node::ConstI64 { value, ty } => {
                    o.extend_from_slice(&[1, type_tag(*ty)]);
                    o.extend_from_slice(&value.to_le_bytes())
                }
                Node::Unary { op, operand, ty } => {
                    o.extend_from_slice(&[2, type_tag(*ty), op_tag(*op)]);
                    o.extend_from_slice(&operand.to_le_bytes())
                }
                Node::Binary { op, lhs, rhs, ty } => {
                    o.extend_from_slice(&[3, type_tag(*ty), op_tag(*op)]);
                    o.extend_from_slice(&lhs.to_le_bytes());
                    o.extend_from_slice(&rhs.to_le_bytes())
                }
                Node::Return { value, ty } | Node::Pop { value, ty } => {
                    o.extend_from_slice(&[
                        if matches!(n, Node::Return { .. }) {
                            4
                        } else {
                            5
                        },
                        type_tag(*ty),
                    ]);
                    o.extend_from_slice(&value.to_le_bytes())
                }
            }
        }
        Ok(o)
    }
}
fn type_tag(t: Type) -> u8 {
    match t {
        Type::I64 => 1,
        Type::Bool => 2,
        Type::Unit => 3,
    }
}
fn op_tag(o: Opcode) -> u8 {
    match o {
        Opcode::ConstI64 => 1,
        Opcode::Add => 2,
        Opcode::Sub => 3,
        Opcode::Mul => 4,
        Opcode::Div => 5,
        Opcode::Neg => 6,
        Opcode::Eq => 7,
        Opcode::Ne => 8,
        Opcode::Lt => 9,
        Opcode::Gt => 10,
        Opcode::Le => 11,
        Opcode::Ge => 12,
        Opcode::Return => 13,
        Opcode::Pop => 14,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic() {
        let m = Module::new(vec![Node::ConstI64 {
            value: 42,
            ty: Type::I64,
        }]);
        assert_eq!(m.encode_canonical().unwrap(), m.encode_canonical().unwrap())
    }
    #[test]
    fn reject_untyped() {
        let m = Module::new(vec![Node::ConstI64 {
            value: 1,
            ty: Type::Unit,
        }]);
        assert_eq!(m.validate(), Err(IrError::InvalidOperandType { node: 0 }))
    }
}
