#![cfg(test)]

use num_traits::identities;

use crate::{
    quad::QuadTree, Geometry, IsEntity, SpatialError,
};

mod boundary;
mod grid;
mod base4;
mod geometry;
mod quadtree;
