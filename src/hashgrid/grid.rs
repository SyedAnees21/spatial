use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    hash::Hash,
};

use num_traits::{Float, FromPrimitive, One, PrimInt, ToPrimitive};

use super::{Boundary, Coordinate, DataIndex, Entity, HashIndex, Query, QueryResult, QueryType};

pub type Grid<K, V> = HashMap<K, V>;
pub type Floors<T> = Vec<T>;
pub type DataRef<'a, T> = &'a T;

pub type DefaultHx = u64;
// pub type DefaultDx = usize;

// pub struct Data<'a, T, Dx = DefaultDx> {
//     pub index: Dx,
//     pub refer: DataRef<'a, T>
// }


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
#[derive(Debug)]
pub struct CellSizes<F> {
    x_size: F,
    y_size: F,
    floor_size: F,
}

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

#[derive(Debug)]
pub struct GridParameters<F> {
    pub cell_per_axis: CellsPerAxis,
    pub cell_sizes: CellSizes<F>,
}

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
#[derive(Debug)]
pub struct HashGrid<'a, F, T, Hx = DefaultHx> {
    pub grids: Floors<Grid<Hx, Vec<DataRef<'a, T>>>>,
    pub params: GridParameters<F>,
    pub bounds: GridBoundary<F>,
    pub wrap: bool,
}

impl<'a, F, T, Hx> HashGrid<'a, F, T, Hx>
where
    F: Float + FromPrimitive + ToPrimitive,
    Hx: PrimInt + FromPrimitive + ToPrimitive + Hash,
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
        B: Boundary<Item = F>,
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
        let length_on_x = bounds.size()[0];
        let length_on_y = bounds.size()[1];
        let length_on_z = bounds.size()[2];

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
            center: bounds.centre(),
            size: bounds.size(),
        };

        Self {
            grids: Floors::with_capacity(floors),
            params,
            bounds,
            wrap,
        }
    }

    pub fn insert(&mut self, entity: DataRef<'a, T>)
    where
        T: Coordinate<Item = F> + Entity,
    {
        // Getting the grid's extreme boundary parameters to apply the boundary
        // limits to the calculated cell cords if necessary
        let grid_max_bounds = self.bounds.max();
        let grid_min_bounds = self.bounds.min();

        let mut coodrinates = (entity.x(), entity.y(), entity.z());

        // Validating if the point is within the grid bounds
        if !self.bounds.is_inside(coodrinates) {
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
        let (cx, cy, floor) = self.get_cell_coordinates(coodrinates);

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

    pub fn query<Id>(&self, query: Query<F, Id>) -> QueryResult<'a, F, Id, T>
    where
        Id: DataIndex,
        T: Coordinate<Item = F> + Entity<ID = Id>,
    {
        let radius_x = (F::from_u32(self.xcells()).unwrap() * query.radius())
            .max(F::one())
            .ceil()
            .to_i32()
            .unwrap();
        let radius_y = (F::from_u32(self.ycells()).unwrap() * query.radius())
            .max(F::one())
            .ceil()
            .to_i32()
            .unwrap();
        let radius_f = (F::from_usize(self.floors()).unwrap() * query.radius())
            .max(F::one())
            .ceil()
            .to_i32()
            .unwrap();

        let (cx, cy, floor) = self.get_cell_coordinates((query.x(), query.y(), query.z()));

        let base_cx = cx as i32;
        let base_cy = cy as i32;
        let base_floor = floor as i32;

        let range_x = (base_cx - radius_x).max(0)..=(base_cx + radius_x).min(self.xcells() as i32);
        let range_y = (base_cy - radius_y).max(0)..=(base_cy + radius_y).min(self.ycells() as i32);
        let range_z =
            (base_floor - radius_f).max(0)..=(base_floor + radius_f).min(self.floors() as i32);

        let relevant_indices = range_x
            .clone()
            .flat_map(|dx| {
                let range_z = range_z.clone();
                range_y.clone().flat_map(move |dy| {
                    range_z
                        .clone()
                        .map(move |dz| (dx as u32, dy as u32, dz as usize))
                })
            })
            .map(|(dx, dy, df)| (self.key(dx, dy), df));

        let mut result = QueryResult {
            query,
            data: Vec::new(),
        };

        match query.query_type() {
            QueryType::Find(id) => {
                for (hashindex, floor) in relevant_indices {
                    if let Some(d_list) = self.grids[floor].get(&hashindex.key()) {
                        if let Some(&entity) = d_list.iter().find(|&&d| d.id() == id) {
                            result.data.push(entity);
                            break;
                        }
                    }
                }
            }
            QueryType::Relevant => {
                for (hashindex, floor) in relevant_indices {
                    if let Some(d_list) = self.grids[floor].get(&hashindex.key()) {
                        result.data.copy_from_slice(d_list);
                    }
                }
            }
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
        T: Coordinate<Item = F> + Entity,
    {
        // Getting the grid's extreme boundary parameters to apply the boundary
        // limits to the calculated cell cords if necessary
        let grid_max_bounds = self.bounds.max();
        let grid_min_bounds = self.bounds.min();

        for entity in data.iter() {
            // Getting the cell coordinates from entity coordinates
            // z-axis from the entity coordinates defines at which floor of the grid
            // to look for the cell
            let mut coodrinates = (entity.x(), entity.y(), entity.z());

            // Wrapping around the nearest grid bounds if the wrap is enabled and the
            // entity is outside the grid bounds or else do not add the entity inside the grid
            if !self.bounds.is_inside(coodrinates) {
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
            let (cx, cy, floor) = self.get_cell_coordinates(coodrinates);

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
    pub fn get_cell_coordinates(&self, coordinates: (F, F, F)) -> (u32, u32, usize) {
        // Destructuring the entity coordinates into x, y, z components
        let (x, y, z) = coordinates;

        // Normalizing the x and y component according to cell size to find the
        // cell coordinates inside the grid
        let cx = (x / self.cell_size_x()).floor().to_u32().unwrap();
        let cy = (y / self.cell_size_y()).floor().to_u32().unwrap();

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
        (((k1 + k2) * (k1 + k2 + 1)) / 2 + k2).into()
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
