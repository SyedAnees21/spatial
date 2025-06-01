use crate::{Bounds2D, IsEntity, SpatialError, Vector2D};

const MAX_QUADS: usize = 4;
type Quadrant<T> = Box<QuadTreeNode<T>>;
type Entities<'a, T> = Vec<&'a T>;
type LevelCount = usize;

pub struct QuadTree<Entity>
where
    Entity: IsEntity + Clone,
{
    root: QuadTreeNode<Entity>,
    capacity: usize,
}

impl<Entity: IsEntity + Clone> QuadTree<Entity> {
    pub fn new(
        boundary_min: Vector2D,
        boundary_max: Vector2D,
        capacity: usize,
    ) -> Result<Self, SpatialError> {
        if boundary_min >= boundary_max {
            return Err(SpatialError::InvalidBounds);
        }

        if capacity <= 0 {
            return Err(SpatialError::InvalidCapacity);
        }

        Ok(Self {
            root: QuadTreeNode::new(boundary_min, boundary_max, capacity, 0),
            capacity,
        })
    }

    pub fn insert(&mut self, entity: Entity) -> Result<bool, SpatialError> {
        if !self.root.boundary.contains(&entity.bounding_box()) {
            return Err(SpatialError::OutOfBounds);
        }
        Ok(self.root.insert(&entity))
    }

    pub fn levels(&self) -> LevelCount {
        self.root.max_subtree_depth() + 1
    }

    pub fn iter_levels<'tree>(&'tree self) -> Levels<'tree, Entity> {
        Levels::new(&self.root)
    }

    pub fn iter_nodes<'tree>(&'tree self) -> Nodes<'tree, Entity> {
        Nodes::new(&self.root)
    }

    pub fn clear(&mut self) -> Vec<Entity> {
        let mut return_container = vec![];
        self.root.drain(&mut return_container);

        let boundary = self.root.boundary;
        self.root = QuadTreeNode::new(boundary.min, boundary.max, self.capacity, 0);

        return_container
    }
}

impl<Entity: IsEntity + Clone> QuadTree<Entity> {
    pub fn query_bounds(
        &self,
        bounds_center: Vector2D,
        bounds_size: Vector2D,
    ) -> Option<Entities<Entity>> {
        let search_bounds = Bounds2D::from_center_size(bounds_center, bounds_size);
        let mut collector = vec![];
        self.root
            .query_within_bounds(&search_bounds, &mut collector, &|_, _| true);

        if collector.is_empty() {
            return None;
        }
        Some(collector)
    }

    pub fn query_point(&self, point: Vector2D) -> Option<Entities<Entity>> {
        let mut collector = vec![];
        self.root
            .query_with_point(point, &mut collector, &|_, _| true);

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
            .query_within_bounds(&search_bounds, &mut collector, &predicate);

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
            .query_with_point(point, &mut collector, &predicate);

        if collector.is_empty() {
            return None;
        }
        Some(collector)
    }
}

pub struct QuadTreeNode<Entity>
where
    Entity: IsEntity + Clone,
{
    depth: usize,
    capacity: usize,
    items: Vec<Entity>,
    boundary: Bounds2D,
    quadrants: Option<[Quadrant<Entity>; MAX_QUADS]>,
}

impl<Entity> QuadTreeNode<Entity>
where
    Entity: IsEntity + Clone,
{
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

    fn query_with_point<'tree, F>(
        &'tree self,
        point: Vector2D,
        collector: &mut Entities<'tree, Entity>,
        predicate: &F,
    ) where
        F: Fn(Vector2D, &Entity) -> bool,
    {
        if !self.boundary.contains_point(point) {
            return;
        }

        for entity in self.items.iter() {
            if entity.bounding_box().contains_point(point) {
                if predicate(point, entity) {
                    collector.push(entity);
                }
            }
        }

        if !self.is_leaf() {
            let quadrants = self.quadrants.as_ref().unwrap();
            for quad in quadrants.iter() {
                quad.query_with_point(point, collector, predicate);
            }
        }
    }

    fn query_within_bounds<'tree, F>(
        &'tree self,
        bounds: &Bounds2D,
        collector: &mut Entities<'tree, Entity>,
        predicate: &F,
    ) where
        F: Fn(&Bounds2D, &Entity) -> bool,
    {
        if !self.boundary.intersects(&bounds) {
            return;
        }

        for entity in self.items.iter() {
            if entity.bounding_box().intersects(&bounds) {
                if predicate(bounds, entity) {
                    collector.push(entity);
                }
            }
        }

        if !self.is_leaf() {
            let quadrants = self.quadrants.as_ref().unwrap();
            for quad in quadrants.iter() {
                quad.query_within_bounds(bounds, collector, predicate);
            }
        }
    }

    fn insert(&mut self, entity: &Entity) -> bool {
        if !self.boundary.contains(&entity.bounding_box()) {
            return false;
        }

        if self.items.len() < self.capacity {
            self.items.push(entity.to_owned());
            return true;
        }

        if self.is_leaf() {
            self.subdivide();
        }

        let all_entities = std::mem::take(&mut self.items);

        if let Some(ref mut quads) = self.quadrants {
            for entity in all_entities.iter() {
                if !quads.iter_mut().any(|quad| quad.insert(entity)) {
                    self.items.push(entity.clone());
                }
            }

            if !quads.iter_mut().any(|quad| quad.insert(entity)) {
                self.items.push(entity.to_owned());
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

    fn drain(&mut self, out: &mut Vec<Entity>) {
        out.extend(self.items.drain(..));

        if let Some(ref mut quadrants) = self.quadrants {
            for quad in quadrants.iter_mut() {
                quad.drain(out);
            }
        }
    }
}

pub struct Levels<'a, Entity>
where
    Entity: IsEntity + Clone,
{
    root: &'a QuadTreeNode<Entity>,
    current_level: LevelCount,
    max_depth: usize,
}

impl<'a, Entity> Levels<'a, Entity>
where
    Entity: IsEntity + Clone,
{
    fn new(root: &'a QuadTreeNode<Entity>) -> Self {
        let max_depth = root.max_subtree_depth();

        Self {
            root,
            current_level: 0,
            max_depth,
        }
    }

    fn gather_at_depth(
        &self,
        tree_node: &'a QuadTreeNode<Entity>,
        container: &mut Vec<&'a Entity>,
    ) {
        if tree_node.depth == self.current_level {
            for entity in tree_node.items.iter() {
                container.push(entity);
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

impl<'a, Entity> Iterator for Levels<'a, Entity>
where
    Entity: IsEntity + Clone,
{
    type Item = Entities<'a, Entity>;

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
pub struct NodeInfo<'a, Entity>(LevelCount, &'a Bounds2D, &'a [Entity]);

impl<'a, Entity> From<&'a QuadTreeNode<Entity>> for NodeInfo<'a, Entity>
where
    Entity: IsEntity + Clone,
{
    fn from(value: &'a QuadTreeNode<Entity>) -> Self {
        Self(value.depth, &value.boundary, &value.items)
    }
}

impl<'a, Entity: IsEntity + Clone> NodeInfo<'a, Entity> {
    pub fn node_level(&self) -> LevelCount {
        self.0
    }

    pub fn bounding_box(&self) -> &Bounds2D {
        self.1
    }

    pub fn entities(&self) -> &[Entity] {
        self.2
    }
}

pub struct Nodes<'a, Entity>
where
    Entity: IsEntity + Clone,
{
    stack: Vec<&'a QuadTreeNode<Entity>>,
}

impl<'a, Entity: IsEntity + Clone> Nodes<'a, Entity> {
    fn new(root_node: &'a QuadTreeNode<Entity>) -> Self {
        Self {
            stack: vec![root_node],
        }
    }
}

impl<'a, Entity: IsEntity + Clone> Iterator for Nodes<'a, Entity> {
    type Item = NodeInfo<'a, Entity>;
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
