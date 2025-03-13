use std::{alloc::GlobalAlloc, marker::PhantomData};

pub use hashgrid::{Boundary, DataIndex, HashGrid, HashIndex};
use traits::Float;

pub mod hashgrid;
mod traits;

mod tests;

#[derive(Debug)]
pub struct Vertex<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F
}

#[macro_export]
macro_rules! vertex {
    ($x:expr, $y:expr, $z:expr) => {
        Vertex {
            x: $x as f32,
            y: $y as f32,
            z: $z as f32
        }
    };
    ($x:expr, $y:expr) => {
        Vertex {
            x: $x as f32,
            y: $y as f32,
            z: 0.0
        }
    };
    () => {
        Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    };
}
