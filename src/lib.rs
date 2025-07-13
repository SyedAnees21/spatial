use std::ops::{Add, Sub};
pub use hashgrid::{Boundary, DataIndex, HashGrid, HashIndex};
pub use quad::QuadTree;
pub use traits::Float;

pub mod hashgrid;
mod codec;
mod quad;
mod traits;
mod types;

mod tests;

#[derive(Debug)]
pub enum SpatialError {
    InvalidBounds,
    InvalidCapacity,
    OutOfBounds,
}

#[derive(Debug, Copy, Clone)]
pub enum Geometry {
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

impl Geometry {
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
        let min_x = min.0.into();
        let min_y = min.1.into();
        let max_x = max.0.into();
        let max_y = max.1.into();

        let center = ((max_x + min_x) / 2., (max_y + min_y) / 2.);
        let size = (max_x - min_x, max_y - min_y);

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

impl Geometry {
    pub fn intersects(&self, other: Self) -> bool {
        use Geometry::*;

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
                let (min_1, max_1) = Geometry::rect_min_max(c1, s1);
                let (min_2, max_2) = Geometry::rect_min_max(c2, s2);

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
                Geometry::disctance_squared(c1, c2) <= sum * sum
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
                let ((min_x, min_y), (max_x, max_y)) = Geometry::rect_min_max(bc, bs);
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
                Geometry::disctance_squared((cx, cy), rc) <= r * r
            }
            _ => {
                panic!("Geometric-Types only supports intersection between Bounds and radius combinations")
            }
        }
    }

    pub fn contains(&self, other: Self) -> bool {
        use Geometry::*;

        const TOP_RIGHT: usize = 0;
        const TOP_LEFT: usize = 1;
        const BOTTOM_RIGHT: usize = 2;
        const BOTTOM_LEFT: usize = 3;

        match (*self, other) {
            (Rect { center, size }, Point { x, y }) => {
                let (min, max) = Geometry::rect_min_max(center, size);

                x >= min.0 && y >= min.1 && x <= max.0 && y <= max.1
            },
            (Radius { radius, center }, Point { x, y }) => {
                Geometry::disctance_squared(center, (x, y)) <= radius * radius
            },
            (Radius { radius: r1, center: c1 }, Radius { radius:r2, center:c2 }) => {
                Geometry::disctance_squared(c1, c2) <= (r1 - r2).powi(2) && r1 > r2
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
                let (min_1, max_1) = Geometry::rect_min_max(c1, s1);
                let (min_2, max_2) = Geometry::rect_min_max(c2, s2);

                min_1.0 <= min_2.0 && min_1.1 <= min_2.1 && max_1.0 >= max_2.0 && max_1.1 >= max_2.1
            },
            (
                Rect { center: cb, size },
                Radius {
                    radius: r,
                    center: c,
                },
            ) => {
                let (min, max) = Geometry::rect_min_max(cb, size);
                c.0 - r >= min.0 && c.0 + r <= max.0 && c.1 - r >= min.1 && c.1 + r <= max.1
            },
            (
                Radius {
                    radius: r,
                    center: c,
                },
                Rect { center, size },
            ) => {
                let b_corners = Geometry::rect_corners(center, size);

                Geometry::disctance_squared(c, b_corners[TOP_RIGHT]) <= r * r
                    && Geometry::disctance_squared(c, b_corners[TOP_LEFT]) <= r * r
                    && Geometry::disctance_squared(c, b_corners[BOTTOM_RIGHT]) <= r * r
                    && Geometry::disctance_squared(c, b_corners[BOTTOM_LEFT]) <= r * r
            },
            
            _ => panic!("Geometric-Types only supports containment between Bounds-radius, radius-Bounds, Bounds-Point, Radius-Point, Bounds-Bounds, Radius-Radius")
            
        }
    }
}

impl Geometry {
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
        let (min, max) = Geometry::rect_min_max(center, size);

        [
            max,            // Top-right corner
            (min.0, max.1), // Top-left corner
            (max.0, min.1), // Bottom-right corner
            min,            // Bottom-left corner
        ]
    }
}


pub type EntityID = u64;

pub trait IsEntity: Clone {
    fn id(&self) -> EntityID;
    fn position(&self) -> (f64, f64);
    fn bounds(&self) -> Geometry;
    fn contains_geometry(&self, geometry: Geometry) -> bool;
    fn intersects_geometry(&self, geometry: Geometry) -> bool;
}

#[macro_export]
macro_rules! ensure {
    ($match:expr, $err:expr) => {
        if !$match {
            return Err($err);
        }
    };
}
