use std::ops::{Add, Sub};

pub use hashgrid::{Boundary, DataIndex, HashGrid, HashIndex};

pub mod hashgrid;
mod quad;
mod tests;

#[derive(Debug)]
pub enum SpatialError {
    InvalidBounds,
    InvalidCapacity,
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2D(f64, f64);

impl Vector2D {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.0
    }

    pub fn xy(&self) -> (f64, f64) {
        (self.0, self.1)
    }

    pub fn div(self, scalar: f64) -> Self {
        (self.0 / scalar, self.1 / scalar).into()
    }
}

impl Add for Vector2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0, self.1 + rhs.1).into()
    }
}

impl Sub for Vector2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 - rhs.0, self.1 - rhs.1).into()
    }
}

impl<T> From<(T, T)> for Vector2D
where
    T: Into<f64>,
{
    fn from(value: (T, T)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds2D<Point = Vector2D> {
    min: Point,
    max: Point,
}

impl Bounds2D {
    pub fn new(min: Vector2D, max: Vector2D) -> Self {
        Self { min, max }
    }

    pub fn from_center_size(center: Vector2D, size: Vector2D) -> Self {
        let half_size = size.div(2.0);
        Self { min: center - half_size, max: center + half_size }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.min.x() <= other.min.x()
            && self.min.y() <= other.min.y()
            && self.max.x() >= other.max.x()
            && self.max.y() >= other.max.y()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x() < other.max.x()
            && self.max.x() > other.min.x()
            && self.min.y() < other.max.y()
            && self.max.y() > other.min.y()
    }

    pub fn contains_point(&self, point: Vector2D) -> bool {
        point.x() >= self.min.x()
            && point.y() >= self.min.y()
            && point.x() <= self.max.x()
            && point.y() <= self.max.y()
    }
}

pub trait HasPosition {
    fn position(&self) -> Vector2D;
}

pub trait HasBounds {
    fn bounding_box(&self) -> Bounds2D;
}
