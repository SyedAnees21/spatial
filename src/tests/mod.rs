#![cfg(test)]

use num_traits::identities;

use crate::{
    quad::QuadTree, Bounds2D, IsEntity, SpatialError, Vector2D,
};

mod boundary;
mod grid;
mod base4;
mod geometry;

#[test]
fn into_coordinates() {
    let _: Vector2D = (1_u32, 2_u32).into();
    let _: Vector2D = (1_i32, 2_i32).into();
    let _: Vector2D = (1_f32, 2_f32).into();
    let _: Vector2D = (1_f64, 2_f64).into();
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

    fn position(&self) -> Vector2D {
        self.position
    }

    fn bounding_box(&self) -> Bounds2D {
        self.bounds
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
        quadtree.query_point_and_filter((1.5, 1.5).into(), |_, player| player.id() == 1);

    assert!(query_result.is_none());

    let query_result =
        quadtree.query_point_and_filter((1.5, 1.5).into(), |_, player| player.id() == 2);

    assert!(query_result.is_some());

    println!("{:?}", query_result);

    for node in quadtree.iter_levels() {
        println!("\n{:?}", node)
    }

    for node in quadtree.iter_nodes() {
        println!("\n{:?}", node)
    }

    let entities = quadtree.clear();

    for node in quadtree.iter_levels() {
        println!("\n{:?}", node)
    }

    for node in quadtree.iter_nodes() {
        println!("\n{:?}", node)
    }

    Ok(())
}

