use std::ops::{Add, Sub};

pub use hashgrid::{Boundary, DataIndex, HashGrid, HashIndex};
pub use quad::QuadTree;
pub use traits::Float;

mod codec;
pub mod hashgrid;
mod quad;
mod traits;

mod tests;

#[derive(Debug)]
pub enum SpatialError {
    InvalidBounds,
    InvalidCapacity,
    OutOfBounds,
}

#[derive(Debug, Copy, Clone)]
pub enum GeoTypes {
    Point {
        x: f64,
        y: f64,
    },
    Rect {
        center: (f64, f64),
        size: (f64, f64),
    },
    Radius {
        radius: f64,
        center: (f64, f64),
    },
}

impl GeoTypes {
    pub fn point<T>(x: T, y: T) -> Self
    where
        T: Into<f64> + Copy,
    {
        Self::Point {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn rect<T>(center: (T, T), size: (T, T)) -> Self
    where
        T: Into<f64> + Copy,
    {
        Self::Rect {
            center: (center.0.into(), center.1.into()),
            size: (size.0.into(), size.1.into()),
        }
    }

    pub fn rect_from_min_max<T>(min: (T,T), max: (T,T)) -> Self 
    where
        T: Into<f64> + Copy,
    {
        let (min_x, min_y) = min;
        let (max_x, max_y) = max;

        let center = ((max_x.into() + min_x.into()) / 2., (max_y.into() + min_y.into()) / 2.);
        let size = (max_x.into() - min_x.into(), max_y.into() - min_y.into());

        Self::Rect { center, size }
    }

    pub fn radius<T>(radius: T, center: (T, T)) -> Self
    where
        T: Into<f64> + Copy,
    {
        Self::Radius {
            radius: radius.into(),
            center: (center.0.into(), center.1.into()),
        }
    }
}

impl GeoTypes {
    pub fn intersects(&self, other: Self) -> bool {
        use GeoTypes::*;

        match (*self, other) {
            (
                Rect {
                    center: c1,
                    size: s1,
                },
                Rect {
                    center: c2,
                    size: s2,
                },
            ) => {
                let (min_1, max_1) = GeoTypes::rect_min_max(c1, s1);
                let (min_2, max_2) = GeoTypes::rect_min_max(c2, s2);

                min_1.0 < max_2.0 && max_1.0 > min_2.0 && min_1.1 < max_2.1 && max_1.1 > min_2.1
            }
            (
                Radius {
                    radius: r1,
                    center: c1,
                },
                Radius {
                    radius: r2,
                    center: c2,
                },
            ) => {
                let sum = r1 + r2;
                GeoTypes::disctance_squared(c1, c2) <= sum * sum
            }
            (
                Rect {
                    center: bc,
                    size: bs,
                },
                Radius {
                    radius: r,
                    center: rc,
                },
            )
            | (
                Radius {
                    radius: r,
                    center: rc,
                },
                Rect {
                    center: bc,
                    size: bs,
                },
            ) => {
                // First compute AABB‐min,max for the Bounds
                let ((min_x, min_y), (max_x, max_y)) = GeoTypes::rect_min_max(bc, bs);
                // Then find the closest point in the rectangle to the circle center:
                let cx = if rc.0 < min_x {
                    min_x
                } else if rc.0 > max_x {
                    max_x
                } else {
                    rc.0
                };
                let cy = if rc.1 < min_y {
                    min_y
                } else if rc.1 > max_y {
                    max_y
                } else {
                    rc.1
                };
                // If that closest point is within radius distance, they intersect:
                GeoTypes::disctance_squared((cx, cy), rc) <= r * r
            }
            _ => {
                panic!("Geometric-Types only supports intersection between Bounds and radius combinations")
            }
        }
    }

    pub fn contains(&self, other: Self) -> bool {
        use GeoTypes::*;

        const TOP_RIGHT: usize = 0;
        const TOP_LEFT: usize = 1;
        const BOTTOM_RIGHT: usize = 2;
        const BOTTOM_LEFT: usize = 3;

        match (*self, other) {
            (Rect { center, size }, Point { x, y }) => {
                let (min, max) = GeoTypes::rect_min_max(center, size);

                x >= min.0 && y >= min.1 && x <= max.0 && y <= max.1
            },
            (Radius { radius, center }, Point { x, y }) => {
                GeoTypes::disctance_squared(center, (x, y)) <= radius * radius
            },
            (Radius { radius: r1, center: c1 }, Radius { radius:r2, center:c2 }) => {
                GeoTypes::disctance_squared(c1, c2) <= (r1 - r2).powi(2) && r1 > r2
            }
            (
                Rect {
                    center: c1,
                    size: s1,
                },
                Rect {
                    center: c2,
                    size: s2,
                },
            ) => {
                let (min_1, max_1) = GeoTypes::rect_min_max(c1, s1);
                let (min_2, max_2) = GeoTypes::rect_min_max(c2, s2);

                min_1.0 <= min_2.0 && min_1.1 <= min_2.1 && max_1.0 >= max_2.0 && max_1.1 >= max_2.1
            },
            (
                Rect { center: cb, size },
                Radius {
                    radius: r,
                    center: c,
                },
            ) => {
                let (min, max) = GeoTypes::rect_min_max(cb, size);
                c.0 - r >= min.0 && c.0 + r <= max.0 && c.1 - r >= min.1 && c.1 + r <= max.1
            },
            (
                Radius {
                    radius: r,
                    center: c,
                },
                Rect { center, size },
            ) => {
                let b_corners = GeoTypes::rect_corners(center, size);

                GeoTypes::disctance_squared(c, b_corners[TOP_RIGHT]) <= r * r
                    && GeoTypes::disctance_squared(c, b_corners[TOP_LEFT]) <= r * r
                    && GeoTypes::disctance_squared(c, b_corners[BOTTOM_RIGHT]) <= r * r
                    && GeoTypes::disctance_squared(c, b_corners[BOTTOM_LEFT]) <= r * r
            },
            
            _ => panic!("Geometric-Types only supports containment between Bounds-radius, radius-Bounds, Bounds-Point, Radius-Point, Bounds-Bounds, Radius-Radius")
            
        }
    }
}

impl GeoTypes {
    fn rect_min_max(center: (f64, f64), size: (f64, f64)) -> ((f64, f64), (f64, f64)) {
        let (cx, cy) = center;
        let (sx, sy) = size;

        let hsx = sx * 0.5;
        let hsy = sy * 0.5;

        ((cx - hsx, cy - hsy), (cx + hsx, cy + hsy))
    }

    fn disctance_squared(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        let dx = p1.0 - p2.0;
        let dy = p1.1 - p2.1;

        dx * dx + dy * dy
    }

    fn rect_corners(center: (f64, f64), size: (f64, f64)) -> [(f64, f64); 4] {
        let (min, max) = GeoTypes::rect_min_max(center, size);

        [
            max,            // Top-right corner
            (min.0, max.1), // Top-left corner
            (max.0, min.1), // Bottom-right corner
            min,            // Bottom-left corner
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2D(f64, f64);

impl Vector2D {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
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
        Self {
            min: center - half_size,
            max: center + half_size,
        }
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

pub type EntityID = u64;

pub trait IsEntity: Clone {
    fn id(&self) -> EntityID;
    fn position(&self) -> Vector2D;
    fn bounding_box(&self) -> Bounds2D;
}

#[macro_export]
macro_rules! ensure {
    ($match:expr, $err:expr) => {
        if !$match {
            return Err($err);
        }
    };
}
