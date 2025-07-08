use std::ops::{Add, Sub};

use crate::{quad::QuadTree, Geometry, IsEntity, SpatialError};

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

impl From<Vector2D> for (f64, f64) {
    fn from(value: Vector2D) -> Self {
        (value.0, value.1)
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
}

#[derive(Debug, Clone)]
pub struct Player {
    id: u64,
    bounds: Bounds2D,
    position: Vector2D,
}

impl Player {
    fn id(&self) -> u64 {
        self.id
    }
}

impl From<(u64, Bounds2D, Vector2D)> for Player {
    fn from(value: (u64, Bounds2D, Vector2D)) -> Self {
        Self {
            id: value.0,
            bounds: value.1,
            position: value.2,
        }
    }
}

impl IsEntity for Player {
    fn id(&self) -> crate::EntityID {
        self.id
    }

    fn position(&self) -> (f64, f64) {
        self.position.xy()
    }

    fn bounds(&self) -> crate::Geometry {
        let size = self.bounds.max - self.bounds.min;
        Geometry::rect(self.position(), size.xy())
    }

    fn intersects_geometry(&self, geometry: Geometry) -> bool {
        self.bounds().intersects(geometry)
    }

    fn contains_geometry(&self, geometry: Geometry) -> bool {
        self.bounds().contains(geometry)
    }
}

#[test]
fn quadtree_smoke() -> Result<(), SpatialError> {
    let mut quadtree = QuadTree::<Player>::new((0, 0).into(), (10, 10).into(), 1)?;

    println!("{}", quadtree.levels());

    let player_1: Player = (
        1,
        Bounds2D::new((6, 6).into(), (9, 9).into()),
        (7.5, 7.5).into(),
    )
        .into();

    assert!(quadtree.insert(player_1)?);

    println!("{}", quadtree.levels());

    let player_2: Player = (
        2,
        Bounds2D::new((1, 1).into(), (2, 2).into()),
        (1.5, 1.5).into(),
    )
        .into();

    assert!(quadtree.insert(player_2)?);

    println!("{}", quadtree.levels());

    let query_result =
        quadtree.query_and_filter(Geometry::point(1.5, 1.5), |_, player| player.id() == 1);

    assert!(query_result.is_empty());

    let query_result =
        quadtree.query_and_filter(Geometry::point(1.5, 1.5), |_, player| player.id() == 2);

    assert!(!query_result.is_empty());

    println!("{:?}", query_result.collect::<Vec<_>>());

    println!(
        "Player 1 path in tree {:?}",
        quadtree.entity_path_in_tree(1)
    );
    println!(
        "Player 2 path in tree {:?}",
        quadtree.entity_path_in_tree(2)
    );

    Ok(())
}
