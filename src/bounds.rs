// use nalgebra::{Point, Point2, Scalar};
use num_traits::Zero;

use crate::types::{Point, Point2D};

pub struct Bounds<const D: usize> {
    min: Point<D>,
    max: Point<D>,
}

impl<const D: usize> Bounds<D> {
    pub fn new<F>(min: [F; D], max: [F; D]) -> Self
    where
        F: Into<f64> + Copy,
    {
        let p_min = Point::new(min);
        let p_max = Point::new(max);

        Self {
            min: p_min,
            max: p_max,
        }
    }

    pub fn min(&self) -> Point<D> {
        self.min
    }

    pub fn max(&self) -> Point<D> {
        self.max
    }

    pub fn size(&self) -> Point<D> {
        self.max - self.min
    }
}

pub type Bounds2D = Bounds<2>;
