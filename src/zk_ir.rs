//! ZK-IR: Zero-Knowledge Intermediate Representation output.
//!
//! Transforms EVM execution traces into a constraint-system-friendly IR
//! that can be consumed by zkEVM proving backends (e.g. Halo2, Plonky3).

/// A single ZK-IR instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkInstruction {
    /// Assign a value to a wire.
    Assign { wire: u32, value: [u8; 32] },
    /// Assert two wires are equal.
    AssertEq { lhs: u32, rhs: u32 },
    /// Add two wires, result in `out`.
    Add { lhs: u32, rhs: u32, out: u32 },
    /// Multiply two wires, result in `out`.
    Mul { lhs: u32, rhs: u32, out: u32 },
    /// Conditional: if `cond != 0` use `then_wire` else `else_wire`.
    Select { cond: u32, then_wire: u32, else_wire: u32, out: u32 },
    /// Range check: assert wire value fits in `bits` bits.
    RangeCheck { wire: u32, bits: u8 },
}

/// A ZK-IR program — a flat sequence of instructions.
#[derive(Debug, Default)]
pub struct ZkProgram {
    /// Next available wire index.
    next_wire: u32,
    /// Instruction sequence.
    pub instructions: Vec<ZkInstruction>,
}

impl ZkProgram {
    /// Allocate a fresh wire ID.
    pub fn alloc_wire(&mut self) -> u32 {
        let w = self.next_wire;
        self.next_wire += 1;
        w
    }

    /// Emit an `Assign` instruction, returning the wire.
    pub fn assign(&mut self, value: [u8; 32]) -> u32 {
        let wire = self.alloc_wire();
        self.instructions.push(ZkInstruction::Assign { wire, value });
        wire
    }

    /// Emit an `Add` instruction, returning the output wire.
    pub fn add(&mut self, lhs: u32, rhs: u32) -> u32 {
        let out = self.alloc_wire();
        self.instructions.push(ZkInstruction::Add { lhs, rhs, out });
        out
    }

    /// Emit a `Mul` instruction, returning the output wire.
    pub fn mul(&mut self, lhs: u32, rhs: u32) -> u32 {
        let out = self.alloc_wire();
        self.instructions.push(ZkInstruction::Mul { lhs, rhs, out });
        out
    }

    /// Emit a `RangeCheck` instruction.
    pub fn range_check(&mut self, wire: u32, bits: u8) {
        self.instructions.push(ZkInstruction::RangeCheck { wire, bits });
    }

    /// Total number of wires allocated.
    pub fn wire_count(&self) -> u32 {
        self.next_wire
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_simple_program() {
        let mut prog = ZkProgram::default();
        let a = prog.assign([1u8; 32]);
        let b = prog.assign([2u8; 32]);
        let c = prog.add(a, b);
        prog.range_check(c, 8);
        assert_eq!(prog.wire_count(), 3);
        assert_eq!(prog.instructions.len(), 4);
    }
}
