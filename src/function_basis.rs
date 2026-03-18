pub trait FunctionSet<T>: Copy {
    const CARDINALITY: usize;
    fn apply(self, lhs: T, rhs: T) -> T;
}

#[derive(Clone, Copy)]
pub enum BooleanOp {
    And,
    Or,
    Xor,
    Nand,
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
