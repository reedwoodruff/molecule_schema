use molecule_core::CompId;
use std::{
    marker::PhantomData,
    ops::{Add, BitAnd, BitOr},
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
    Id,
    Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
    Min: Integer,
    MinIsNonExistent: Bit,
    Max: Integer,
    MaxIsNonExistent: Bit,
    ZeroAllowed: Bit,
>(
    PhantomData<(
        Id,
        Count,
        Min,
        MinIsNonExistent,
        Max,
        MaxIsNonExistent,
        ZeroAllowed,
    )>,
);
impl<
        Id,
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
    > SlotTSMarker
    for SlotTS<Id, Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>
{
    type CountIsGreaterThanOrEqualToMin = GrEq<Count, Min>;
    type CountIsGreaterThanMin = Gr<Count, Min>;
    type CountIsLessThanOrEqualToMax = LeEq<Count, Max>;
    type CountIsLessThanMax = Le<Count, Max>;
    type CountIsGreaterThanZero = Gr<Count, Z0>;
}

pub trait CountIsLessThanOrEqualToMax {}
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > CountIsLessThanOrEqualToMax
    for SlotTS<Id, Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>
where
    SlotTS<Id, Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>:
        SlotTSMarker<CountIsLessThanOrEqualToMax = B1>,
{
}

pub trait WithinUpperBounds {}
// If Max does not exist
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        // MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > WithinUpperBounds for SlotTS<Id, Count, Min, MinIsNonExistent, Max, B1, ZeroAllowed>
{
}
// If Max does exist and count is less than it
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        // MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > WithinUpperBounds for SlotTS<Id, Count, Min, MinIsNonExistent, Max, B0, ZeroAllowed>
where
    SlotTS<Id, Count, Min, MinIsNonExistent, Max, B0, ZeroAllowed>: CountIsLessThanOrEqualToMax,
{
}

pub trait CountIsGreaterThanOrEqualToMin {}
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > CountIsGreaterThanOrEqualToMin
    for SlotTS<Id, Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>
where
    SlotTS<Id, Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>:
        SlotTSMarker<CountIsGreaterThanOrEqualToMin = B1>,
{
}

pub trait WithinLowerBounds {}
// If Min exists and Count is greater than it
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        // MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > WithinLowerBounds for SlotTS<Id, Count, Min, B0, Max, MaxIsNonExistent, ZeroAllowed>
where
    SlotTS<Id, Count, Min, B0, Max, MaxIsNonExistent, ZeroAllowed>: CountIsGreaterThanOrEqualToMin,
{
}
// If Min does not exist
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        // MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > WithinLowerBounds for SlotTS<Id, Count, Min, B1, Max, MaxIsNonExistent, ZeroAllowed>
{
}

pub trait FulfilledSlotTS {
    const IMPLEMENTED: bool = true;
}
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > FulfilledSlotTS
    for SlotTS<Id, Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>
where
    SlotTS<Id, Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>:
        WithinLowerBounds + WithinUpperBounds,
{
}

pub trait SlotCanAddOne {
    const IMPLEMENTED: bool = true;
}
// If Max does not exist
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        // MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > SlotCanAddOne for SlotTS<Id, Count, Min, MinIsNonExistent, Max, B1, ZeroAllowed>
{
}
// If Max does exist and count is less than it
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        // MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > SlotCanAddOne for SlotTS<Id, Count, Min, MinIsNonExistent, Max, B0, ZeroAllowed>
where
    SlotTS<Id, Count, Min, MinIsNonExistent, Max, B0, ZeroAllowed>:
        SlotTSMarker<CountIsLessThanMax = B1>,
{
}

pub trait SlotCanSubtractOne {
    const IMPLEMENTED: bool = true;
}
// If Min does not exist
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        // MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > SlotCanSubtractOne for SlotTS<Id, Count, Min, B1, Max, MaxIsNonExistent, ZeroAllowed>
where
    SlotTS<Id, Count, Min, B1, Max, MaxIsNonExistent, ZeroAllowed>:
        SlotTSMarker<CountIsGreaterThanZero = B1>,
{
}
// If Min does exist and zero is not allowed
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        // MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        // ZeroAllowed: Bit,
    > SlotCanSubtractOne for SlotTS<Id, Count, Min, B0, Max, MaxIsNonExistent, B0>
where
    SlotTS<Id, Count, Min, B0, Max, MaxIsNonExistent, B0>: SlotTSMarker<CountIsGreaterThanMin = B1>,
{
}
// If Min does exist and zero is allowed
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        // MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        // ZeroAllowed: Bit,
    > SlotCanSubtractOne for SlotTS<Id, Count, Min, B0, Max, MaxIsNonExistent, B1>
where
    SlotTS<Id, Count, Min, B0, Max, MaxIsNonExistent, B1>:
        SlotTSMarker<CountIsGreaterThanZero = B1>,
{
}

// ------------------------------------
// Define a composite Id type
// ------------------------------------

pub trait IdIsEqual<Rhs = Self> {
    type Output: Bit;
}
type IdEq<Lhs, Rhs> = <Lhs as IdIsEqual<Rhs>>::Output;

impl<
        A1: Unsigned,
        B1: Unsigned,
        C1: Unsigned,
        D1: Unsigned,
        A2: Unsigned,
        B2: Unsigned,
        C2: Unsigned,
        D2: Unsigned,
    > IdIsEqual<CompId<A2, B2, C2, D2>> for CompId<A1, B1, C1, D1>
where
    A1: IsEqual<A2>,
    B1: IsEqual<B2>,
    C1: IsEqual<C2>,
    D1: IsEqual<D2>,
    Eq<A1, A2>: BitAnd<Eq<B1, B2>>,
    And<Eq<A1, A2>, Eq<B1, B2>>: Bit,
    And<Eq<A1, A2>, Eq<B1, B2>>: BitAnd<Eq<C1, C2>>,
    And<And<Eq<A1, A2>, Eq<B1, B2>>, Eq<C1, C2>>: Bit,
    And<And<Eq<A1, A2>, Eq<B1, B2>>, Eq<C1, C2>>: BitAnd<Eq<D1, D2>>,
    And<And<And<Eq<A1, A2>, Eq<B1, B2>>, Eq<C1, C2>>, Eq<D1, D2>>: Bit,
{
    type Output = And<And<And<Eq<A1, A2>, Eq<B1, B2>>, Eq<C1, C2>>, Eq<D1, D2>>;
}
// N1 is used to denote a not-found item, so it will never be equal to any CompositeId
impl<A: Unsigned, B: Unsigned, C: Unsigned, D: Unsigned> IdIsEqual<CompId<A, B, C, D>> for N1 {
    type Output = B0;
}
impl<A: Unsigned, B: Unsigned, C: Unsigned, D: Unsigned> IdIsEqual<N1> for CompId<A, B, C, D> {
    type Output = B0;
}
impl IdIsEqual<N1> for N1 {
    type Output = B1;
}

pub struct OperativeTS<Id, State>(PhantomData<(Id, State)>);

pub trait IdGetter {
    type Id;
}
impl<IdA, IdB, IdC, IdD, State> IdGetter for OperativeTS<CompId<IdA, IdB, IdC, IdD>, State> {
    type Id = CompId<IdA, IdB, IdC, IdD>;
}
impl<IdA, IdB, IdC, IdD, State> IdGetter for (OperativeTS<CompId<IdA, IdB, IdC, IdD>, State>,) {
    type Id = CompId<IdA, IdB, IdC, IdD>;
}
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > IdGetter for SlotTS<Id, Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>
{
    type Id = Id;
}
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > IdGetter for (SlotTS<Id, Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>,)
{
    type Id = Id;
}
impl IdGetter for () {
    type Id = N1;
}
// impl<Lhs, Rhs> IdGetter for (Lhs, Rhs)
// where
//     Lhs: IdGetter,
// {
//     type Id = Lhs::Id;
// }
impl<Id, State, Rest> IdGetter for (OperativeTS<Id, State>, Rest) {
    type Id = Id;
}

// pub trait LinkedTypeList {
//     type Tail;
// }
// impl<Lhs, Rhs> LinkedTypeList for (Lhs, Rhs) {
//     type Tail = Rhs;
// }

pub trait StateGetter {
    type State;
}
impl<Id, State> StateGetter for OperativeTS<Id, State> {
    type State = State;
}
impl<
        Id,
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        Min: Integer,
        MinIsNonExistent: Bit,
        Max: Integer,
        MaxIsNonExistent: Bit,
        ZeroAllowed: Bit,
    > StateGetter for SlotTS<Id, Count, Min, MinIsNonExistent, Max, MaxIsNonExistent, ZeroAllowed>
{
    type State = Count;
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

// --------------------------------------
// FIND
// --------------------------------------

pub trait TSSearch<Id> {
    type Result;
}

impl<Id> TSSearch<Id> for () {
    type Result = ();
}
impl<Id, First, Rest> TSSearch<Id> for (First, Rest)
where
    Id: IdIsEqual<First::Id>,
    Id: IdIsEqual<Rest::Id>,
    (First, Rest): TSInnerSearch<Id, IdEq<Id, First::Id>, IdEq<Id, Rest::Id>>,
    First: IdGetter,
    Rest: IdGetter,
{
    type Result =
        <(First, Rest) as TSInnerSearch<Id, IdEq<Id, First::Id>, IdEq<Id, Rest::Id>>>::Result;
}

impl<Id, First> TSSearch<Id> for (First,)
where
    Id: IdIsEqual<First::Id>,
    (First,): TSInnerSearch<Id, IdEq<Id, First::Id>, B0>,
    First: IdGetter,
{
    type Result = <(First,) as TSInnerSearch<Id, IdEq<Id, First::Id>, B0>>::Result;
}

pub trait TSInnerSearch<Id, IsMatch, IsNextMatch> {
    type Result;
}

impl<Id, T, Tail, NextMatch> TSInnerSearch<Id, B1, NextMatch> for (T, Tail)
where
    T: IdGetter,
{
    type Result = T;
}

impl<T, Id, Tail> TSInnerSearch<Id, B0, B1> for (T, Tail)
where
    T: IdGetter,
    Id: IdIsEqual<T::Id>,
    Tail: TSInnerSearch<Id, B1, B0>,
{
    type Result = Tail::Result;
}
impl<First, Second, Id, Tail> TSInnerSearch<Id, B0, B0> for (First, (Second, Tail))
where
    Tail: IdGetter,
    Id: IdIsEqual<Tail::Id>,
    (Second, Tail): TSInnerSearch<Id, B0, IdEq<Id, Tail::Id>>,
{
    type Result = <(Second, Tail) as TSInnerSearch<Id, B0, IdEq<Id, Tail::Id>>>::Result;
}
impl<First, Id> TSInnerSearch<Id, B0, B0> for (First, ()) {
    type Result = ();
}
impl<Id, T, NextMatch> TSInnerSearch<Id, B1, NextMatch> for (T,)
where
    T: IdGetter,
    Id: IdIsEqual<T::Id>,
    (T, ()): IfThenElse<IdEq<Id, T::Id>>,
{
    type Result = <(T, ()) as IfThenElse<IdEq<Id, T::Id>>>::Output;
}
impl<Id, T, NextMatch> TSInnerSearch<Id, B0, NextMatch> for (T,)
where
    T: IdGetter,
    Id: IdIsEqual<T::Id>,
    (T, ()): IfThenElse<IdEq<Id, T::Id>>,
{
    type Result = <(T, ()) as IfThenElse<IdEq<Id, T::Id>>>::Output;
}

// -----------------------------------------------
// ADD
// -----------------------------------------------

pub trait TSAddToList<T> {
    type Result;
}

impl<T, NewItem> TSAddToList<NewItem> for T
where
    NewItem: IdGetter,
    T: ItemExists<NewItem::Id>,
    T::Exists: AssertItemDoesNotExist,
{
    type Result = (NewItem, T);
}
pub trait ItemExists<Id> {
    type Exists: Bit;
}

impl<Id> ItemExists<Id> for () {
    type Exists = B0;
}

impl<Id, First, Rest> ItemExists<Id> for (First, Rest)
where
    Id: IdIsEqual<First::Id>,
    First: IdGetter,
    Rest: ItemExists<Id>,
    IdEq<Id, First::Id>: BitOr<Rest::Exists>,
    <<Id as IdIsEqual<<First as IdGetter>::Id>>::Output as BitOr<
        <Rest as ItemExists<Id>>::Exists,
    >>::Output: Bit,
{
    type Exists = Or<IdEq<Id, First::Id>, Rest::Exists>;
}
pub trait AssertItemDoesNotExist {}

impl AssertItemDoesNotExist for B0 {}

// ------------------------------------------
// EDIT
// ------------------------------------------

pub trait TSEdit<Id, NewState> {
    type Result;
}

impl<Id, NewState> TSEdit<Id, NewState> for () {
    type Result = ();
}
impl<Id, First, Rest, NewState> TSEdit<Id, NewState> for (First, Rest)
where
    Id: IdIsEqual<First::Id>,
    Id: IdIsEqual<Rest::Id>,
    (First, Rest): TSInnerEdit<Id, IdEq<Id, First::Id>, IdEq<Id, Rest::Id>, NewState>,
    First: IdGetter,
    Rest: IdGetter,
{
    type Result = <(First, Rest) as TSInnerEdit<
        Id,
        IdEq<Id, First::Id>,
        IdEq<Id, Rest::Id>,
        NewState,
    >>::Result;
}

impl<Id, First, NewState> TSEdit<Id, NewState> for (First,)
where
    Id: IdIsEqual<First::Id>,
    (First,): TSInnerEdit<Id, IdEq<Id, First::Id>, B0, NewState>,
    First: IdGetter,
{
    type Result = <(First,) as TSInnerEdit<Id, IdEq<Id, First::Id>, B0, NewState>>::Result;
}

pub trait TSInnerEdit<Id, IsMatch, IsNextMatch, NewState> {
    type Result;
}

impl<Id, T, Tail, NextMatch, NewState> TSInnerEdit<Id, B1, NextMatch, NewState> for (T, Tail)
where
    T: IdGetter,
{
    type Result = (OperativeTS<Id, NewState>, Tail);
}

impl<T, Id, Tail, NewState> TSInnerEdit<Id, B0, B1, NewState> for (T, Tail)
where
    T: IdGetter,
    Id: IdIsEqual<T::Id>,
    Tail: TSInnerEdit<Id, B1, B0, NewState>,
{
    type Result = (T, Tail::Result);
}
impl<First, Second, Id, Tail, NewState> TSInnerEdit<Id, B0, B0, NewState>
    for (First, (Second, Tail))
where
    Tail: IdGetter,
    Id: IdIsEqual<Tail::Id>,
    (Second, Tail): TSInnerEdit<Id, B0, IdEq<Id, Tail::Id>, NewState>,
{
    type Result = (
        First,
        <(Second, Tail) as TSInnerEdit<Id, B0, IdEq<Id, Tail::Id>, NewState>>::Result,
    );
}
impl<First, Id, NewState> TSInnerEdit<Id, B0, B0, NewState> for (First, ()) {
    type Result = (First, ());
}
impl<Id, T, NextMatch, NewState> TSInnerEdit<Id, B1, NextMatch, NewState> for (T,) {
    type Result = (OperativeTS<Id, NewState>,);
}
impl<Id, T, NextMatch, NewState> TSInnerEdit<Id, B0, NextMatch, NewState> for (T,) {
    type Result = ();
}

// ---------------------------------------------------
// REMOVE
// ---------------------------------------------------

pub trait TSRemove<Id> {
    type Result;
}

impl<Id> TSRemove<Id> for () {
    type Result = ();
}
impl<Id, First, Rest> TSRemove<Id> for (First, Rest)
where
    Id: IdIsEqual<First::Id>,
    Id: IdIsEqual<Rest::Id>,
    (First, Rest): TSInnerRemove<Id, IdEq<Id, First::Id>, IdEq<Id, Rest::Id>>,
    First: IdGetter,
    Rest: IdGetter,
{
    type Result =
        <(First, Rest) as TSInnerRemove<Id, IdEq<Id, First::Id>, IdEq<Id, Rest::Id>>>::Result;
}

impl<Id, First> TSRemove<Id> for (First,)
where
    Id: IdIsEqual<First::Id>,
    (First,): TSInnerRemove<Id, IdEq<Id, First::Id>, B0>,
    First: IdGetter,
{
    type Result = <(First,) as TSInnerRemove<Id, IdEq<Id, First::Id>, B0>>::Result;
}

pub trait TSInnerRemove<Id, IsMatch, IsNextMatch> {
    type Result;
}

impl<Id, T, Tail, NextMatch> TSInnerRemove<Id, B1, NextMatch> for (T, Tail)
where
    T: IdGetter,
{
    type Result = Tail;
}

impl<T, Id, Tail> TSInnerRemove<Id, B0, B1> for (T, Tail)
where
    T: IdGetter,
    Id: IdIsEqual<T::Id>,
    Tail: TSInnerRemove<Id, B1, B0>,
{
    type Result = (T, Tail::Result);
}
impl<First, Second, Id, Tail> TSInnerRemove<Id, B0, B0> for (First, (Second, Tail))
where
    Tail: IdGetter,
    Id: IdIsEqual<Tail::Id>,
    (Second, Tail): TSInnerRemove<Id, B0, IdEq<Id, Tail::Id>>,
{
    type Result = (
        First,
        <(Second, Tail) as TSInnerRemove<Id, B0, IdEq<Id, Tail::Id>>>::Result,
    );
}
impl<First, Id> TSInnerRemove<Id, B0, B0> for (First, ()) {
    type Result = (First, ());
}
impl<Id, T, NextMatch> TSInnerRemove<Id, B1, NextMatch> for (T,) {
    type Result = ();
}
impl<Id, T, NextMatch> TSInnerRemove<Id, B0, NextMatch> for (T,) {
    type Result = (T,);
}

// ------------------------------------------
// Compound ID
// ------------------------------------------

trait AddOneId<IsMax> {
    type Output; // : AddOneId;
}
impl AddOneId<B1> for CompId<U255, U255, U255, U255> {
    type Output = CompId<U0, U0, U0, U0>;
}

impl<A, B, C, D> AddOneId<B0> for CompId<A, B, C, D>
where
    And<NotEq<A, U255>, And<NotEq<B, U255>, And<NotEq<C, U255>, NotEq<D, U255>>>>: Same<B1>,
    A: Add<B1> + Cmp<U255> + IsEqual<U255> + IsNotEqual<U255> + Unsigned,
    B: Add<B1> + Cmp<U255> + IsEqual<U255> + IsNotEqual<U255> + Unsigned,
    C: Add<B1> + Cmp<U255> + IsEqual<U255> + IsNotEqual<U255> + Unsigned,
    D: Add<B1> + Cmp<U255> + IsEqual<U255> + IsNotEqual<U255> + Unsigned,
    (
        CompId<UTerm, UTerm, UTerm, UTerm>,
        CompId<<A as Add<B1>>::Output, UTerm, UTerm, UTerm>,
    ): IfThenElse<<A as IsEqual<U255>>::Output>,
    (
        <(
            CompId<U0, U0, U0, U0>,
            CompId<<A as Add<B1>>::Output, U0, U0, U0>,
        ) as IfThenElse<<A as IsEqual<U255>>::Output>>::Output,
        CompId<A, <B as Add<B1>>::Output, U0, U0>,
    ): IfThenElse<<B as IsEqual<U255>>::Output>,
    (
        <(
            <(
                CompId<U0, U0, U0, U0>,
                CompId<<A as Add<B1>>::Output, U0, U0, U0>,
            ) as IfThenElse<<A as IsEqual<U255>>::Output>>::Output,
            CompId<A, <B as Add<B1>>::Output, U0, U0>,
        ) as IfThenElse<<B as IsEqual<U255>>::Output>>::Output,
        CompId<A, B, <C as Add<B1>>::Output, U0>,
    ): IfThenElse<<C as IsEqual<U255>>::Output>,
    (
        <(
            <(
                <(
                    CompId<U0, U0, U0, U0>,
                    CompId<<A as Add<B1>>::Output, U0, U0, U0>,
                ) as IfThenElse<<A as IsEqual<U255>>::Output>>::Output,
                CompId<A, <B as Add<B1>>::Output, U0, U0>,
            ) as IfThenElse<<B as IsEqual<U255>>::Output>>::Output,
            CompId<A, B, <C as Add<B1>>::Output, U0>,
        ) as IfThenElse<<C as IsEqual<U255>>::Output>>::Output,
        CompId<A, B, C, <D as Add<B1>>::Output>,
    ): IfThenElse<<D as IsEqual<U255>>::Output>,
    <C as IsNotEqual<U255>>::Output: BitAnd<<D as IsNotEqual<U255>>::Output>,
    <B as IsNotEqual<U255>>::Output: BitAnd<
        <<C as IsNotEqual<U255>>::Output as BitAnd<<D as IsNotEqual<U255>>::Output>>::Output,
    >,
    <A as IsNotEqual<U255>>::Output: BitAnd<
        <<B as IsNotEqual<U255>>::Output as BitAnd<
            <<C as IsNotEqual<U255>>::Output as BitAnd<<D as IsNotEqual<U255>>::Output>>::Output,
        >>::Output,
    >,
    // <(
    //     <(
    //         <(
    //             <(CompId<U0, U0, U0, U0>, CompId<Add1<A>, U0, U0, U0>) as IfThenElse<
    //                 Eq<A, U255>,
    //             >>::Output,
    //             CompId<A, Add1<B>, U0, U0>,
    //         ) as IfThenElse<Eq<B, U255>>>::Output,
    //         CompId<A, B, Add1<C>, U0>,
    //     ) as IfThenElse<Eq<C, U255>>>::Output,
    //     CompId<A, B, C, Add1<D>>,
    // ) as IfThenElse<Eq<D, U255>>>::Output: AddOneId,
{
    type Output = <(
        <(
            <(
                <(CompId<U0, U0, U0, U0>, CompId<Add1<A>, U0, U0, U0>) as IfThenElse<
                    Eq<A, U255>,
                >>::Output,
                CompId<A, Add1<B>, U0, U0>,
            ) as IfThenElse<Eq<B, U255>>>::Output,
            CompId<A, B, Add1<C>, U0>,
        ) as IfThenElse<Eq<C, U255>>>::Output,
        CompId<A, B, C, Add1<D>>,
    ) as IfThenElse<Eq<D, U255>>>::Output;
}

type AddUno<Lhs> = <Lhs as AddOneId<B0>>::Output;

#[cfg(test)]
mod tests {

    use std::mem::transmute_copy;

    use molecule_core::IdToU32;
    use to_composite_id_macro::to_comp_id;

    use super::*;

    type Slot1 = SlotTS<to_comp_id!(1), P1, Z0, B1, P3, B0, B0>;
    type Slot2 = SlotTS<to_comp_id!(2), P2, P1, B0, P3, B0, B0>;
    type Slot3 = SlotTS<to_comp_id!(3), P3, P1, B0, P5, B0, B1>;
    type Op1Id = to_comp_id! {1};
    type Op2Id = to_comp_id! {2};
    type Op3Id = to_comp_id! {3};
    type Op1 = OperativeTS<Op1Id, (Slot1,)>;
    type Op2 = OperativeTS<Op2Id, (Slot2,)>;
    type Op3 = OperativeTS<Op3Id, (Slot3,)>;

    #[test]
    fn create_tracking_graph_type() {
        fn print_type_of<T>(_: &T)
        where
            T: GetCurrent,
            T: StateGetter,
            T::Output: IdToU32,
        {
            println!("{}", std::any::type_name::<T::State>());
            println!("---");
            println!(
                "UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU"
            );
            // println!("{}", T::Output::to_u32());
        }
        struct GraphTypestateContainer<Current, State>(PhantomData<(Current, State)>);
        impl<Current, State> GraphTypestateContainer<Current, State> {
            fn update_node<NodeId, NewNodeState>(
                self,
            ) -> GraphTypestateContainer<Current, <State as TSEdit<NodeId, NewNodeState>>::Result>
            where
                State: TSEdit<NodeId, NewNodeState>,
            {
                GraphTypestateContainer(PhantomData)
            }
            fn add_node<NewNodeState>(
                self,
            ) -> GraphTypestateContainer<AddUno<Current>, (OperativeTS<Current, NewNodeState>, State)>
            where
                Current: AddOneId<B0>,
                AddUno<Current>: AddOneId<B0>,
                NewNodeState: IdGetter,
            {
                GraphTypestateContainer(PhantomData)
            }
            fn remove_node<Id>(
                self,
            ) -> GraphTypestateContainer<Current, <State as TSRemove<Id>>::Result>
            where
                State: TSRemove<Id>,
                Id: IdIsEqual,
            {
                GraphTypestateContainer(PhantomData)
            }
        }
        impl<Current, State> StateGetter for GraphTypestateContainer<Current, State> {
            type State = State;
        }
        trait GetCurrent {
            type Output;
        }
        impl<Current, State> GetCurrent for GraphTypestateContainer<Current, State> {
            type Output = Current;
        }

        type TestId = CompId<U0, U0, U0, U1>;
        let inst = GraphTypestateContainer::<AddUno<TestId>, (Op1,)>(PhantomData);
        print_type_of(&inst);

        let inst = inst.add_node::<()>();
        print_type_of(&inst);
        let inst = inst.add_node::<()>();
        print_type_of(&inst);
        let inst = inst.update_node::<TestId, String>();
        print_type_of(&inst);
        let inst = inst.remove_node::<TestId>();
        print_type_of(&inst);
        let inst = inst.remove_node::<AddUno<TestId>>();
        print_type_of(&inst);
        let inst = inst.remove_node::<AddUno<AddUno<TestId>>>();
        print_type_of(&inst);
        panic!();
    }

    #[test]
    fn test_search() {
        assert_eq!(
            <<(Op1, Op2) as TSSearch<Op1Id>>::Result as IdGetter>::Id::to_u32(),
            1
        );
        assert_eq!(
            <<(Op1, (Op2,)) as TSSearch<Op2Id>>::Result as IdGetter>::Id::to_u32(),
            2
        );

        let result = <<() as TSSearch<to_comp_id!(42)>>::Result as IdGetter>::Id::to_i32();
        assert_eq!(result, -1);

        // Test searching in a single-element tuple
        assert_eq!(
            <<(Op1,) as TSSearch<Op1Id>>::Result as IdGetter>::Id::to_u32(),
            1
        );

        // Test searching for the final element in a tuple
        assert_eq!(
            <<(Op1, (Op2, (Op3,))) as TSSearch<Op3Id>>::Result as IdGetter>::Id::to_u32(),
            3
        );

        // Test searching for the middle element in a tuple
        assert_eq!(
            <<(Op1, (Op2, (Op3,))) as TSSearch<Op2Id>>::Result as IdGetter>::Id::to_u32(),
            2
        );
        // Test searching for the a bigger tuple
        assert_eq!(
            <<(
                Op1,
                (
                    Op2,
                    (
                        Op3,
                        (
                            OperativeTS<to_comp_id!(100), ()>,
                            OperativeTS<to_comp_id!(20), ()>
                        )
                    )
                )
            ) as TSSearch<to_comp_id!(100)>>::Result as IdGetter>::Id::to_u32(),
            100
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
        // Uncomment to verify compilation error

        // type SlotAtMax = SlotTS<P89, P3, Z0, B1, P3, B0, B0>;
        // assert_eq!(<SlotAtMax as SlotCanAddOne>::IMPLEMENTED, true);
    }

    #[test]
    fn test_slot_can_subtract_one() {
        // Can subtract one
        assert_eq!(<Slot2 as SlotCanSubtractOne>::IMPLEMENTED, true);

        // Cannot subtract one (at min)
        // Uncomment to verify compilation error

        // type SlotAtMin = SlotTS<P99, P1, P1, B0, P3, B0, B0>;
        // assert_eq!(<SlotAtMin as SlotCanSubtractOne>::IMPLEMENTED, true);
    }

    #[test]
    fn test_ts_add_operative() {
        type InitialState = (Op1, ());

        type NewState = <InitialState as TSAddToList<Op2>>::Result;
        assert_eq!(
            <<NewState as TSSearch<Op2Id>>::Result as IdGetter>::Id::to_u32(),
            2
        );

        type NewState2 = <NewState as TSAddToList<Op3>>::Result;
        assert_eq!(
            <<NewState2 as TSSearch<Op3Id>>::Result as IdGetter>::Id::to_u32(),
            3
        );

        // Adding the same operative again should not compile
        // Uncomment to verify compilation error
        // For some reason you have to use the type for the error to appear, so uncomment the print as well

        // type InvalidState = <NewState2 as TSAddToList<Op1>>::Result;
        // println!("{}", std::any::type_name::<InvalidState>());
    }

    #[test]
    fn test_ts_edit_operative() {
        type InitialState = (Op1, (Op2, (Op3,)));

        // Ensure the state was edited correctly for terminal item
        type FirstEditedState = <InitialState as TSEdit<Op3Id, (Slot1,)>>::Result;
        // println!("{}", std::any::type_name::<FirstEditedState>());
        // panic!();
        assert_eq!(
            <<<<FirstEditedState as TSSearch<Op3Id>>::Result as StateGetter>::State as TSSearch<
                to_comp_id!(1),
            >>::Result as IdGetter>::Id::to_u32(),
            1
        );
        assert_eq!(
            <<<<FirstEditedState as TSSearch<Op3Id>>::Result as StateGetter>::State as TSSearch<
                to_comp_id!(2),
            >>::Result as IdGetter>::Id::to_i32(),
            -1
        );

        // Ensure the state was edited correctly for mid-tuple item
        type EditedState = <InitialState as TSEdit<Op2Id, (Slot3,)>>::Result;
        // println!("{}", std::any::type_name::<FirstEditedState>());
        // panic!();
        assert_eq!(
            <<<<EditedState as TSSearch<Op2Id>>::Result as StateGetter>::State as TSSearch<
                to_comp_id!(3),
            >>::Result as IdGetter>::Id::to_u32(),
            3
        );
        assert_eq!(
            <<<<EditedState as TSSearch<Op2Id>>::Result as StateGetter>::State as TSSearch<
                to_comp_id!(2),
            >>::Result as IdGetter>::Id::to_i32(),
            -1
        );

        // Test that it works with a single item in a tuple
        type InitialState2 = (Op1,);
        type EditedState2 = <InitialState2 as TSEdit<Op1Id, (Slot3,)>>::Result;
        assert_eq!(
            <<<<EditedState2 as TSSearch<Op1Id>>::Result as StateGetter>::State as TSSearch<
                to_comp_id!(3),
            >>::Result as IdGetter>::Id::to_u32(),
            3
        );
    }
    #[test]
    fn test_ts_remove_operative() {
        type InitialState = (Op1, (Op2, (Op3,)));
        type EditedState = <InitialState as TSRemove<Op3Id>>::Result;
        assert_eq!(
            <<EditedState as TSSearch<Op3Id>>::Result as IdGetter>::Id::to_i32(),
            -1
        );
        type EditedState2 = <EditedState as TSRemove<Op2Id>>::Result;
        assert_eq!(
            <<EditedState2 as TSSearch<Op2Id>>::Result as IdGetter>::Id::to_i32(),
            -1
        );
        type EditedState3 = <EditedState2 as TSRemove<Op3Id>>::Result;
        assert_eq!(
            <<EditedState3 as TSSearch<Op3Id>>::Result as IdGetter>::Id::to_i32(),
            -1
        );
    }
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
///         let _result = <<(Op1, Op2) as Search<U3>>::Result as IdGetter>::Id::to_u32();
///    }
/// ```
fn test_failed_search() {}

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
