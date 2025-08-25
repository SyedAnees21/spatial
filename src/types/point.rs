use crate::types::{D2, D3};
use std::{
    array,
    ops::{Add, Deref, DerefMut, Index, Sub},
};

#[derive(Clone, Copy)]
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

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const D: usize> AsRef<[f64; D]> for Point<D> {
    #[inline]
    fn as_ref(&self) -> &[f64; D] {
        &self.0
    }
}

impl<const D: usize> Default for Point<D> {
    #[inline]
    fn default() -> Self {
        Self(array::from_fn(|_| f64::default()))
    }
}

impl<const D: usize> Sub for Point<D> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(array::from_fn(|i| self[i] - rhs[i]))
    }
}

impl<const D: usize> Add for Point<D> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(array::from_fn(|i| self[i] + rhs[i]))
    }
}

pub type Point2D = Point<D2>;

impl Point2D {
    #[inline]
    pub fn from_xy<F>(x: F, y: F) -> Point2D
    where
        F: Into<f64> + Copy,
    {
        Point::new([x, y])
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self[1]
    }

    #[inline]
    pub fn xy(&self) -> [f64; 2] {
        self.as_ref().clone()
    }
}

#[repr(C)]
pub struct XY {
    pub x: f64,
    pub y: f64,
}

/// Deref to `XY`, so `p.x` and `p.y` work
impl Deref for Point2D {
    type Target = XY;

    #[inline]
    fn deref(&self) -> &XY {
        // SAFETY: `[f64;2]` has identical layout to `XY` under `repr(C)`
        unsafe { &*(self.0.as_ptr() as *const XY) }
    }
}
/// If you also want to mutate via `.x` / `.y`:
impl DerefMut for Point2D {
    #[inline]
    fn deref_mut(&mut self) -> &mut XY {
        unsafe { &mut *(self.0.as_mut_ptr() as *mut XY) }
    }
}

pub type Point3D = Point<D3>;

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

#[repr(C)]
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Deref to `XY`, so `p.x` and `p.y` work
impl Deref for Point3D {
    type Target = XYZ;

    #[inline]
    fn deref(&self) -> &XYZ {
        // SAFETY: `[f64;2]` has identical layout to `XY` under `repr(C)`
        unsafe { &*(self.0.as_ptr() as *const XYZ) }
    }
}
/// If you also want to mutate via `.x` / `.y`:
impl DerefMut for Point3D {
    #[inline]
    fn deref_mut(&mut self) -> &mut XYZ {
        unsafe { &mut *(self.0.as_mut_ptr() as *mut XYZ) }
    }
}
