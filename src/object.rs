use crate::types::{Point, D2};

pub type EntityId = u64;

pub trait IsEntity<const D: usize> {
    fn id(&self) -> EntityId;
    fn position(&self) -> Point<D>;
}

pub trait IsEntity2D: IsEntity<D2> {}

pub trait Intersectable {
    fn intersects(&self) -> bool;
}

pub trait HasBounds {
    type B: Intersectable;

    fn bounds(&self) -> Self::B;
}


#[cfg(test)]
mod tests {
    use crate::types::Point2D;

    use super::IsEntity;

    struct Object {
        id: u8,
        position: (f32, f32),
    }

    impl IsEntity<2> for Object {
        fn id(&self) -> super::EntityId {
            self.id as super::EntityId
        }

        fn position(&self) -> super::Point<2> {
            let (x, y) = self.position;
            Point2D::from_xy(x, y)
        }
    }

    #[test]
    fn validate() {}
}
