use crate::partition::{SpatialInsertion, SpatialQuery};

pub struct InterestManager<S>
where
    S: SpatialInsertion + SpatialQuery,
{
    state: S,
}
