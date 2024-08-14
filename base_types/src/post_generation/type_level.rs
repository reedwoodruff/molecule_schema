use std::{
    marker::PhantomData,
    ops::{Add, BitAnd, BitOr, Sub},
};
use typenum::*;

// // Trait to represent the graph system's traits
// trait GraphTrait {}

// // Main builder struct
// struct Builder<Fields, Slots> {
//     _fields: PhantomData<Fields>,
//     _slots: PhantomData<Slots>,
// }

// // Slot types with min and max

// pub type Infinity = U1024;
pub type NonExistent = P9;

pub trait SlotTSMarker {
    // const FULFILLS_UPPER_BOUND: bool;
    // const FULFILLS_LOWER_BOUND: bool;
    type Count: Integer + IsGreaterOrEqual<Self::Min> + IsLessOrEqual<Self::Max> + IsGreater<Z0>;
    // + std::ops::Add<B1>
    // + std::ops::Sub<B1>
    // + typenum::Cmp<Self::Max>
    // + typenum::Cmp<Self::Min>
    // + typenum::Cmp<Z0>
    // + typenum::private::IsEqualPrivate<
    //     typenum::Z0,
    //     <Self::Count as typenum::Cmp<typenum::Z0>>::Output,
    // > + typenum::private::IsGreaterOrEqualPrivate<
    //     Self::Min,
    //     <Self::Count as typenum::Cmp<Self::Min>>::Output,
    // > + typenum::private::IsLessOrEqualPrivate<
    //     Self::Max,
    //     <Self::Count as typenum::Cmp<Self::Max>>::Output,
    // >;
    type Min: Integer + Cmp<NonExistent> + typenum::IsEqual<NonExistent>;
    type Max: Integer + Cmp<NonExistent> + typenum::IsEqual<NonExistent>;
    type ZeroAllowed: Bit + IsEqual<B0> + IsEqual<B1>;
    // type IsFulfilled: Bit;
    // type CanAddOne: Bit;
    // type CanSubtractOne: Bit;
    type MaxIsNonExistent: Bit;
    type MinIsNonExistent: Bit;
    type CountIsGreaterThanOrEqualToMin: Bit;
    type CountIsGreaterThanZero: Bit;
    type CountIsLessThanOrEqualToMax: Bit;
}
pub struct SlotTS<
    Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
    // + std::ops::Add<B1>
    // + std::ops::Sub<B1>
    // + typenum::Cmp<Max>
    // + typenum::Cmp<Min>
    // + typenum::Cmp<Z0>
    // + typenum::private::IsEqualPrivate<typenum::Z0, <Count as typenum::Cmp<typenum::Z0>>::Output>
    // + typenum::private::IsGreaterOrEqualPrivate<Min, <Count as typenum::Cmp<Min>>::Output>
    // + typenum::private::IsLessOrEqualPrivate<Max, <Count as typenum::Cmp<Max>>::Output>,
    Min: Integer + Cmp<NonExistent> + typenum::IsEqual<NonExistent>,
    Max: Integer + Cmp<NonExistent> + typenum::IsEqual<NonExistent> + typenum::Cmp<Count>,
    ZeroAllowed: Bit + IsEqual<B0> + IsEqual<B1>,
>(PhantomData<(Count, Min, Max, ZeroAllowed)>);
impl<
        Count: Integer + IsGreaterOrEqual<Min> + IsLessOrEqual<Max> + IsGreater<Z0>,
        // + std::ops::Add<B1>
        // + std::ops::Sub<B1>
        // + typenum::Cmp<Max>
        // + typenum::Cmp<Min>
        // + typenum::Cmp<Z0>
        // + typenum::private::IsEqualPrivate<
        //     typenum::Z0,
        //     <Count as typenum::Cmp<typenum::Z0>>::Output,
        // > + typenum::private::IsGreaterOrEqualPrivate<Min, <Count as typenum::Cmp<Min>>::Output>
        // + typenum::private::IsLessOrEqualPrivate<Max, <Count as typenum::Cmp<Max>>::Output>,
        Min: Integer + Cmp<NonExistent> + typenum::IsEqual<NonExistent>,
        Max: Integer + Cmp<NonExistent> + typenum::IsEqual<NonExistent> + typenum::Cmp<Count>,
        ZeroAllowed: Bit + IsEqual<B0> + IsEqual<B1>,
    > SlotTSMarker for SlotTS<Count, Min, Max, ZeroAllowed>
{
    type Count = Count;
    type Min = Min;
    type Max = Max;
    type ZeroAllowed = ZeroAllowed;
    // type IsFulfilled = op!(((Count <= Max) | (Max == NonExistent))
    //     & ((Count >= Min) | ((Count == Z0) & (ZeroAllowed == B1)) | (Min == NonExistent)));
    // type IsFulfilled = op!(((P1 <= P3) | (P3 == NonExistent))
    //     & ((P1 >= P1) | ((P1 == Z0) & (ZeroAllowed == B1)) | (P1 == NonExistent)));
    // const FULFILLS_UPPER_BOUND: bool = Count::I32 <= Max::I32 || Max::I32 == NonExistent::I32;
    // const FULFILLS_LOWER_BOUND: bool = Count::I32 >= Min::I32
    //     || (Count::I32 == 0 && ZeroAllowed::BOOL)
    //     || Min::I32 == NonExistent::I32;
    // type CountIsLessThanOrEqualToMax = <Count as IsLessOrEqual<Max>>::Output;
    type MaxIsNonExistent = Eq<Max, NonExistent>;
    type MinIsNonExistent = Eq<Max, NonExistent>;
    type CountIsGreaterThanOrEqualToMin = GrEq<Count, Min>;
    type CountIsLessThanOrEqualToMax = LeEq<Count, Max>;
    type CountIsGreaterThanZero = Gr<Count, Z0>;
    // type MaxIsNonExistent = op!(Max == NonExistent);
    // type IsFulfilled = <Self::CountIsLessThanOrEqualToMax as BitOr<Self::MaxIsNonExistent>>::Output; //Self::MaxIsNonExistent); // | (Max == NonExistent));
    // type IsFulfilled = Or<Self::CountIsLessThanOrEqualToMax, Self::MaxIsNonExistent>;
    // type CanAddOne = op!((Self::MaxIsNonExistent == B1) | (Count + B1 <= Max));
    // type CanSubtractOne =
    //     op!(Min == NonExistent | Count - B1 >= Min | (Count >= Z0 & ZeroAllowed == B1));

    // const FULFILLS_UPPER_BOUND: bool = Count::I32 <= Max::I32 || Max::I32 == NonExistent::I32;
    // const FULFILLS_LOWER_BOUND: bool = Count::I32 >= Min::I32
    //     || (Count::I32 == 0 && ZeroAllowed::BOOL)
    //     || Min::I32 == NonExistent::I32;
    // pub const IS_FULFILLED: bool = Self::FULFILLS_UPPER_BOUND && Self::FULFILLS_LOWER_BOUND;
    // pub const CAN_ADD_ONE: bool = if Max::I32 == NonExistent::I32 {
    //     true
    // } else {
    //     Count::I32 + 1 <= Max::I32
    // };
    // /// Allow subtraction if subtracting one would leave the count within the min or if zero is allowed and the count is greater than zero
    // pub const CAN_SUBTRACT_ONE: bool = if NonExistent::I32 == Min::I32 {
    //     if Count::I32 == 0 {
    //         false
    //     } else {
    //         true
    //     }
    // } else {
    //     Count::I32 - 1 >= Min::I32 || (Count::I32 > 0 && ZeroAllowed::BOOL)
    // };
}

// // Trait to modify a specific slot
// trait ModifySlot<Name, Op> {
//     type Output;
// }

// // Operation to increment a slot
// struct Increment;

// // Operation to decrement a slot
// struct Decrement;

// // Implementation for Builder
// impl<Fields, Slots> Builder<Fields, Slots> {
//     fn new() -> Self {
//         Builder {
//             _fields: PhantomData,
//             _slots: PhantomData,
//         }
//     }
// }

// // Implementation to modify a slot
// impl<Fields, Name, Count, Min, Max, Traits, RestSlots, Op>
//     Builder<Fields, (Slot<Name, Count, Min, Max, Traits>, RestSlots)>
// where
//     (Slot<Name, Count, Min, Max, Traits>, RestSlots): ModifySlot<Name, Op>,
// {
//     fn modify_slot(
//         self,
//     ) -> Builder<
//         Fields,
//         <(Slot<Name, Count, Min, Max, Traits>, RestSlots) as ModifySlot<Name, Op>>::Output,
//     > {
//         Builder {
//             _fields: PhantomData,
//             _slots: PhantomData,
//         }
//     }
// }

// // ModifySlot implementation for increment
// impl<Name, Count, Min, Max, Traits, RestSlots> ModifySlot<Name, Increment>
//     for (Slot<Name, Count, Min, Max, Traits>, RestSlots)
// where
//     Count: Add<B1>,
//     Max: Cmp<<Count as Add<B1>>::Output>,
//     <Max as Cmp<<Count as Add<B1>>::Output>>::Output: IsGreaterOrEqual<B1>,
// {
//     type Output = (
//         Slot<Name, <Count as Add<B1>>::Output, Min, Max, Traits>,
//         RestSlots,
//     );
// }

// // ModifySlot implementation for decrement
// impl<Name, Count, Min, Max, Traits, RestSlots> ModifySlot<Name, Decrement>
//     for (Slot<Name, Count, Min, Max, Traits>, RestSlots)
// where
//     Count: Sub<B1>,
//     <Count as Sub<B1>>::Output: Cmp<Min>,
//     <<Count as Sub<B1>>::Output as Cmp<Min>>::Output: IsGreaterOrEqual<B1>,
// {
//     type Output = (
//         Slot<Name, <Count as Sub<B1>>::Output, Min, Max, Traits>,
//         RestSlots,
//     );
// }

// // Recursive case for ModifySlot
// impl<Name, OtherName, OtherSlot, RestSlots, Op> ModifySlot<Name, Op> for (OtherSlot, RestSlots)
// where
//     RestSlots: ModifySlot<Name, Op>,
// {
//     type Output = (OtherSlot, <RestSlots as ModifySlot<Name, Op>>::Output);
// }

// // Trait to check if all slots are fulfilled
// trait AllSlotsFulfilled {}

// impl AllSlotsFulfilled for () {}

// impl<Name, Count, Min, Max, Traits, RestSlots> AllSlotsFulfilled
//     for (Slot<Name, Count, Min, Max, Traits>, RestSlots)
// where
//     Count: Cmp<Min>,
//     Max: Cmp<Count>,
//     <Count as Cmp<Min>>::Output: IsGreaterOrEqual<B1>,
//     <Max as Cmp<Count>>::Output: IsGreaterOrEqual<B1>,
//     RestSlots: AllSlotsFulfilled,
// {
// }

// // Build method
// impl<Fields, Slots> Builder<Fields, Slots>
// where
//     Slots: AllSlotsFulfilled,
// {
//     fn build(self) -> String {
//         "Built successfully".to_string()
//     }
// }

// // Example usage
// struct SlotA;
// struct SlotB;

// trait BuilderExt<Fields, Slots> {
//     fn add_to_slot_a(self) -> Self;
//     fn remove_from_slot_a(self) -> Self;
//     fn add_to_slot_b(self) -> Self;
//     fn remove_from_slot_b(self) -> Self;
// }

// impl<Fields, Slots> BuilderExt<Fields, Slots> for Builder<Fields, Slots>
// where
//     Slots: ModifySlot<SlotA, Increment>,
//     Slots: ModifySlot<SlotA, Decrement>,
//     Slots: ModifySlot<SlotB, Increment>,
//     Slots: ModifySlot<SlotB, Decrement>,
// {
//     fn add_to_slot_a(self) -> Builder<Fields, <Slots as ModifySlot<SlotA, Increment>>::Output> {
//         self.modify_slot::<SlotA, Increment>()
//     }

//     fn remove_from_slot_a(
//         self,
//     ) -> Builder<Fields, <Slots as ModifySlot<SlotA, Decrement>>::Output> {
//         self.modify_slot::<SlotA, Decrement>()
//     }

//     fn add_to_slot_b(self) -> Builder<Fields, <Slots as ModifySlot<SlotB, Increment>>::Output> {
//         self.modify_slot::<SlotB, Increment>()
//     }

//     fn remove_from_slot_b(
//         self,
//     ) -> Builder<Fields, <Slots as ModifySlot<SlotB, Decrement>>::Output> {
//         self.modify_slot::<SlotB, Decrement>()
//     }
// }

// fn main() {
//     let builder = Builder::<(), (Slot<SlotA, U0, U1, U3, ()>, Slot<SlotB, U0, U1, U2, ()>)>::new();
//     let builder = builder.add_to_slot_a().add_to_slot_b();
//     let builder = builder.add_to_slot_a().remove_from_slot_b();
//     // let builder = builder.add_to_slot_a(); // This would fail to compile if we try to add more than Max
//     // let builder = builder.remove_from_slot_b(); // This would fail to compile if we try to remove below Min
//     let result = builder.build();
//     println!("{}", result);
// }

// // Example traits
// struct Trait1;
// struct Trait2;
// impl GraphTrait for Trait1 {}
// impl GraphTrait for Trait2 {}
