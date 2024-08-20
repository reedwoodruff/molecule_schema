use std::{
    marker::PhantomData,
    ops::{Add, BitAnd, BitOr, Sub},
};
use typenum::*;

pub type NonExistent = P9;

pub trait SlotTSMarker {
    type CountIsGreaterThanOrEqualToMin: Bit;
    type CountIsGreaterThanMin: Bit;
    type CountIsGreaterThanZero: Bit;
    type CountIsLessThanOrEqualToMax: Bit;
    type CountIsLessThanMax: Bit;
}
pub struct SlotTS<
    Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
    Min: Integer,
    MinIsNonExistent: Bit,
    Max: Integer,
    MaxIsNonExistent: Bit,
    ZeroAllowed: Bit,
>(
    PhantomData<(
        Count,
        Min,
        MinIsNonExistent,
        Max,
        MaxIsNonExistent,
        ZeroAllowed,
    )>,
);
impl<
        Count: Integer
            + IsGreaterOrEqual<Min>
            + IsGreater<Min>
            + IsLessOrEqual<Max>
            + IsLess<Max>
            + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > SlotTSMarker for SlotTS<Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>
{
    type CountIsGreaterThanOrEqualToMin = GrEq<Count, Min>;
    type CountIsGreaterThanMin = Gr<Count, Min>;
    type CountIsLessThanOrEqualToMax = LeEq<Count, Max>;
    type CountIsLessThanMax = Le<Count, Max>;
    type CountIsGreaterThanZero = Gr<Count, Z0>;
}

pub trait CountIsLessThanOrEqualToMax {}
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > CountIsLessThanOrEqualToMax
    for SlotTS<Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>
where
    SlotTS<Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>:
        SlotTSMarker<CountIsLessThanOrEqualToMax = B1>,
{
}

pub trait WithinUpperBounds {}
// If Max does not exist
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        // MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > WithinUpperBounds for SlotTS<Count, Min, MinIsNonExistent, Max, B1, ZeroAllowed>
{
}
// If Max does exist and count is less than it
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        // MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > WithinUpperBounds for SlotTS<Count, Min, MinIsNonExistent, Max, B0, ZeroAllowed>
where
    SlotTS<Count, Min, MinIsNonExistent, Max, B0, ZeroAllowed>: CountIsLessThanOrEqualToMax,
{
}

pub trait CountIsGreaterThanOrEqualToMin {}
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > CountIsGreaterThanOrEqualToMin
    for SlotTS<Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>
where
    SlotTS<Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>:
        SlotTSMarker<CountIsGreaterThanOrEqualToMin = B1>,
{
}

pub trait WithinLowerBounds {}
// If Min exists and Count is greater than it
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        // MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > WithinLowerBounds for SlotTS<Count, Min, B0, Max, MaxIsNonExistent, ZeroAllowed>
where
    SlotTS<Count, Min, B0, Max, MaxIsNonExistent, ZeroAllowed>: CountIsGreaterThanOrEqualToMin,
{
}
// If Min does not exist
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        // MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > WithinLowerBounds for SlotTS<Count, Min, B1, Max, MaxIsNonExistent, ZeroAllowed>
{
}

pub trait FulfilledSlotTS {}
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > FulfilledSlotTS for SlotTS<Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>
where
    SlotTS<Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>:
        WithinLowerBounds + WithinUpperBounds,
{
}

pub trait SlotCanAddOne {}
// If Max does not exist
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        // MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > SlotCanAddOne for SlotTS<Count, Min, MinIsNonExistent, Max, B1, ZeroAllowed>
{
}
// If Max does exist and count is less than it
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        // MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > SlotCanAddOne for SlotTS<Count, Min, MinIsNonExistent, Max, B0, ZeroAllowed>
where
    SlotTS<Count, Min, MinIsNonExistent, Max, B0, ZeroAllowed>:
        SlotTSMarker<CountIsLessThanMax = B1>,
{
}

pub trait SlotCanSubtractOne {}
// If Min does not exist
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        // MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > SlotCanSubtractOne for SlotTS<Count, Min, B1, Max, MaxIsNonExistent, ZeroAllowed>
where
    SlotTS<Count, Min, B1, Max, MaxIsNonExistent, ZeroAllowed>:
        SlotTSMarker<CountIsGreaterThanZero = B1>,
{
}
// If Min does exist and zero is not allowed
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        // MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        // ZeroAllowed: Bit,
    > SlotCanSubtractOne for SlotTS<Count, Min, B0, Max, MaxIsNonExistent, B0>
where
    SlotTS<Count, Min, B0, Max, MaxIsNonExistent, B0>: SlotTSMarker<CountIsGreaterThanMin = B1>,
{
}
// If Min does exist and zero is allowed
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        // MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        // ZeroAllowed: Bit,
    > SlotCanSubtractOne for SlotTS<Count, Min, B0, Max, MaxIsNonExistent, B1>
where
    SlotTS<Count, Min, B0, Max, MaxIsNonExistent, B1>: SlotTSMarker<CountIsGreaterThanZero = B1>,
{
}

pub trait FulfilledSlotTupleTS {}
impl<A> FulfilledSlotTupleTS for (A,) where A: FulfilledSlotTS {}
impl<A, B> FulfilledSlotTupleTS for (A, B)
where
    A: FulfilledSlotTS,
    B: FulfilledSlotTS,
{
}
impl<A, B, C> FulfilledSlotTupleTS for (A, B, C)
where
    A: FulfilledSlotTS,
    B: FulfilledSlotTS,
    C: FulfilledSlotTS,
{
}

impl<A, B, C, D> FulfilledSlotTupleTS for (A, B, C, D)
where
    A: FulfilledSlotTS,
    B: FulfilledSlotTS,
    C: FulfilledSlotTS,
    D: FulfilledSlotTS,
{
}
impl<A, B, C, D, E> FulfilledSlotTupleTS for (A, B, C, D, E)
where
    A: FulfilledSlotTS,
    B: FulfilledSlotTS,
    C: FulfilledSlotTS,
    D: FulfilledSlotTS,
    E: FulfilledSlotTS,
{
}
impl<A, B, C, D, E, F> FulfilledSlotTupleTS for (A, B, C, D, E, F)
where
    A: FulfilledSlotTS,
    B: FulfilledSlotTS,
    C: FulfilledSlotTS,
    D: FulfilledSlotTS,
    E: FulfilledSlotTS,
    F: FulfilledSlotTS,
{
}
impl<A, B, C, D, E, F, G> FulfilledSlotTupleTS for (A, B, C, D, E, F, G)
where
    A: FulfilledSlotTS,
    B: FulfilledSlotTS,
    C: FulfilledSlotTS,
    D: FulfilledSlotTS,
    E: FulfilledSlotTS,
    F: FulfilledSlotTS,
    G: FulfilledSlotTS,
{
}
impl<A, B, C, D, E, F, G, H> FulfilledSlotTupleTS for (A, B, C, D, E, F, G, H)
where
    A: FulfilledSlotTS,
    B: FulfilledSlotTS,
    C: FulfilledSlotTS,
    D: FulfilledSlotTS,
    E: FulfilledSlotTS,
    F: FulfilledSlotTS,
    G: FulfilledSlotTS,
    H: FulfilledSlotTS,
{
}
impl<A, B, C, D, E, F, G, H, I> FulfilledSlotTupleTS for (A, B, C, D, E, F, G, H, I)
where
    A: FulfilledSlotTS,
    B: FulfilledSlotTS,
    C: FulfilledSlotTS,
    D: FulfilledSlotTS,
    E: FulfilledSlotTS,
    F: FulfilledSlotTS,
    G: FulfilledSlotTS,
    H: FulfilledSlotTS,
    I: FulfilledSlotTS,
{
}
impl<A, B, C, D, E, F, G, H, I, J> FulfilledSlotTupleTS for (A, B, C, D, E, F, G, H, I, J)
where
    A: FulfilledSlotTS,
    B: FulfilledSlotTS,
    C: FulfilledSlotTS,
    D: FulfilledSlotTS,
    E: FulfilledSlotTS,
    F: FulfilledSlotTS,
    G: FulfilledSlotTS,
    H: FulfilledSlotTS,
    I: FulfilledSlotTS,
    J: FulfilledSlotTS,
{
}
