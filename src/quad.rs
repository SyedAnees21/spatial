use core::panic;
use std::collections::HashMap;

use crate::{codec::Base4Int, ensure, EntityID, Geometry, IsEntity, SpatialError};

const MAX_QUADS: usize = 4;
type Quadrant = Box<QuadTreeNode>;
type LevelCount = usize;
type LeafPath = Base4Int;
type NodeIndex = u8;

pub struct EntityMap<E: IsEntity>(HashMap<EntityID, (E, LeafPath)>);

impl<E: IsEntity> EntityMap<E> {
    pub fn new_with_capacity(cap: usize) -> Self {
        Self(HashMap::with_capacity(cap))
    }

    pub fn insert_entity(&mut self, id: EntityID, entity: E, path: LeafPath) {
        let _ = self.0.insert(id, (entity, path));
    }

    pub fn get_entity(&self, id: &EntityID) -> &E {
        &self.0.get(id).unwrap().0
    }

    pub fn get_path(&self, id: EntityID) -> Option<Vec<NodeIndex>> {
        if let Some((_, p)) = self.0.get(&id) {
            return Some(p.peek_all());
        }
        None
    }

    pub fn get_entity_path_mut(&mut self, id: EntityID) -> Option<&mut (E, LeafPath)> {
        self.0.get_mut(&id)
    }

    pub fn drain(&mut self) -> Vec<E> {
        self.0.drain().map(|(_, (e, _))| e).collect()
    }
}

pub struct QuadTree<E>
where
    E: IsEntity,
{
    map: EntityMap<E>,
    root: QuadTreeNode,
}

impl<E: IsEntity> QuadTree<E> {
    pub fn new<T>(
        boundary_min: (T, T),
        boundary_max: (T, T),
        capacity: usize,
    ) -> Result<Self, SpatialError>
    where
        T: Into<f64> + Copy + PartialEq + PartialOrd,
    {
        ensure!(capacity > 0, SpatialError::InvalidCapacity);
        ensure!(boundary_min < boundary_max, SpatialError::InvalidBounds);

        Ok(Self {
            root: QuadTreeNode::new(boundary_min, boundary_max, capacity, 0),
            map: EntityMap::new_with_capacity(capacity),
        })
    }

    pub fn insert(&mut self, entity: E) -> Result<bool, SpatialError> {
        ensure!(
            self.root.boundary.contains(entity.bounds()),
            SpatialError::OutOfBounds
        );

        let id = entity.id();
        self.map.insert_entity(id, entity, LeafPath::new());

        Ok(self.root.insert(id, &mut self.map, 0))
    }

    pub fn levels(&self) -> LevelCount {
        self.root.max_subtree_depth() + 1
    }

    pub fn iterate_levels(&self) -> Levels<'_> {
        Levels::new(&self.root)
    }

    pub fn iterate_nodes(&self) -> Nodes<'_> {
        Nodes::new(&self.root)
    }

    pub fn clear(&mut self) -> Vec<E> {
        let out = self.map.drain();
        self.root.drain();

        let boundary = self.root.boundary;
        let capacity = self.root.capacity;
        self.root = QuadTreeNode::with_geometry(boundary, capacity, 0);

        out
    }

    pub fn entity_path_in_tree(&self, id: EntityID) -> Option<Vec<u8>> {
        self.map.get_path(id)
    }

    pub fn query(&self, query_type: Geometry) -> Entities<'_, E> {
        let mut collector = vec![];
        self.root
            .inner_query(query_type, &self.map, &mut collector, &|_, _| true);

        Entities::new(&self.map, collector)
    }

    pub fn query_and_filter<F>(&self, query_type: Geometry, predicate: F) -> Entities<'_, E>
    where
        F: Fn(Geometry, &E) -> bool,
    {
        let mut collector = vec![];
        self.root
            .inner_query(query_type, &self.map, &mut collector, &predicate);

        Entities::new(&self.map, collector)
    }
}

pub struct QuadTreeNode {
    depth: usize,
    capacity: usize,
    items: Vec<EntityID>,
    boundary: Geometry,
    quadrants: Option<[Quadrant; MAX_QUADS]>,
}

impl QuadTreeNode {
    fn new<T>(min: (T, T), max: (T, T), capacity: usize, depth: usize) -> Self
    where
        T: Into<f64> + Copy,
    {
        Self {
            boundary: Geometry::rect_from_min_max(min, max),
            items: Vec::with_capacity(capacity),
            capacity,
            depth,
            quadrants: None,
        }
    }

    fn with_geometry(geometry: Geometry, capacity: usize, depth: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            boundary: geometry,
            capacity,
            depth,
            quadrants: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.quadrants.is_none()
    }

    fn max_subtree_depth(&self) -> usize {
        if let Some(ref quadrants) = self.quadrants {
            return quadrants
                .iter()
                .map(|quad| quad.max_subtree_depth())
                .max()
                .unwrap_or(self.depth);
        }
        self.depth
    }

    fn inner_query<'t, E, F>(
        &self,
        query_type: Geometry,
        map: &'t EntityMap<E>,
        collector: &mut Vec<EntityID>,
        // collector: &mut Vec<&'t E>,
        predicate: &F,
    ) where
        F: Fn(Geometry, &E) -> bool,
        E: IsEntity,
    {
        let contains_or_intersects = match query_type {
            Geometry::Point { x: _, y: _ } => self.boundary.contains(query_type),
            _ => self.boundary.intersects(query_type),
        };

        if contains_or_intersects {
            for id in self.items.iter() {
                let entity = map.get_entity(id);

                let contains_or_intersects = match query_type {
                    Geometry::Point { x: _, y: _ } => entity.contains_geometry(query_type),
                    _ => entity.intersects_geometry(query_type),
                };

                if contains_or_intersects && predicate(query_type, entity) {
                    collector.push(*id);
                }
            }
        }

        if let Some(ref quadrants) = self.quadrants {
            for quad in quadrants {
                quad.inner_query(query_type, map, collector, predicate);
            }
        }
    }

    fn insert<E>(&mut self, id: EntityID, map: &mut EntityMap<E>, nidx: NodeIndex) -> bool
    where
        E: IsEntity,
    {
        let (entity, ref mut path) = map.get_entity_path_mut(id).unwrap();

        if !self.boundary.contains(entity.bounds()) {
            return false;
        }

        path.push(nidx);

        if self.items.len() < self.capacity {
            self.items.push(id);
            return true;
        }

        if self.is_leaf() {
            self.subdivide();
        }

        let _ = entity;
        let _ = path;

        let all_ids = std::mem::take(&mut self.items);

        if let Some(ref mut quads) = self.quadrants {
            for id in all_ids.iter() {
                if !quads
                    .iter_mut()
                    .enumerate()
                    .any(|(nidx, quad)| quad.insert(*id, map, nidx as NodeIndex))
                {
                    self.items.push(*id);
                }
            }

            if !quads
                .iter_mut()
                .enumerate()
                .any(|(nidx, quad)| quad.insert(id, map, nidx as NodeIndex))
            {
                self.items.push(id);
            }
        }

        true
    }

    fn subdivide(&mut self) {
        let Geometry::Rect { center, size } = self.boundary else {
            panic!("Expected boundary geometry to be Rect")
        };

        let (min, max) = Geometry::rect_min_max(center, size);

        let ne = Geometry::rect_from_min_max(center, max);
        let nw = Geometry::rect_from_min_max((min.0, center.1), (center.0, max.1));
        let se = Geometry::rect_from_min_max((center.0, min.1), (max.0, center.1));
        let sw = Geometry::rect_from_min_max(min, center);

        let new_depth = self.depth + 1;

        self.quadrants = Some([
            Box::new(QuadTreeNode::with_geometry(ne, self.capacity, new_depth)),
            Box::new(QuadTreeNode::with_geometry(nw, self.capacity, new_depth)),
            Box::new(QuadTreeNode::with_geometry(se, self.capacity, new_depth)),
            Box::new(QuadTreeNode::with_geometry(sw, self.capacity, new_depth)),
        ])
    }

    fn drain(&mut self) {
        self.items.drain(..);

        if let Some(ref mut quadrants) = self.quadrants {
            for quad in quadrants.iter_mut() {
                quad.drain();
            }
        }
    }
}

pub struct Entities<'t, E>
where
    E: IsEntity,
{
    map: &'t EntityMap<E>,
    ids: Vec<EntityID>,
}

impl<'t, E> Entities<'t, E>
where
    E: IsEntity,
{
    fn new(map: &'t EntityMap<E>, ids: Vec<EntityID>) -> Self {
        Self { map, ids }
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}

impl<'t, E: IsEntity> Iterator for Entities<'t, E> {
    type Item = &'t E;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.ids.pop() {
            return Some(self.map.get_entity(&id));
        }
        None
    }
}

pub struct Levels<'a> {
    root: &'a QuadTreeNode,
    current_level: LevelCount,
    max_depth: usize,
}

impl<'a> Levels<'a> {
    fn new(root: &'a QuadTreeNode) -> Self {
        let max_depth = root.max_subtree_depth();

        Self {
            root,
            current_level: 0,
            max_depth,
        }
    }

    fn gather_at_depth(&self, tree_node: &'a QuadTreeNode, container: &mut Vec<EntityID>) {
        if tree_node.depth == self.current_level {
            for entity in tree_node.items.iter() {
                container.push(*entity);
            }
            return;
        }

        if tree_node.depth < self.current_level {
            if let Some(ref quadrants) = tree_node.quadrants {
                for quad in quadrants.iter() {
                    self.gather_at_depth(quad, container);
                }
            }
        }
    }
}

impl<'a> Iterator for Levels<'a> {
    type Item = Vec<EntityID>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_level > self.max_depth {
            return None;
        }

        let mut container = vec![];
        self.gather_at_depth(self.root, &mut container);

        self.current_level += 1;
        Some(container)
    }
}

#[derive(Debug)]
pub struct NodeInfo<'a>(LevelCount, &'a Geometry, &'a [EntityID]);

impl<'a> From<&'a QuadTreeNode> for NodeInfo<'a> {
    fn from(value: &'a QuadTreeNode) -> Self {
        Self(value.depth, &value.boundary, &value.items)
    }
}

impl<'a> NodeInfo<'a> {
    pub fn node_level(&self) -> LevelCount {
        self.0
    }

    pub fn bounding_box(&self) -> &Geometry {
        self.1
    }

    pub fn entities(&self) -> &[EntityID] {
        self.2
    }
}

pub struct Nodes<'a> {
    stack: Vec<&'a QuadTreeNode>,
}

impl<'a> Nodes<'a> {
    fn new(root_node: &'a QuadTreeNode) -> Self {
        Self {
            stack: vec![root_node],
        }
    }
}

impl<'a> Iterator for Nodes<'a> {
    type Item = NodeInfo<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(ref quadrants) = node.quadrants {
                for quad in quadrants.iter() {
                    self.stack.push(quad);
                }
            }
            return Some(node.into());
        }
        None
    }
}
