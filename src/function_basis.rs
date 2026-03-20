use std::ops::{BitAnd, BitOr, BitXor, Not};

pub trait FunctionSet<T>: Copy {
    const CARDINALITY: usize;
    fn apply(self, lhs: T, rhs: T) -> T;
}

#[derive(Clone, Copy, Debug)]
pub enum BooleanOp {
    And,
    Or,
    Xor,
    Nand,
}

#[derive(Clone, Copy, Debug)]
pub enum NandCircuit {
    Nand,
}

impl<B> FunctionSet<B> for NandCircuit
where
    B: BitAnd<Output = B> + Not<Output = B>
{
    const CARDINALITY: usize = 1;
    fn apply(self, lhs: B, rhs: B) -> B {
        match self {
            Self::Nand => !(lhs & rhs),
        }
    }
}


impl FunctionSet<bool> for BooleanOp {
    const CARDINALITY: usize = 4;
    fn apply(self, lhs: bool, rhs: bool) -> bool {
        match self {
            Self::And => lhs & rhs,
            Self::Or => lhs | rhs,
            Self::Xor => lhs ^ rhs,
            Self::Nand => !(lhs & rhs),
        }
    }
}


impl FunctionSet<u64> for BooleanOp {
    const CARDINALITY: usize = 4;
    fn apply(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::And => lhs & rhs,
            Self::Or => lhs | rhs,
            Self::Xor => lhs ^ rhs,
            Self::Nand => !(lhs & rhs),
        }
    }
}
