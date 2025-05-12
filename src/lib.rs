use std::{alloc::GlobalAlloc, marker::PhantomData};

pub use hashgrid::{Boundary, DataIndex, HashGrid};
use traits::Float;

pub mod hashgrid;
mod traits;

mod tests;

#[derive(Debug)]
pub struct Vertex<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

#[macro_export]
macro_rules! vertex {
    ($type:ty, $x:expr, $y:expr, $z:expr) => {
        Vertex {
            x: <$type>::from_i32($x as i32),
            y: <$type>::from_i32($y as i32),
            z: <$type>::from_i32($z as i32),
        }
    };
    ($x:expr, $y:expr, $z:expr) => {
        Vertex {
            x: $x as f32,
            y: $y as f32,
            z: $z as f32,
        }
    };
    ($x:expr, $y:expr) => {
        Vertex {
            x: $x as f32,
            y: $y as f32,
            z: 0.0,
        }
    };
    () => {
        Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    };
}

pub type Floors<T> = Vec<T>;
pub type Rows<T> = Vec<T>;
pub type Columns<T> = Vec<T>;

pub struct Metadata {

}

pub struct Grids<T> {
    pub grids: Floors<Rows<Columns<T>>>,
    pub metadata: Metadata,
}

impl<T> Grids<T> {
    pub fn new(x: usize, y: usize, z: usize) -> Grids<T> {
        Self {
            grids: Floors::new(),
            metadata: Metadata {  }
        }
    }
}