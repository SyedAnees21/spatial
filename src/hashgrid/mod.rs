use core::fmt;
use grid::{DataRef, DefaultDx};
use num_traits::{Float, FromPrimitive, PrimInt, ToPrimitive, Unsigned, Zero};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

pub use grid::HashGrid;

mod grid;

type DefaultFloat = f32;

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

impl<F: Float + FromPrimitive + ToPrimitive> Boundary<F> for GridBoundary<F> {
    type T = [F; 3];

    fn centre(&self) -> [F; 3] {
        self.center
    }

    fn size(&self) -> [F; 3] {
        self.size
    }
}

impl<F: Float + FromPrimitive + ToPrimitive> Coordinate<F> for [F; 3] {
    fn new(x: F, y: F, z: F) -> Self {
        [x, y, z]
    }

    fn x(&self) -> F {
        self[0]
    }

    fn y(&self) -> F {
        self[1]
    }

    fn z(&self) -> F {
        self[2]
    }
}

// impl<F, T> From<T> for [F;3]
// where
//     F: Float + FromPrimitive + ToPrimitive,
//     T: Coordinate<Component = F>,
// {
//     fn from(value: T) -> Self {
//         [value.x(), value.y(), value.z()]
//     }
// }

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
/// # #[derive(Debug, Copy, Clone, PartialEq)]
/// # struct Point2 {
/// #     x: f32,
/// #     y: f32,
/// #     z: f32,
/// # }
/// # impl Coordinate for Point2 {
/// #     fn new(x: f32, y: f32, _: f32) -> Self {
/// #         Self {
/// #             x,
/// #             y,
/// #             z: 0.0,
/// #         }
/// #     }
/// #     fn x(&self) -> f32 {
/// #       self.x
/// #     }
/// #     fn y(&self) -> f32 {
/// #       self.y
/// #     }
/// # }
/// # struct Bounds {
/// #     center: Point2,
/// #     size: Point2,
/// # }
/// #
/// # impl Boundary for Bounds {
/// #     type T = Point2;
/// #     
/// #     fn centre(&self) -> Self::T {
/// #         self.center
/// #     }
/// #     
/// #     fn size(&self) -> Self::T {
/// #         self.size
/// #     }
/// # }
/// # #[derive(Debug, PartialEq)]
/// # struct Object2D {
/// #     id: u32,
/// #     position: Point2
/// # }
/// # impl Entity for Object2D {
/// #     type Position = Point2;
/// #     
/// #     fn id(&self) -> u32 {
/// #         self.id
/// #     }
/// #     
/// #     fn position(&self) -> Self::Position {
/// #         self.position
/// #     }
/// # }
///
/// // Assuming that the bounds implements HashGrid::Boundary trait
/// let bounds = Bounds {
///     center: Point2::new(0.0, 0.0, 0.0),
///     size: Point2::new(100.0, 100.0, 0.0)
/// };
///
/// // Creating the Hashgrid with f32 as the base float and object as the base data type
/// // Object type must implements the HashGrid::{Entity, Coordinate} traits
/// let mut hashgrid = HashGrid::<f32, Object2D>::new([2,2], 0, &bounds, false);
///
/// // Creating two objects at different locations
/// let obj1 = Object2D {
///     id: 0,
///     position: Point2::new(22.0, 30.0, 0.0)
/// };
///
/// let obj2 = Object2D {
///     id: 0,
///     position: Point2::new(15.0, 45.0, 0.0)
/// };
///
/// // Inserting the objects into the hashgrid
/// hashgrid.insert(&obj1);
/// hashgrid.insert(&obj2);
///
/// // Creating a query to query the hash grid for relevant data within a single cell
/// // this is defined by the radius = 0, at some location define by the query coordinates
/// let query = Query::from(Point2::new(25., 25., 0.), QueryType::Relevant, 0.0);
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
pub struct Query<F, Id, C> {
    pub radius: F,
    pub ty: QueryType<Id>,
    pub coordinates: C,
}

impl<F, Id, C> fmt::Display for Query<F, Id, C>
where
    F: Float + FromPrimitive + ToPrimitive + Display + Copy,
    Id: DataIndex + Display,
    C: Coordinate<F>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Query [\n  Type: {}\n  Radius: {}\n  Coordinates: (x= {}, y= {}, z= {})\n  ]",
            self.ty,
            self.radius,
            self.coordinates.x(),
            self.coordinates.y(),
            self.coordinates.z(),
        )
    }
}

impl<F, Id, C> Query<F, Id, C>
where
    F: Float + FromPrimitive + ToPrimitive,
    Id: DataIndex,
    C: Coordinate<F>,
{
    pub fn from(cords: C, query_type: QueryType<Id>, radius: F) -> Self {
        Self {
            radius,
            ty: query_type,
            coordinates: cords,
        }
    }

    pub fn x(&self) -> F {
        self.coordinates.x()
    }

    pub fn y(&self) -> F {
        self.coordinates.y()
    }

    pub fn z(&self) -> F {
        self.coordinates.z()
    }

    pub fn radius(&self) -> F {
        self.radius
    }
    pub fn query_type(&self) -> QueryType<Id> {
        self.ty
    }
}

/// QueryResult is the return type for [`Query`]. When we query the hashgrid, hashgrid returns
/// a response in `QueryResult`.
///
/// It contains the original query made to hashgrid, and the list of immutable references to the data
/// collected as the response. To access the data isnside the QueryResult, use method [`QueryResult::data`]
/// and to see the original query use [`QueryResult::query`]
#[derive(Debug)]
pub struct QueryResult<'a, F, Id, T, C> {
    query: Query<F, Id, C>,
    data: Vec<DataRef<'a, T>>,
}

impl<'a, F, Id, T, C> QueryResult<'a, F, Id, T, C>
where
    F: Float + FromPrimitive + ToPrimitive,
    Id: DataIndex,
    C: Coordinate<F>,
{
    pub fn query(&self) -> &Query<F, Id, C> {
        &self.query
    }

    pub fn data(&self) -> &[DataRef<'a, T>] {
        &self.data
    }
}

impl<'a, F, Id, T, C> fmt::Display for QueryResult<'a, F, Id, T, C>
where
    F: Float + FromPrimitive + ToPrimitive + Display,
    Id: DataIndex + Display,
    T: Debug,
    C: Coordinate<F>,
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

/// Type used as unique cell indices or the cell hash for identifying the grid cell
/// to insert or retreive the data.
///
/// `HashIndex` is generic over the type to be used as hash index and is passed through
/// the `HashGrid` initialization. If there is no type passed for the hashindex, then
/// it defaults to the type `u64`
///
/// # Example
///
/// This is how we can pass the hashindex type at the time of [`HashGrid`] initialization
///
/// ```rust
/// # use spatial::hashgrid::{HashGrid, Boundary, Coordinate};
/// # #[derive(Copy, Clone)]
/// # struct Point3 {
/// #     x: f32,
/// #     y: f32,
/// #     z: f32,
/// # }
/// # impl Coordinate for Point3 {
/// #     fn new(x: f32, y: f32, z: f32) -> Self {
/// #         Self {
/// #             x,
/// #             y,
/// #             z,
/// #         }
/// #     }
/// #     fn x(&self) -> f32 {
/// #       self.x
/// #     }
/// #     fn y(&self) -> f32 {
/// #       self.y
/// #     }
/// #     fn z(&self) -> f32 {
/// #       self.z
/// #     }
/// # }
/// # struct Bounds {
/// #     center: Point3,
/// #     size: Point3,
/// # }
/// #
/// # impl Boundary for Bounds {
/// #     type T = Point3;
/// #     
/// #     fn centre(&self) -> Point3 {
/// #         self.center
/// #     }
/// #     
/// #     fn size(&self) -> Point3 {
/// #         self.size
/// #     }
/// # }
/// # let boundary = Bounds {
/// #     center: Point3::new(0.0, 0.0, 0.0),
/// #     size: Point3::new(100.0, 100.0, 100.0)
/// # };
/// // Here we are initializing the HashGrid with `f32` as bas float type
/// // and passing no type for the data to the hashgrid and the `u32` as the
/// // HashIndex type
/// let hashgrid = HashGrid::<f32, (), u32>::new([2,2], 2, &boundary, false);
/// ```
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

/// `Entity` trait obligates the data object to have a unique id
///
/// This is a trait bound imposed by the hashgrid to must implement for data type for which
/// the hashgrid is being created.
pub trait Entity<D = DefaultDx, F = DefaultFloat>
where
    F: Float + FromPrimitive + ToPrimitive,
    D: DataIndex,
{
    type Position: Coordinate<F>;

    /// Mendatory method to return the unique ID value of the data type
    fn id(&self) -> D;

    fn position(&self) -> Self::Position;
}

/// `Coordinate` trait obligates the data object to have spatial coordinates components. This
/// trait can be implemented on the 2D object types as well.
///
/// This is a trait bound imposed by the hashgrid to must implement for data type for which
/// the hashgrid is being created.
pub trait Coordinate<F = DefaultFloat>: Clone + Copy
where
    F: Float + FromPrimitive + ToPrimitive,
{
    fn new(x: F, y: F, z: F) -> Self;

    /// Mendatory method to return the x coordinate value of the data type
    fn x(&self) -> F;

    /// Mendatory method to return the y coordinate value of the data type
    fn y(&self) -> F;

    // Optional method to return the z coordinate value of the data type if
    /// the type is 3D
    fn z(&self) -> F {
        Zero::zero()
    }

    fn xyz(&self) -> (F, F, F) {
        (self.x(), self.y(), self.z())
    }
}

pub trait Boundary<F = DefaultFloat>
where
    F: Float + FromPrimitive + ToPrimitive,
{
    type T: Coordinate<F>;

    fn centre(&self) -> Self::T;
    fn size(&self) -> Self::T;

    fn is_inside<U>(&self, point: U) -> bool
    where
        U: Coordinate<F>,
    {
        let half_size = [
            self.size().x().div(F::from_f32(2.0).unwrap()),
            self.size().y().div(F::from_f32(2.0).unwrap()),
            self.size().z().div(F::from_f32(2.0).unwrap()),
        ];

        let dx = point.x() - self.centre().x().abs();
        let dy = point.y() - self.centre().y().abs();
        let dz = point.z() - self.centre().z().abs();

        dx <= half_size[0] && dy <= half_size[1] && dz <= half_size[2]
    }

    fn boundary_max(&self) -> Self::T {
        let half_size = [
            self.size().x().div(F::from_f32(2.0).unwrap()),
            self.size().y().div(F::from_f32(2.0).unwrap()),
            self.size().z().div(F::from_f32(2.0).unwrap()),
        ];

        Self::T::new(
            self.centre().x() + half_size[0],
            self.centre().y() + half_size[1],
            self.centre().z() + half_size[2],
        )
    }

    fn boundary_min(&self) -> Self::T {
        let half_size = [
            self.size().x().div(F::from_f32(2.0).unwrap()),
            self.size().y().div(F::from_f32(2.0).unwrap()),
            self.size().z().div(F::from_f32(2.0).unwrap()),
        ];

        Self::T::new(
            self.centre().x() - half_size[0],
            self.centre().y() - half_size[1],
            self.centre().z() - half_size[2],
        )
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
