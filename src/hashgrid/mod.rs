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

/// ### Cells per Axis
///
/// Stores the total number of cells on each axis defined by the user at the time of
/// grid construction
#[derive(Debug)]
pub struct CellsPerAxis {
    xcells: u32,
    ycells: u32,
    floors: usize,
}

impl CellsPerAxis {
    pub fn from(cells: &[u32], floors: usize) -> Self {
        assert!(
            cells.len() == 2,
            "Invalid components, expected 2 components for cells per axis"
        );
        Self {
            xcells: cells[0],
            ycells: cells[1],
            floors,
        }
    }
}

/// ### Cell Sizes
///
/// Holds the cell size for every individual cell on each axis, These sizes are calculated
/// during the grid initialization depending upon the [`CellsPerAxis`] and grid bounds
#[derive(Debug)]
pub struct CellSizes<F> {
    x_size: F,
    y_size: F,
    floor_size: F,
}

/// ### Grid Boundary
///
/// Defines the dimensions of bounding `rectangle(2D)`/`box(3D)` for the grid, which acts as
/// the grid boundary. This type implements the [`Boundary`] trait
#[derive(Debug)]
pub struct GridBoundary<F> {
    pub center: [F; 3],
    pub size: [F; 3],
}

impl<F: Float + FromPrimitive + ToPrimitive> Boundary for GridBoundary<F> {
    type Item = F;

    fn centre(&self) -> [Self::Item; 3] {
        self.center
    }

    fn size(&self) -> [Self::Item; 3] {
        self.size
    }
}

/// Stores the grid information regarding the cell sizes and number of cells per axis
#[derive(Debug)]
pub struct GridParameters<F> {
    pub cell_per_axis: CellsPerAxis,
    pub cell_sizes: CellSizes<F>,
}

/// This enum defines the type of query to be made to the [`HashGrid`]. It specifies
/// the following two basic queries:
///
/// * `Find:` It tells the hashgrid to find the data which matches the `Id` parameter in the find query
/// * `Relevant:` Asks the hashgrid to return all th relevant data around the query point of interest within a radius
///
/// `QueryType` is one of the major constituent of the main [`Query`]
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

/// Used for querying the [`HashGrid`].
///
/// Query provides the parameters to inquire the hashgrid flexibly. It parameterized over:
///
/// * `F (base float type):`Float type which is being used in the hashgrid
/// * `Id (base integer type)`: Integer type use for pattern matching to find the data in the hashgrid
///
/// `Id` type implements [`DataIndex`] trait, moreover `F` and `Id` are infered from the grid generics at
/// the time of hashgrid initialization.
///
/// # Examples
///
/// Here is how we can use the `Query` to query the hashgrid:
///
/// ```rust
/// use spatial::hashgrid::{HashGrid, Boundary, Coordinate, Entity, Query, QueryType};
/// # struct Bounds {
/// #     center: (f32,f32,f32),
/// #     size: (f32,f32,f32),
/// # }
/// #
/// # impl Boundary for Bounds {
/// #     type Item = f32;
/// #     
/// #     fn centre(&self) -> [Self::Item; 3] {
/// #         [self.center.0, self.center.1, self.center.2]
/// #     }
/// #     
/// #     fn size(&self) -> [Self::Item; 3] {
/// #         [self.size.0, self.size.1, self.size.2]
/// #     }
/// # }
/// # #[derive(Debug, PartialEq)]
/// # struct Object {
/// #     id: u32,
/// #     position: (f32,f32),
/// # }
/// #
/// # impl Entity for Object {
/// #     type ID = u32;
/// #     fn id(&self) -> Self::ID {
/// #         self.id
/// #     }
/// # }
/// #     
/// #     impl Coordinate for Object {
/// #     type Item = f32;
/// #     fn x(&self) -> Self::Item {
/// #         self.position.0
/// #     }
/// #
/// #     fn y(&self) -> Self::Item {
/// #         self.position.1
/// #     }
/// # }
///
/// // Assuming that the bounds implements HashGrid::Boundary trait
/// let bounds = Bounds {
///     center: (0.0, 0.0, 0.0),
///     size: (100.0, 100.0, 100.0)
/// };
///
/// // Creating the Hashgrid with f32 as the base float and object as the base data type
/// // Object type must implements the HashGrid::{Entity, Coordinate} traits
/// let mut hashgrid = HashGrid::<f32, Object>::new([2,2], 0, &bounds, false);
///
/// // Creating two objects at different locations
/// let obj1 = Object {
///     id: 0,
///     position: (22.0, 30.0)
/// };
///
/// let obj2 = Object {
///     id: 0,
///     position: (15.0, 45.0)
/// };
///
/// // Inserting the objects into the hashgrid
/// hashgrid.insert(&obj1);
/// hashgrid.insert(&obj2);
///
/// // Creating a query to query the hash grid for relevant data within a single cell
/// // this is defined by the radius = 0, at some location define by the query coordinates
/// let query = Query::from((25., 25., 0.), QueryType::Relevant, 0.0);
///
/// // Now querying the hashgrid
/// let res = hashgrid.query(query);
///
/// // We must get the two objects in the query response from the grid
/// assert_eq!(res.data(), &[&obj1, &obj2])
/// ```
///
/// Querying the hashgrid returns the [`QueryResult`] as response.
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

// pub type DefaultDx = usize;

// pub struct Data<'a, T, Dx = DefaultDx> {
//     pub index: Dx,
//     pub refer: DataRef<'a, T>
// }

pub trait DataIndex: Copy + Default + Ord + fmt::Debug + 'static {}

macro_rules! impl_data_index(
    ( $( $t:ident ),* ) => {
        $(
            impl DataIndex for $t {}
        )*
    };
);

impl_data_index!(u8, u16, u32, u64, usize);
