use std::collections::HashMap;

use crate::{codec::Base4Int, ensure, Bounds2D, EntityID, IsEntity, SpatialError, Vector2D};

const MAX_QUADS: usize = 4;
type Quadrant = Box<QuadTreeNode>;
type Entities<'a, T> = Vec<&'a T>;
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
    pub fn new(
        boundary_min: Vector2D,
        boundary_max: Vector2D,
        capacity: usize,
    ) -> Result<Self, SpatialError> {
        ensure!(capacity > 0, SpatialError::InvalidCapacity);
        ensure!(boundary_min < boundary_max, SpatialError::InvalidBounds);

        Ok(Self {
            root: QuadTreeNode::new(boundary_min, boundary_max, capacity, 0),
            map: EntityMap::new_with_capacity(capacity),
        })
    }

    pub fn insert(&mut self, entity: E) -> Result<bool, SpatialError> {
        ensure!(
            self.root.boundary.contains(&entity.bounding_box()),
            SpatialError::OutOfBounds
        );

        let id = entity.id();

        self.map.insert_entity(id, entity, LeafPath::new());

        Ok(self.root.insert(id, &mut self.map, 0))
    }

    pub fn levels(&self) -> LevelCount {
        self.root.max_subtree_depth() + 1
    }

    pub fn iter_levels(&self) -> Levels<'_> {
        Levels::new(&self.root)
    }

    pub fn iter_nodes(&self) -> Nodes<'_> {
        Nodes::new(&self.root)
    }

    pub fn clear(&mut self) -> Vec<E> {
        let out = self.map.drain();
        self.root.drain();

        let boundary = self.root.boundary;
        let capacity = self.root.capacity;
        self.root = QuadTreeNode::new(boundary.min, boundary.max, capacity, 0);

        out
    }
}

impl<Entity: IsEntity> QuadTree<Entity> {
    pub fn query_bounds(
        &self,
        bounds_center: Vector2D,
        bounds_size: Vector2D,
    ) -> Option<Entities<Entity>> {
        let search_bounds = Bounds2D::from_center_size(bounds_center, bounds_size);
        let mut collector = vec![];
        self.root
            .query_within_bounds(&search_bounds, &self.map, &mut collector, &|_, _| true);

        if collector.is_empty() {
            return None;
        }
        Some(collector)
    }

    pub fn query_point(&self, point: Vector2D) -> Option<Entities<Entity>> {
        let mut collector = vec![];
        self.root
            .query_with_point(point, &self.map, &mut collector, &|_, _| true);

        if collector.is_empty() {
            return None;
        }
        Some(collector)
    }

    pub fn query_bounds_and_filter<F>(
        &self,
        bounds_center: Vector2D,
        bounds_size: Vector2D,
        predicate: F,
    ) -> Option<Entities<Entity>>
    where
        F: Fn(&Bounds2D, &Entity) -> bool,
    {
        let search_bounds = Bounds2D::from_center_size(bounds_center, bounds_size);
        let mut collector = vec![];
        self.root
            .query_within_bounds(&search_bounds, &self.map, &mut collector, &predicate);

        if collector.is_empty() {
            return None;
        }
        Some(collector)
    }

    pub fn query_point_and_filter<F>(
        &self,
        point: Vector2D,
        predicate: F,
    ) -> Option<Entities<Entity>>
    where
        F: Fn(Vector2D, &Entity) -> bool,
    {
        let mut collector = vec![];
        self.root
            .query_with_point(point, &self.map, &mut collector, &predicate);

        if collector.is_empty() {
            return None;
        }
        Some(collector)
    }
}

pub struct QuadTreeNode {
    depth: usize,
    capacity: usize,
    items: Vec<EntityID>,
    boundary: Bounds2D,
    quadrants: Option<[Quadrant; MAX_QUADS]>,
}

impl QuadTreeNode {
    fn new(min: Vector2D, max: Vector2D, capacity: usize, depth: usize) -> Self {
        Self {
            boundary: Bounds2D::new(min, max),
            items: Vec::with_capacity(capacity),
            capacity,
            depth,
            quadrants: None,
        }
    }

    fn with_bounds(boundary: Bounds2D, capacity: usize, depth: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            boundary,
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

    fn query_with_point<'t, E, F>(
        &'t self,
        point: Vector2D,
        map: &'t EntityMap<E>,
        collector: &mut Vec<&'t E>,
        predicate: &F,
    ) where
        F: Fn(Vector2D, &E) -> bool,
        E: IsEntity,
    {
        if !self.boundary.contains_point(point) {
            return;
        }

        for id in self.items.iter() {
            let entity = map.get_entity(id);
            if entity.bounding_box().contains_point(point) {
                if predicate(point, entity) {
                    collector.push(entity);
                }
            }
        }

        if !self.is_leaf() {
            let quadrants = self.quadrants.as_ref().unwrap();
            for quad in quadrants.iter() {
                quad.query_with_point(point, map, collector, predicate);
            }
        }
    }

    fn query_within_bounds<'t, E, F>(
        &'t self,
        bounds: &Bounds2D,
        map: &'t EntityMap<E>,
        collector: &mut Vec<&'t E>,
        predicate: &F,
    ) where
        F: Fn(&Bounds2D, &E) -> bool,
        E: IsEntity,
    {
        if !self.boundary.intersects(&bounds) {
            return;
        }

        for id in self.items.iter() {
            let entity = map.get_entity(id);
            if entity.bounding_box().intersects(&bounds) {
                if predicate(bounds, entity) {
                    collector.push(entity);
                }
            }
        }

        if !self.is_leaf() {
            let quadrants = self.quadrants.as_ref().unwrap();
            for quad in quadrants.iter() {
                quad.query_within_bounds(bounds, map, collector, predicate);
            }
        }
    }

    fn insert<E>(&mut self, id: EntityID, map: &mut EntityMap<E>, nidx: NodeIndex) -> bool
    where
        E: IsEntity,
    {
        let (entity, ref mut path) = map.get_entity_path_mut(id).unwrap();

        if !self.boundary.contains(&entity.bounding_box()) {
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
        let center = (self.boundary.min + self.boundary.max).div(2.);
        let (min, max) = (self.boundary.max, self.boundary.min);

        let ne = Bounds2D::new(center, self.boundary.max);
        let nw = Bounds2D::new((min.x(), center.y()).into(), (center.x(), max.y()).into());
        let se = Bounds2D::new((center.x(), min.y()).into(), (max.x(), center.y()).into());
        let sw = Bounds2D::new(self.boundary.min, center);

        let new_depth = self.depth + 1;

        self.quadrants = Some([
            Box::new(QuadTreeNode::with_bounds(ne, self.capacity, new_depth)),
            Box::new(QuadTreeNode::with_bounds(nw, self.capacity, new_depth)),
            Box::new(QuadTreeNode::with_bounds(se, self.capacity, new_depth)),
            Box::new(QuadTreeNode::with_bounds(sw, self.capacity, new_depth)),
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
pub struct NodeInfo<'a>(LevelCount, &'a Bounds2D, &'a [EntityID]);

impl<'a> From<&'a QuadTreeNode> for NodeInfo<'a> {
    fn from(value: &'a QuadTreeNode) -> Self {
        Self(value.depth, &value.boundary, &value.items)
    }
}

impl<'a> NodeInfo<'a> {
    pub fn node_level(&self) -> LevelCount {
        self.0
    }

    pub fn bounding_box(&self) -> &Bounds2D {
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
