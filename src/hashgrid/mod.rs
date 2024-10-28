use core::fmt;
use grid::DataRef;
use num_traits::{Float, FromPrimitive, One, PrimInt, ToPrimitive, Unsigned, Zero};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::Div,
};

pub use grid::HashGrid;

mod grid;

#[derive(Debug, Clone, Copy)]
pub enum QueryType<Id> {
    Find(Id),
    Relevant,
}

impl<Id: Display> fmt::Display for QueryType<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryType::Find(id) => write!(f, "Find({})", id),
            QueryType::Relevant => write!(f, "Relevant"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Query<F, Id> {
    pub radius: F,
    pub ty: QueryType<Id>,
    pub coordinates: (F, F, F),
}

impl<F, Id> fmt::Display for Query<F, Id>
where
    F: Float + FromPrimitive + ToPrimitive + Display,
    Id: DataIndex + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Query [\n  Type: {}\n  Radius: {}\n  Coordinates: (x= {}, y= {}, z= {})\n]",
            self.ty, self.radius, self.coordinates.0, self.coordinates.1, self.coordinates.2,
        )
    }
}

impl<F, Id> Query<F, Id>
where
    F: Float + FromPrimitive + ToPrimitive,
    Id: DataIndex,
{
    pub fn from(cords: (F, F, F), query_type: QueryType<Id>, radius: F) -> Self {
        Self {
            radius,
            ty: query_type,
            coordinates: cords,
        }
    }

    pub fn x(&self) -> F {
        self.coordinates.0
    }

    pub fn y(&self) -> F {
        self.coordinates.1
    }

    pub fn z(&self) -> F {
        self.coordinates.2
    }

    pub fn radius(&self) -> F {
        self.radius
    }
    pub fn query_type(&self) -> QueryType<Id> {
        self.ty
    }
}

#[derive(Debug)]
pub struct QueryResult<'a, F, Id, T> {
    query: Query<F, Id>,
    data: Vec<DataRef<'a, T>>,
}

impl<'a, F, Id, T> QueryResult<'a, F, Id, T>
where
    F: Float + FromPrimitive + ToPrimitive,
    Id: DataIndex,
{
    pub fn query(&self) -> Query<F, Id> {
        self.query
    }

    pub fn data(&self) -> &[DataRef<'a, T>] {
        &self.data
    }
}

impl<'a, F, Id, T> fmt::Display for QueryResult<'a, F, Id, T>
where
    F: Float + FromPrimitive + ToPrimitive + Display,
    Id: DataIndex + Display,
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "QueryResult [\n  {}\n  Data: {:?}\n]",
            self.query(),
            self.data()
        )
    }
}

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
