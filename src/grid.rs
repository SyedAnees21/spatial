use crate::{
    object::IsEntity2D,
    partition::{SpatialInsertion, SpatialQuery},
    relevance::Relevance,
    types::{Bounds2D, Point2D},
};

pub struct HashGrid<T> {
    grid: Grid<T>,
    boundary: Bounds2D,
    cell_size: [f64; 2],
}

impl<T: Clone> HashGrid<T> {
    pub fn new(bounds_min: [f32; 2], bounds_max: [f32; 2], [xcells, ycells]: [u32; 2]) -> Self {
        Self::new_with_capacity(bounds_min, bounds_max, [xcells, ycells], 0)
    }

    pub fn new_with_capacity(
        bounds_min: [f32; 2],
        bounds_max: [f32; 2],
        [xcells, ycells]: [u32; 2],
        capacity: usize,
    ) -> Self {
        let boundary = Bounds2D::new(bounds_min, bounds_max);

        let [x, y] = boundary.size().xy();

        let cell_width = (x / xcells as f64).floor();
        let cell_height = (y / ycells as f64).floor();

        Self {
            grid: Grid::with_capacity(xcells as usize, ycells as usize, capacity),
            boundary,
            cell_size: [cell_width, cell_height],
        }
    }

    pub fn cells_vertical(&self) -> usize {
        self.grid.height()
    }

    pub fn cells_horizontal(&self) -> usize {
        self.grid.width()
    }
}

impl<T> SpatialInsertion for HashGrid<T>
where
    T: Clone + IsEntity2D,
{
    type Object = T;

    fn insert(&mut self, object: T) -> bool {
        let b_min = self.boundary.min();
        let adjusted_pos = object.position() - b_min;

        let [c_width, c_height] = self.cell_size;

        let x = (adjusted_pos.x() / c_width).floor() as usize;
        let y = (adjusted_pos.y() / c_height).floor() as usize;

        self.grid.insert(x, y, object, IndexMajor::Row)
    }
}

impl<T: Clone> SpatialQuery for HashGrid<T> {
    type Objects = Vec<T>;
    type Query = Point2D;

    fn query(
        &self,
        query: Self::Query,
        relevance: Relevance,
    ) -> impl Iterator<Item = &Self::Objects> {
        let b_min = self.boundary.min();
        let adjusted_query = query - b_min;

        let xcells = self.cells_horizontal();
        let ycells = self.cells_vertical();

        let x_proximity = (xcells as f64 * *relevance).round() as usize;
        let y_proximity = (ycells as f64 * *relevance).round() as usize;

        let [c_width, c_height] = self.cell_size;

        let x = (adjusted_query.x() / c_width).floor() as usize;
        let y = (adjusted_query.y() / c_height).floor() as usize;

        let x_range = x.saturating_sub(x_proximity)..=(x + x_proximity).min(xcells);
        let y_range = y.saturating_sub(y_proximity)..=(y + y_proximity).min(ycells);

        self.grid.get_by_range(x_range, y_range, IndexMajor::Row)
    }

    // fn query_mut(
    //     &mut self,
    //     query: Self::Query,
    //     relevance: Relevance,
    // ) -> impl Iterator<Item = &mut Self::Objects> {
    //     let b_min = self.boundary.min();
    //     let adjusted_query = query - b_min;

    //     let [c_width, c_height] = self.cell_size;

    //     let x = (adjusted_query.x() / c_width).floor() as usize;
    //     let y = (adjusted_query.y() / c_height).floor() as usize;

    //     self.grid
    //         .get_mut(x, y, IndexMajor::Row)
    //         .into_iter()
    //         .flatten()
    // }

    fn contains(&self, query: Self::Query, relevance: Relevance) -> bool {
        let b_min = self.boundary.min();
        let adjusted_query = query - b_min;

        let [c_width, c_height] = self.cell_size;

        let x = (adjusted_query.x() / c_width).floor() as usize;
        let y = (adjusted_query.y() / c_height).floor() as usize;

        self.grid.contains(x, y, IndexMajor::Row)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndexMajor {
    Row,
    Column,
}

struct Grid<T> {
    width: usize,
    height: usize,
    storage: Vec<Vec<T>>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self::with_capacity(width, height, 0)
    }

    pub fn with_capacity(width: usize, height: usize, capacity: usize) -> Self {
        Self {
            width,
            height,
            storage: vec![Vec::with_capacity(capacity); width * height],
        }
    }

    pub fn insert(&mut self, x: usize, y: usize, object: T, major: IndexMajor) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let index = self.compute_index(x, y, major);
        let cell = &mut self.storage[index];
        cell.push(object);

        true
    }

    pub fn contains(&self, x: usize, y: usize, major: IndexMajor) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let index = self.compute_index(x, y, major);
        !self.storage[index].is_empty()
    }

    #[inline(always)]
    pub fn total_cells(&self) -> usize {
        self.width * self.height
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline(always)]
    pub fn compute_index(&self, x: usize, y: usize, major: IndexMajor) -> usize {
        match major {
            IndexMajor::Row => y * self.width + x,
            IndexMajor::Column => x * self.height + y,
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn get(&self, x: usize, y: usize, major: IndexMajor) -> Option<&Vec<T>> {
        if x < self.width && y < self.height {
            return None;
        }

        let index = self.compute_index(x, y, major);
        Some(&self.storage[index])
    }

    pub fn get_mut(&mut self, x: usize, y: usize, major: IndexMajor) -> Option<&mut Vec<T>> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = self.compute_index(x, y, major);
        Some(&mut self.storage[index])
    }

    pub fn get_by_range<R>(&self, x_range: R, y_range: R, major: IndexMajor) -> GridCells<'_, T>
    where
        R: Iterator<Item = usize> + Clone,
    {
        x_range
            .flat_map(move |x| y_range.clone().map(move |y| self.get(x, y, major)))
            .flatten()
            .collect()
    }
}

pub struct GridCells<'a, T> {
    list: Vec<&'a Vec<T>>,
}

impl<'a, T> FromIterator<&'a Vec<T>> for GridCells<'a, T> {
    fn from_iter<I: IntoIterator<Item = &'a Vec<T>>>(iter: I) -> Self {
        Self {
            list: iter.into_iter().collect(),
        }
    }
}

impl<'a, T> Iterator for GridCells<'a, T> {
    type Item = &'a Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}
