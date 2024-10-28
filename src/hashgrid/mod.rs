use core::fmt;
use num_traits::{Float, FromPrimitive, One, PrimInt, ToPrimitive, Unsigned, Zero};
use std::{hash::Hash, ops::Div};

pub use grid::HashGrid;

mod grid;

pub struct HashIndex<T: PrimInt + FromPrimitive + ToPrimitive + Hash>(T);

impl<T> HashIndex<T>
where
    T: PrimInt + FromPrimitive + ToPrimitive + Hash,
{
    pub fn key(&self) -> T {
        self.0
    }
}

impl<U, T> From<U> for HashIndex<T>
where
    U: Unsigned + ToPrimitive + FromPrimitive,
    T: PrimInt + FromPrimitive + ToPrimitive + Hash,
{
    fn from(value: U) -> Self {
        HashIndex(T::from(value).unwrap())
    }
}

pub trait Entity {
    type ID: DataIndex;

    fn id(&self) -> Self::ID;
}

pub trait Coordinate {
    type Item: Float;

    fn x(&self) -> Self::Item;
    fn y(&self) -> Self::Item;
    fn z(&self) -> Self::Item {
        Zero::zero()
    }
}

pub trait Boundary {
    type Item: Float + FromPrimitive + ToPrimitive;

    fn centre(&self) -> [Self::Item; 3];
    fn size(&self) -> [Self::Item; 3];

    fn is_inside(&self, point: (Self::Item, Self::Item, Self::Item)) -> bool {
        let half_size = [
            self.size()[0].div(Self::Item::one() + Self::Item::one()),
            self.size()[1].div(Self::Item::one() + Self::Item::one()),
            self.size()[2].div(Self::Item::one() + Self::Item::one()),
        ];

        let dx = (point.0 - self.centre()[0]).abs();
        let dy = (point.1 - self.centre()[1]).abs();
        let dz = (point.2 - self.centre()[2]).abs();

        dx <= half_size[0] && dy <= half_size[1] && dz <= half_size[2]
    }

    fn max(&self) -> [Self::Item; 3] {
        let half_size = [
            self.size()[0].div(Self::Item::one() + Self::Item::one()),
            self.size()[1].div(Self::Item::one() + Self::Item::one()),
            self.size()[2].div(Self::Item::one() + Self::Item::one()),
        ];

        [
            self.centre()[0] + half_size[0],
            self.centre()[1] + half_size[1],
            self.centre()[2] + half_size[2],
        ]
    }

    fn min(&self) -> [Self::Item; 3] {
        let half_size = [
            self.size()[0].div(Self::Item::one() + Self::Item::one()),
            self.size()[1].div(Self::Item::one() + Self::Item::one()),
            self.size()[2].div(Self::Item::one() + Self::Item::one()),
        ];

        [
            self.centre()[0] - half_size[0],
            self.centre()[1] - half_size[1],
            self.centre()[2] - half_size[2],
        ]
    }
}

pub trait DataIndex: Copy + Default + Ord + fmt::Debug + 'static {}

macro_rules! impl_data_index(
    ( $( $t:ident ),* ) => {
        $(
            impl DataIndex for $t {}
        )*
    };
);

impl_data_index!(u8, u16, u32, u64, usize);