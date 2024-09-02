use std::marker::PhantomData;

use typenum::*;

pub struct CompId<A, B, C, D>(PhantomData<(A, B, C, D)>);
// impl<A: Unsigned, B: Unsigned, C: Unsigned, D: Unsigned> CompId for CompositeId<A, B, C, D> {}

// Implement PartialEq for CompositeId
impl<A, B, C, D> PartialEq for CompId<A, B, C, D> {
    fn eq(&self, _other: &Self) -> bool {
        // This is always true because if the types are the same, the values are the same
        true
    }
}
impl<A: Unsigned, B: Unsigned, C: Unsigned, D: Unsigned> CompId<A, B, C, D> {
    pub fn new() -> Self {
        CompId(PhantomData)
    }
}

impl<A: Unsigned, B: Unsigned, C: Unsigned, D: Unsigned> IdToU32 for CompId<A, B, C, D> {
    fn to_u32() -> u32 {
        (A::to_u32() << 24) | (B::to_u32() << 16) | (C::to_u32() << 8) | D::to_u32()
    }
}

pub trait IdToU32 {
    fn to_u32() -> u32;
}
