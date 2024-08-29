use std::{
    any::TypeId,
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, BitAnd, BitOr, Not, Sub},
};
use typenum::*;

use super::reactive::{EditRGSO, RBaseGraphEnvironment};

pub type NonExistent = P9;

pub trait SlotTSMarker {
    type CountIsGreaterThanOrEqualToMin: Bit;
    type CountIsGreaterThanMin: Bit;
    type CountIsGreaterThanZero: Bit;
    type CountIsLessThanOrEqualToMax: Bit;
    type CountIsLessThanMax: Bit;
}
pub struct SlotTS<
    // Id,
    Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
    Min: Integer,
    MinIsNonExistent: Bit,
    Max: Integer,
    MaxIsNonExistent: Bit,
    ZeroAllowed: Bit,
>(
    PhantomData<(
        // Id,
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

pub trait FulfilledSlotTS {
    const IMPLEMENTED: bool = true;
}
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

pub trait SlotCanAddOne {
    const IMPLEMENTED: bool = true;
}
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

pub trait SlotCanSubtractOne {
    const IMPLEMENTED: bool = true;
}
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

pub trait FulfilledSlotTupleTS {
    const IMPLEMENTED: bool = true;
}
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

// ------------------------------------
struct SlotId<const ID: u32>;

pub struct OperativeTS<Id, State>(PhantomData<(Id, State)>);

pub trait IdAndStateGetter {
    type Id;
    type State;
}

impl<Id, State> IdAndStateGetter for OperativeTS<Id, State> {
    type Id = Id;
    type State = State;
}

pub trait IfThenElse<Condition: Bit> {
    type Output;
}

impl<T, F> IfThenElse<B1> for (T, F) {
    type Output = T;
}
impl<T, F> IfThenElse<B0> for (T, F) {
    type Output = F;
}

impl IdAndStateGetter for () {
    type Id = N1;
    type State = N1;
}

// --------------------------------------
// FIND
// --------------------------------------

pub trait TSInnerSearch<Id, IsMatch> {
    type Result;
}

impl<Id, T, Tail> TSInnerSearch<Id, B1> for (T, Tail)
where
    T: IdAndStateGetter,
{
    type Result = T;
}

impl<T, Id, Tail> TSInnerSearch<Id, B0> for (T, Tail)
where
    T: IdAndStateGetter,
    Id: IsEqual<T::Id>,
    Tail: TSInnerSearch<Id, Eq<Id, T::Id>>,
{
    type Result = Tail::Result;
}
impl<Id, T> TSInnerSearch<Id, B0> for T
where
    T: IdAndStateGetter,
    Id: IsEqual<T::Id>,
    (T, ()): IfThenElse<Eq<Id, T::Id>>,
{
    type Result = <(T, ()) as IfThenElse<Eq<Id, T::Id>>>::Output;
}
impl<Id, T> TSInnerSearch<Id, B1> for T
where
    T: IdAndStateGetter,
    Id: IsEqual<T::Id>,
    (T, ()): IfThenElse<Eq<Id, T::Id>>,
{
    type Result = <(T, ()) as IfThenElse<Eq<Id, T::Id>>>::Output;
}

pub trait TSSearch<Id> {
    type Result;
}

impl<Id> TSSearch<Id> for () {
    type Result = ();
}
impl<Id, First, Rest> TSSearch<Id> for (First, Rest)
where
    Id: IsEqual<First::Id>,
    (First, Rest): TSInnerSearch<Id, Eq<Id, First::Id>>,
    First: IdAndStateGetter,
{
    type Result = <(First, Rest) as TSInnerSearch<Id, Eq<Id, First::Id>>>::Result;
}
impl<Id, First> TSSearch<Id> for (First,)
where
    Id: IsEqual<First::Id>,
    First: TSInnerSearch<Id, Eq<Id, First::Id>>,
    First: IdAndStateGetter,
{
    type Result = <First as TSInnerSearch<Id, Eq<Id, First::Id>>>::Result;
}

// -----------------------------------------------
// ADD
// -----------------------------------------------

pub trait TSAddOperative<T> {
    type Result;
}

// impl<GTS, NewOperative> TSAddOperative<NewOperative> for (NewOperative, GTS)
// where
//     NewOperative: IdGetter,
//     // GTS: Search<NewOperative::Id>,
//     // <GTS as Search<NewOperative::Id>>::Result: IsEqual<B0>,
// {
//     type Result = (NewOperative, GTS);
// }
impl<GTS, NewOperative> TSAddOperative<NewOperative> for GTS
where
    NewOperative: IdAndStateGetter,
    // GTS: Search<NewOperative::Id>,
    // <GTS as Search<NewOperative::Id>>::Result: IsEqual<B0>,
{
    type Result = (NewOperative, GTS);
}

// ------------------------------------------
// edit
// ------------------------------------------

pub trait TSEditOperative<Id, NewState> {
    type Result;
}

impl<First, Rest, Id, NewState> TSEditOperative<Id, NewState> for (First, Rest)
where
    (First, Rest): TSSearch<Id>,
    Id: IsEqual<First::Id>,
    (First, Rest): ReplaceOperativeInTuple<Id, NewState, Eq<Id, First::Id>>,
    <(First, Rest) as TSSearch<Id>>::Result: IdAndStateGetter<Id = Id>,
    First: IdAndStateGetter,
{
    type Result =
        <(First, Rest) as ReplaceOperativeInTuple<Id, NewState, Eq<Id, First::Id>>>::Result;
}

pub trait ReplaceOperativeInTuple<Id, NewState, IsMatch> {
    type Result;
}
impl<Id, State, NewState, Rest> ReplaceOperativeInTuple<Id, NewState, B1>
    for (OperativeTS<Id, State>, Rest)
{
    type Result = (OperativeTS<Id, NewState>, Rest);
}

impl<FirstId, FirstState, Id, NewState, Rest> ReplaceOperativeInTuple<Id, NewState, B0>
    for (OperativeTS<FirstId, FirstState>, Rest)
where
    Id: IsEqual<FirstId>,
    Rest: ReplaceOperativeInTuple<Id, NewState, Eq<Id, FirstId>>,
{
    type Result = (
        OperativeTS<FirstId, FirstState>,
        <Rest as ReplaceOperativeInTuple<Id, NewState, Eq<Id, FirstId>>>::Result,
    );
}

// ---------------------------------------------------
// REMOVE
// ---------------------------------------------------

pub trait TSRemoveOperative<T> {
    type Result;
}

impl<First, Rest, Id> TSRemoveOperative<Id> for (First, Rest)
where
    (First, Rest): TSSearch<Id>,
    <(First, Rest) as TSSearch<Id>>::Result: IdAndStateGetter<Id = Id>,
    First: IdAndStateGetter,
    Id: IsEqual<First::Id>,
    (First, Rest): RemoveOperativeFromTuple<Id, Eq<Id, First::Id>>,
{
    type Result = <(First, Rest) as RemoveOperativeFromTuple<Id, Eq<Id, First::Id>>>::Result;
}

pub trait RemoveOperativeFromTuple<Id, IsMatch> {
    type Result;
}

impl<Id, State, Rest> RemoveOperativeFromTuple<Id, B1> for (OperativeTS<Id, State>, Rest) {
    type Result = Rest;
}

impl<FirstId, FirstState, Id, Rest> RemoveOperativeFromTuple<Id, B0>
    for (OperativeTS<FirstId, FirstState>, Rest)
where
    Id: Same<FirstId>,
    Rest: RemoveOperativeFromTuple<Id, <Id as Same<FirstId>>::Output>,
{
    type Result = (
        OperativeTS<FirstId, FirstState>,
        <Rest as RemoveOperativeFromTuple<Id, <Id as Same<FirstId>>::Output>>::Result,
    );
}

#[cfg(test)]
mod tests {
    use crate::post_generation::reactive::RGSO;

    use super::*;
    use typenum::*;

    type Slot1 = SlotTS<P1, Z0, B1, P3, B0, B0>;
    type Slot2 = SlotTS<P2, P1, B0, P3, B0, B0>;
    type Slot3 = SlotTS<P3, P1, B0, P5, B0, B1>;
    type Op1 = OperativeTS<P1, (Slot1,)>;
    type Op2 = OperativeTS<P2, (Slot2,)>;
    type Op3 = OperativeTS<P3, (Slot3, Slot1)>;

    #[test]
    fn test_search() {
        assert_eq!(
            <<(Op1, Op2) as TSSearch<P1>>::Result as IdAndStateGetter>::Id::to_i32(),
            1
        );
        assert_eq!(
            <<(Op1, Op2) as TSSearch<P2>>::Result as IdAndStateGetter>::Id::to_i32(),
            2
        );

        let result = <<() as TSSearch<U1>>::Result as IdAndStateGetter>::Id::to_i32();
        assert_eq!(result, -1);

        // Test searching in a single-element tuple
        assert_eq!(
            <<(Op1,) as TSSearch<P1>>::Result as IdAndStateGetter>::Id::to_i32(),
            1
        );
    }
    #[test]
    fn test_fulfilled_slot_ts() {
        // Valid slots
        assert_eq!(<Slot1 as FulfilledSlotTS>::IMPLEMENTED, true);
        assert_eq!(<Slot2 as FulfilledSlotTS>::IMPLEMENTED, true);
    }

    #[test]
    fn test_fulfilled_slot_tuple_ts() {
        // Valid tuples
        assert_eq!(<(Slot1,) as FulfilledSlotTupleTS>::IMPLEMENTED, true);
        assert_eq!(<(Slot1, Slot2) as FulfilledSlotTupleTS>::IMPLEMENTED, true);

        assert_eq!(
            <(Slot1, Slot2, Slot3) as FulfilledSlotTupleTS>::IMPLEMENTED,
            true
        );
    }

    #[test]
    fn test_slot_can_add_one() {
        // Can add one
        assert_eq!(<Slot1 as SlotCanAddOne>::IMPLEMENTED, true);

        // Cannot add one (at max)
        type SlotAtMax = SlotTS<P3, Z0, B1, P3, B0, B0>;
        // Uncomment to verify compilation error
        // assert_eq!(<SlotAtMax as SlotCanAddOne>::IMPLEMENTED, true);
    }

    #[test]
    fn test_slot_can_subtract_one() {
        // Can subtract one
        assert_eq!(<Slot2 as SlotCanSubtractOne>::IMPLEMENTED, true);

        // Cannot subtract one (at min)
        type SlotAtMin = SlotTS<P1, P1, B0, P3, B0, B0>;
        // Uncomment to verify compilation error
        // assert_eq!(<SlotAtMin as SlotCanSubtractOne>::IMPLEMENTED, true);
    }

    #[test]
    fn test_ts_add_operative() {
        type InitialState = (Op1,);
        type NewState = <InitialState as TSAddOperative<Op2>>::Result;

        // Ensure the new state includes Op1
        assert_eq!(
            <<NewState as TSSearch<P2>>::Result as IdAndStateGetter>::Id::to_i32(),
            2
        );

        type NewState2 = <InitialState as TSAddOperative<Op3>>::Result;
        assert_eq!(
            <<NewState2 as TSSearch<P3>>::Result as IdAndStateGetter>::Id::to_i32(),
            3
        );
        // Adding the same operative again should not compile
        // Uncomment to verify compilation error
        // type InvalidState = <NewState as TSAddOperative<Op1>>::Result;
    }

    // #[test]
    // fn test_ts_edit_operative() {
    //     type InitialState = (Op1, Op2);
    //     type NewOp1 = OperativeTS<P1, (P2,)>;
    //     // type NewOp2 = OperativeTS<P1, (P1,)>;
    //     type EditedState = <InitialState as TSEditOperative<P1, (P3,)>>::Result;

    //     // Ensure the state was edited correctly
    //     assert_eq!(
    //         <<EditedState as TSSearch<P1>>::Result as SlotTSMarker>::CountIsLessThanMax::to_bool(),
    //         false
    //     );
    //     // You might need to add more specific checks here to ensure the state was updated correctly
    // }

    // #[test]
    // fn test_ts_remove_operative() {
    //     type InitialState = RBaseGraphEnvironmentWithTypestate<DummySchema, (Op1, Op2)>;
    //     type RemovedState = <InitialState as TSRemoveOperative<P1>>::Result;

    //     // Ensure Op1 was removed
    //     // This should not compile if Op1 was successfully removed
    //     // Uncomment to verify compilation error
    //     // let _result = <<RemovedState as Search<P1>>::Result as IdGetter>::Id::to_i32();

    //     // Ensure Op2 is still present
    //     assert_eq!(
    //         <<RemovedState as TSSearch<P2>>::Result as IdAndStateGetter>::Id::to_i32(),
    //         2
    //     );
    // }
}

#[allow(dead_code)]
#[doc(hidden)]
/// ```compile_fail
///    use typenum::*;
///    use base_types::post_generation::type_level::*;
///    fn test() {
///        type InvalidSlot = SlotTS<P4, Z0, B1, P3, B0, B0>;
///        assert_eq!(<InvalidSlot as FulfilledSlotTS>::IMPLEMENTED, true);
///    }
/// ```
fn test_unfulfilled_slot() {}

#[allow(dead_code)]
#[doc(hidden)]
/// ```compile_fail
///    use typenum::*;
///    use base_types::post_generation::type_level::*;
///    type Slot1 = SlotTS<P1, Z0, B1, P3, B0, B0>;
///    fn test() {
///         type InvalidSlot = SlotTS<P4, Z0, B1, P3, B0, B0>;
///         assert_eq!(<(Slot1, InvalidSlot) as FulfilledSlotTupleTS>::IMPLEMENTED, true);
///    }
/// ```
fn test_unfulfilled_slot_tuple() {}

#[allow(dead_code)]
#[doc(hidden)]
/// ```compile_fail
///    use typenum::*;
///    use base_types::post_generation::type_level::*;
///    type Slot1 = SlotTS<P1, Z0, B1, P3, B0, B0>;
///    type Slot2 = SlotTS<P2, P1, B0, P3, B0, B0>;
///    type Op1 = OperativeTS<P1, (Slot1,)>;
///    type Op2 = OperativeTS<P2, (Slot2,)>;
///    fn test() {
///         let _result = <<(Op1, Op2) as Search<U3>>::Result as IdGetter>::Id::to_i32();
///    }
/// ```
fn test_failed_search() {}
