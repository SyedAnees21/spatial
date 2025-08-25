use std::any::Any;

pub use point::{Point, Point2D, Point3D};
pub use bounds::{Bounds, Bounds2D};

mod point;
mod bounds;

pub const D3: usize = 3;
pub const D2: usize = 2;

// pub trait Intersectable<T> {
//     /// Checks if `self` intersects with `other`.
//     fn intersects_with(&self, other: &T) -> bool;
// }

// // Point2D
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Point2D {
//     pub x: f64,
//     pub y: f64,
// }

// impl Intersectable<Point2D> for Point2D {
//     fn intersects_with(&self, other: &Point2D) -> bool {
//         self == other
//     }
// }

// // Point3D
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Point3D {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64,
// }

// impl Intersectable<Point3D> for Point3D {
//     fn intersects_with(&self, other: &Point3D) -> bool {
//         self == other
//     }
// }

// // Bounds2D (axis-aligned rectangle)
// #[derive(Debug, Clone, Copy)]
// pub struct Bounds2D {
//     pub min: Point2D,
//     pub max: Point2D,
// }

// impl Intersectable<Bounds2D> for Bounds2D {
//     fn intersects_with(&self, other: &Bounds2D) -> bool {
//         self.min.x <= other.max.x
//             && self.max.x >= other.min.x
//             && self.min.y <= other.max.y
//             && self.max.y >= other.min.y
//     }
// }

// impl Intersectable<Point2D> for Bounds2D {
//     fn intersects_with(&self, p: &Point2D) -> bool {
//         p.x >= self.min.x && p.x <= self.max.x
//             && p.y >= self.min.y && p.y <= self.max.y
//     }
// }

// impl Intersectable<Circle> for Bounds2D {
//     fn intersects_with(&self, c: &Circle) -> bool {
//         let closest_x = c.center.x.clamp(self.min.x, self.max.x);
//         let closest_y = c.center.y.clamp(self.min.y, self.max.y);
//         let dx = c.center.x - closest_x;
//         let dy = c.center.y - closest_y;
//         dx * dx + dy * dy <= c.radius * c.radius
//     }
// }

// // Bounds3D (axis-aligned box)
// #[derive(Debug, Clone, Copy)]
// pub struct Bounds3D {
//     pub min: Point3D,
//     pub max: Point3D,
// }

// impl Intersectable<Bounds3D> for Bounds3D {
//     fn intersects_with(&self, other: &Bounds3D) -> bool {
//         self.min.x <= other.max.x
//             && self.max.x >= other.min.x
//             && self.min.y <= other.max.y
//             && self.max.y >= other.min.y
//             && self.min.z <= other.max.z
//             && self.max.z >= other.min.z
//     }
// }

// impl Intersectable<Point3D> for Bounds3D {
//     fn intersects_with(&self, p: &Point3D) -> bool {
//         p.x >= self.min.x && p.x <= self.max.x
//             && p.y >= self.min.y && p.y <= self.max.y
//             && p.z >= self.min.z && p.z <= self.max.z
//     }
// }

// impl Intersectable<Sphere> for Bounds3D {
//     fn intersects_with(&self, s: &Sphere) -> bool {
//         let closest_x = s.center.x.clamp(self.min.x, self.max.x);
//         let closest_y = s.center.y.clamp(self.min.y, self.max.y);
//         let closest_z = s.center.z.clamp(self.min.z, self.max.z);
//         let dx = s.center.x - closest_x;
//         let dy = s.center.y - closest_y;
//         let dz = s.center.z - closest_z;
//         dx * dx + dy * dy + dz * dz <= s.radius * s.radius
//     }
// }

// // Circle
// #[derive(Debug, Clone, Copy)]
// pub struct Circle {
//     pub center: Point2D,
//     pub radius: f64,
// }

// impl Intersectable<Circle> for Circle {
//     fn intersects_with(&self, other: &Circle) -> bool {
//         let dx = self.center.x - other.center.x;
//         let dy = self.center.y - other.center.y;
//         let dist_sq = dx * dx + dy * dy;
//         let rad_sum = self.radius + other.radius;
//         dist_sq <= rad_sum * rad_sum
//     }
// }

// impl Intersectable<Point2D> for Circle {
//     fn intersects_with(&self, p: &Point2D) -> bool {
//         let dx = self.center.x - p.x;
//         let dy = self.center.y - p.y;
//         dx * dx + dy * dy <= self.radius * self.radius
//     }
// }

// impl Intersectable<Bounds2D> for Circle {
//     fn intersects_with(&self, b: &Bounds2D) -> bool {
//         b.intersects_with(self)
//     }
// }

// // Sphere
// #[derive(Debug, Clone, Copy)]
// pub struct Sphere {
//     pub center: Point3D,
//     pub radius: f64,
// }

// impl Intersectable<Sphere> for Sphere {
//     fn intersects_with(&self, other: &Sphere) -> bool {
//         let dx = self.center.x - other.center.x;
//         let dy = self.center.y - other.center.y;
//         let dz = self.center.z - other.center.z;
//         let dist_sq = dx * dx + dy * dy + dz * dz;
//         let rad_sum = self.radius + other.radius;
//         dist_sq <= rad_sum * rad_sum
//     }
// }

// impl Intersectable<Point3D> for Sphere {
//     fn intersects_with(&self, p: &Point3D) -> bool {
//         let dx = self.center.x - p.x;
//         let dy = self.center.y - p.y;
//         let dz = self.center.z - p.z;
//         dx * dx + dy * dy + dz * dz <= self.radius * self.radius
//     }
// }

// impl Intersectable<Bounds3D> for Sphere {
//     fn intersects_with(&self, b: &Bounds3D) -> bool {
//         b.intersects_with(self)
//     }
// }
