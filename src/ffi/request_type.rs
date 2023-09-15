use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequestType(u8);

impl RequestType {
    pub const ONE_BEST: Self = Self(1);
    pub const N_BEST: Self = Self(2);
    pub const PARTIAL: Self = Self(4);
    pub const MARGINAL_PROB: Self = Self(8);
    pub const ALTERNATIVE: Self = Self(16);
    pub const ALL_MORPHS: Self = Self(32);
    pub const ALLOC_SENTENCE: Self = Self(64);
}

impl RequestType {
    #[inline]
    pub(crate) fn from_int(n: u8) -> Self {
        Self(n)
    }

    #[inline]
    pub(crate) fn to_int(self) -> u8 {
        self.0
    }
}

impl BitAnd for RequestType {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl BitAndAssign for RequestType {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for RequestType {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl BitOrAssign for RequestType {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for RequestType {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl BitXorAssign for RequestType {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for RequestType {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self(127 ^ self.0)
    }
}
