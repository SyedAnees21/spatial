use core::fmt;
use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
};

use num_traits::{Float, FromPrimitive, One, PrimInt, ToPrimitive};

use super::{
    Boundary, CellSizes, CellsPerAxis, Coordinate, DataIndex, Entity, GridBoundary, GridParameters,
    HashIndex, Query, QueryResult, QueryType,
};

/// Grid is an alias for HashMaps
///
/// Its a wrapper around the core HashMap type and inherets all the functionalities of a HashMap
pub type Grid<K, V> = HashMap<K, V>;

/// Floors is an alias for vec type
///
/// Its a wrapper around the vec type and stores the list of the grids defined by the total number of floors
pub type Floors<T> = Vec<T>;

/// DataRef type defines the generic type parameter for the [`HashGrid`]
///
/// DataRef is actually the immutable reference to the data which is stored and managed in grid and must live
/// as long as the grid lives
pub type DataRef<'a, T> = &'a T;

/// Type alias for default type used by the Hashgrid for hash index
pub type DefaultHx = u64;
pub type DefaultDx = u32;

/// # HashGrid
///
/// A 3D/2D spatial partitioning algorithm to manage the data quickly and efficiently according to the data's spatial
/// characterstics.
///
/// HashGrid is parameterized over:
///
/// * `F (Float type):` Defines the base float type such as `f32` or `f64` for spatial components (x , y, z) and calculations
/// * `T (generic data type):` Defines the data type to insert into the grid, data mus live as long as the grid lives`
/// * `Hx (HashIndex type):` Defines the type to be used for hashes for data search in grid, default type for `Hx` is `u64`
///
#[derive(Debug)]
pub struct HashGrid<'a, F, T, Hx = DefaultHx, Dx = DefaultDx>
where 
    F: Float + FromPrimitive + ToPrimitive,
{
    pub grids: Floors<Grid<Hx, Vec<DataRef<'a, T>>>>,
    pub params: GridParameters<F>,
    pub bounds: GridBoundary<F>,
    pub wrap: bool,
    pub phantom: PhantomData<Dx>,
}

impl<'a, F, T, Hx, Dx> HashGrid<'a, F, T, Hx, Dx>
where
    F: Float + FromPrimitive + ToPrimitive,
    Hx: PrimInt + FromPrimitive + ToPrimitive + Hash,
    Dx: DataIndex,
{
    /// Creates a new instance of [`HashGrid`] according to the number of cells and the bounds
    /// defined as the parameters.
    ///
    /// Parameters to this function specifies:
    ///
    /// - `Cells:` Uniform number of cells in x and y directions for each grid
    /// - `floors:` Number of grids to be initialized vertically in z direction, if the grid is meant to be used as 2D, then set the floors as 0
    /// - `Bounds:` Defining the bounds for each grid, this parameter takes the bounds as any type which implements [`Boundary`] trait
    ///
    /// This is a constructor method which returns the HashGrid lazily initialized without any data, later on you can use the [`HashGrid::update`]
    /// or [`HashGrid::insert`] methods to insert the data into the grid according the individual coordinates of the data.
    pub fn new<B>(cells: [u32; 2], floors: usize, bounds: &B, wrap: bool) -> Self
    where
        B: Boundary<F>,
    {
        // Identifying the max number of floors to initialize
        // the grids at each floor
        let floors = floors.max(One::one());

        // Converting the number of cells for each axis to the
        // base float type for the system
        let x_cells_f = F::from(cells[0]).unwrap();
        let y_cells_f = F::from(cells[1]).unwrap();
        let z_floors_f = F::from(floors).unwrap();

        // Getting the maximum length on each axis of the grid boundary
        let length_on_x = bounds.size().x();
        let length_on_y = bounds.size().y();
        let length_on_z = bounds.size().z();

        // Calculating the cell sizes for each axis to initialize the
        // grid uniformly
        let x_size = length_on_x / x_cells_f;
        let y_size = length_on_y / y_cells_f;

        // Calculating the floor height for each floor if the grid is being
        // initialized as 3D
        let floor_size = (length_on_z / z_floors_f).max(One::one());

        // Constructing the grid parameters
        let params = GridParameters {
            cell_per_axis: CellsPerAxis::from(&cells, floors),
            cell_sizes: CellSizes {
                x_size,
                y_size,
                floor_size,
            },
        };

        // Storing the bounds as the grid boundary parameters to later
        // use them for boundary limitations
        let bounds = GridBoundary {
            center: [
                bounds.centre().x(),
                bounds.centre().y(),
                bounds.centre().z(),
            ],
            size: [bounds.size().x(), bounds.size().y(), bounds.size().z()],
        };

        Self {
            grids: vec![Grid::new(); floors],
            params,
            bounds,
            wrap,
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, entity: DataRef<'a, T>)
    where
        T: Entity<Dx, F>,
    {
        // Getting the grid's extreme boundary parameters to apply the boundary
        // limits to the calculated cell cords if necessary
        let grid_max_bounds = self.bounds.boundary_max();
        let grid_min_bounds = self.bounds.boundary_min();

        let mut coodrinates = entity.position().xyz();

        // Validating if the point is within the grid bounds
        if !self.bounds.is_inside(entity.position()) {
            // Wraps around the nearest cell to the grid if the point is outside and wrap
            // is enabled
            if self.wrap {
                coodrinates.0 = coodrinates
                    .0
                    .min(grid_max_bounds[0])
                    .max(grid_min_bounds[0]);
                coodrinates.1 = coodrinates
                    .1
                    .min(grid_max_bounds[1])
                    .max(grid_min_bounds[1]);
                coodrinates.2 = coodrinates
                    .2
                    .min(grid_max_bounds[2])
                    .max(grid_min_bounds[2]);
            } else {
                // Return without inserting the data if the wrap is disabled and the point is
                // not withing the bounds
                return;
            }
        }

        // Resulting cell coordinates x, y and floor index
        let (cx, cy, floor) =
            self.get_cell_coordinates(coodrinates.0, coodrinates.1, coodrinates.2);

        // Calculating the unique hash index from the cell coordinates to find the cell
        // for the entity
        let hashindex = self.key(cx, cy);

        // Inserting the the entity in to the identified cell of the grid at
        // the identified floor
        match self.grids[floor].entry(hashindex.key()) {
            Occupied(mut entry) => {
                // If the cell is already existing with some data,
                // then we just update the cell with the current entity data
                let grid_cell = entry.get_mut();
                grid_cell.push(entity);
            }
            Vacant(entry) => {
                // If the cell is not present already, we inserts the new cell
                // with having the current entity data inside
                entry.insert(vec![entity]);
            }
        }
    }

    pub fn query<Id, C>(&self, query: Query<F, Id, C>) -> QueryResult<'a, F, Id, T, C>
    where
        Id: DataIndex,
        T: Entity<Id, F>,
        C: Coordinate<F>,
    {
        // let radius_x = (F::from_u32(self.xcells()).unwrap() * query.radius())
        //     .max(F::one())
        //     .ceil()
        //     .to_i32()
        //     .unwrap();
        // let radius_y = (F::from_u32(self.ycells()).unwrap() * query.radius())
        //     .max(F::one())
        //     .ceil()
        //     .to_i32()
        //     .unwrap();
        // let radius_f = (F::from_usize(self.floors()).unwrap() * query.radius())
        //     .max(F::one())
        //     .ceil()
        //     .to_i32()
        //     .unwrap();

        // let (cx, cy, floor) = self.get_cell_coordinates(query.x(), query.y(), query.z());

        // let base_cx = cx as i32;
        // let base_cy = cy as i32;
        // let base_floor = floor as i32;

        // let range_x = (base_cx - radius_x).max(0)..=(base_cx + radius_x).min(self.xcells() as i32);
        // let range_y = (base_cy - radius_y).max(0)..=(base_cy + radius_y).min(self.ycells() as i32);
        // let range_z =
        //     (base_floor - radius_f).max(0)..=(base_floor + radius_f).min(self.floors() as i32 - 1);

        // let relevant_indices = range_x
        //     .clone()
        //     .flat_map(|dx| {
        //         let range_z = range_z.clone();
        //         range_y.clone().flat_map(move |dy| {
        //             range_z
        //                 .clone()
        //                 .map(move |dz| (dx as u32, dy as u32, dz as usize))
        //         })
        //     })
        //     .map(|(dx, dy, df)| (self.key(dx, dy), df));

        let mut result = QueryResult {
            query,
            cells: Vec::new(),
            data: Vec::new(),
        };

        match query.query_type() {
            QueryType::Single => {
                let (x, y, z) = query.coordinates.xyz();
                let (x_index, y_index, floor) = self.get_cell_coordinates(x, y, z);
                let hashindex = self.key(x_index, y_index);

                if let Some(data) = self.grids[floor].get(&hashindex.key()) {
                    result.cells.push(hashindex.key().to_usize().unwrap());
                    result.data.extend_from_slice(data);
                }
            },
            QueryType::Search(id) => {
                // for (hashindex, floor) in relevant_indices {
                //     if let Some(d_list) = self.grids[floor].get(&hashindex.key()) {
                //         if let Some(&entity) = d_list.iter().find(|&&d| d.id() == id) {
                //             result.cells.push(hashindex.key().to_usize().unwrap());
                //             result.data.push(entity);
                //             break;
                //         }
                //     }
                // }
            },
            QueryType::Neighbour(radius) => {
                // for (hashindex, floor) in relevant_indices {
                //     if let Some(d_list) = self.grids[floor].get(&hashindex.key()) {
                //         result.cells.push(hashindex.key().to_usize().unwrap());
                //         result.data.extend_from_slice(d_list);
                //     }
                // }
            },
        }

        result
    }

    /// Inserts the references to individual data from the list of data into the relevant cells of the grid by finding
    /// unique [`HashIndex`] through cell coordinates. These cell coordinates are based on the
    /// data of type [`Entity`] individual spatial coordinates.
    ///
    /// This methods takes the reference list of data of any type which must implements the following traits:
    ///
    /// * [`Entity`]: This trait imposes the data to have a unique id which can be used to find an entity in the grid
    /// * [`Coordinate`]: This trait imposes the type to return the spatial coordinates `x, y, z`.
    ///
    /// Every `entity` or data of type `Entity` is then inserted into the belonging cell using
    /// the unique `HashIndex`.
    pub fn update(&mut self, data: &'a [T])
    where
        T: Entity<Dx, F>,
    {
        // Getting the grid's extreme boundary parameters to apply the boundary
        // limits to the calculated cell cords if necessary
        let grid_max_bounds = self.bounds.boundary_max();
        let grid_min_bounds = self.bounds.boundary_min();

        for entity in data.iter() {
            // Getting the cell coordinates from entity coordinates
            // z-axis from the entity coordinates defines at which floor of the grid
            // to look for the cell
            let mut coodrinates = entity.position().xyz();

            // Wrapping around the nearest grid bounds if the wrap is enabled and the
            // entity is outside the grid bounds or else do not add the entity inside the grid
            if !self.bounds.is_inside(entity.position()) {
                if self.wrap {
                    coodrinates.0 = coodrinates
                        .0
                        .min(grid_max_bounds[0])
                        .max(grid_min_bounds[0]);
                    coodrinates.1 = coodrinates
                        .1
                        .min(grid_max_bounds[1])
                        .max(grid_min_bounds[1]);
                    coodrinates.2 = coodrinates
                        .2
                        .min(grid_max_bounds[2])
                        .max(grid_min_bounds[2]);
                } else {
                    continue;
                }
            }

            // Resulting cell coordinates x, y and floor index
            let (cx, cy, floor) =
                self.get_cell_coordinates(coodrinates.0, coodrinates.1, coodrinates.2);

            // Calculating the unique hash index from the cell coordinates to find the cell
            // for the entity
            let hashindex = self.key(cx, cy);

            // Inserting the the entity in to the identified cell of the grid at
            // the identified floor
            match self.grids[floor].entry(hashindex.key()) {
                Occupied(mut entry) => {
                    // If the cell is already existing with some data,
                    // then we just update the cell with the current entity data
                    let grid_cell = entry.get_mut();
                    grid_cell.push(entity);
                }
                Vacant(entry) => {
                    // If the cell is not present already, we inserts the new cell
                    // with having the current entity data inside
                    entry.insert(vec![entity]);
                }
            }
        }
    }

    /// Calculates the cells coordinates from the entity coordinates to find the cell
    /// location inside the grid.
    ///
    /// Reutrns the `Floor` number, `x` and `y` components of the cell in search.
    pub fn get_cell_coordinates(&self, x: F, y: F, z: F) -> (u32, u32, usize) {
        // Normalizing the x and y component according to cell size to find the
        // cell coordinates inside the grid
        let cx = ((x + self.bounds.boundary_max().x()) / self.cell_size_x()).floor().to_u32().unwrap();
        let cy = ((y + self.bounds.boundary_max().y()) / self.cell_size_y()).floor().to_u32().unwrap();

        // Getting the floor index from the z component
        let floor = (z / self.floor_size()).floor().to_usize().unwrap();

        (cx, cy, floor)
    }

    /// Calculates the unique hash of a specefic cell in the [`HashGrid`] to retreive or
    /// insert the entity or data of type [`Entity`]. It calculate the unique hash id through
    /// cantor pairing formula which uses the cell coordinates x and y as the `k1` and `k2`
    /// components and z componenet to determine on which `floor` to look for the data.
    ///
    /// __Cantor pairing formula__:
    ///
    /// `((k1 + k2) * (k1 + k2 + 1)) / 2 + k2`
    ///
    /// Reutrns the unique cantor number calculate from the cell coordinates as [`HashIndex`]
    pub fn key(&self, k1: u32, k2: u32) -> HashIndex<Hx> {
        ((((k1 + k2) * (k1 + k2 + 1)) / 2) + k2).into()
    }

    /// Cell size defined for cells on x-axis
    ///
    /// Returns the size in base `float` type being used in the [`HashGrid`]
    pub fn cell_size_x(&self) -> F {
        self.params.cell_sizes.x_size
    }

    /// Cell size defined for cells on y-axis
    ///
    /// Returns the size in base `float` type being used in the [`HashGrid`]
    pub fn cell_size_y(&self) -> F {
        self.params.cell_sizes.y_size
    }

    /// Floor height defined for floors on z-axis
    ///
    /// Returns the size in base `float` type being used in the [`HashGrid`]
    pub fn floor_size(&self) -> F {
        self.params.cell_sizes.floor_size
    }

    /// Returns the total number of cells along the x-axis
    pub fn xcells(&self) -> u32 {
        self.params.cell_per_axis.xcells
    }

    /// Returns the total number of cells along the y-axis
    pub fn ycells(&self) -> u32 {
        self.params.cell_per_axis.ycells
    }

    /// Returns the total number of floors along the z-axis
    pub fn floors(&self) -> usize {
        self.params.cell_per_axis.floors
    }
}

impl<'a, F, T, Hx> fmt::Display for HashGrid<'a, F, T, Hx>
where
    F: Float + FromPrimitive + ToPrimitive + Display,
    Hx: PrimInt + FromPrimitive + ToPrimitive + Hash,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashGrid \n[\n  Grids: {}\n  ", self.grids.len())?;
        write!(
            f,
            "Grid Parameters \n  [\n    floors: {}\n    floor-size: {}\n    cells-on-x: {}\n    cell-size-x: {}\n    cells-on-y: {}\n    cell-size-y: {}\n  ]\n  ",
            self.floors(), self.floor_size(), self.xcells(), self.cell_size_x(), self.ycells(), self.cell_size_y()
        )?;

        let center = self.bounds.centre();
        let size = self.bounds.size();

        write!(
            f,
            "Grid Boundary \n  [\n    Center: (x= {}, y= {}, z= {})\n    size-per-axis: ({}, {}, {})\n  ]\n]",
            center[0], center[1], center[2], size[0], size[1], size[2]
        )?;
        Ok(())
    }
}
