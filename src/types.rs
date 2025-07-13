use std::{array, ops::Index};

pub const D3: usize = 3;

pub struct Point<const D: usize>([f64; D]);

impl<const D: usize> Point<D> {
    pub fn new<F>(elements: [F; D]) -> Self
    where
        F: Into<f64> + Copy,
    {
        Self(array::from_fn(|i| elements[i].into()))
    }
}

impl<const D: usize> Index<usize> for Point<D> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const D: usize> AsRef<[f64; D]> for Point<D> {
    fn as_ref(&self) -> &[f64; D] {
        &self.0
    }
}

pub type Point2D = Point<2>;

impl Point2D {
    pub fn from_xy<F>(x: F, y: F) -> Point2D
    where
        F: Into<f64> + Copy,
    {
        Point::new([x, y])
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }
}

pub type Point3D = Point<3>;

impl Point3D {
    pub fn from_xyz<F>(x: F, y: F, z: F) -> Point3D
    where
        F: Into<f64> + Copy,
    {
        Point::new([x, y, z])
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }
}

pub type EntityId = u64;

pub trait IsEntity<const D: usize> {
    fn id(&self) -> EntityId;
    fn position(&self) -> Point<D>;
}

pub trait Intersectable {
    fn intersects(&self) -> bool;
}

pub trait HasBounds {
    type B: Intersectable;

    fn bounds(&self) -> Self::B;
}

#[cfg(test)]
mod tests {
    use crate::types::Point2D;

    use super::IsEntity;

    struct Object {
        id: u8,
        position: (f32, f32)
    }

    impl IsEntity<2> for Object {
        fn id(&self) -> super::EntityId {
            self.id as super::EntityId
        }

        fn position(&self) -> super::Point<2> {
            let (x, y) = self.position;
            Point2D::from_xy(x, y)
        }
    }

    #[test]
    fn validate() {

    }
}