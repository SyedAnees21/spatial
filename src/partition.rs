use crate::relevance::Relevance;

pub trait SpatialInsertion {
    /// The type of objects stored in the spatial partitioning structure.
    type Object;

    /// Inserts an item into the spatial partitioning structure.
    /// Returns true if the insertion was successful.
    fn insert(&mut self, object: Self::Object) -> bool;

    /// Inserts multiple items into the spatial partitioning structure.
    /// Returns the number of successfully inserted items.
    fn insert_many<I>(&mut self, objects: I) -> usize 
    where
        I: IntoIterator<Item = Self::Object>
    {
        let mut count = 0;
        for object in objects {
            if self.insert(object) {
                count += 1;
            }
        }
        count
    }
}

pub trait SpatialQuery {
    /// The type of objects stored in the spatial partitioning structure.
    type Objects;
    /// The type of query used to search the spatial partitioning structure.
    type Query;

    /// Queries the spatial partitioning structure for objects that intersect with the given query.
    /// Returns an iterator over references to matching objects.
    fn query(&self, query: Self::Query, relevance: Relevance) -> impl Iterator<Item = &Self::Objects>;

    /// Mutably queries the spatial partitioning structure for objects that intersect with the given query.
    /// Returns an iterator over mutable references to matching objects.
    // fn query_mut(&mut self, query: Self::Query, relevance: Relevance) -> impl Iterator<Item = &mut Self::Objects>;

    /// Checks if any object in the structure intersects with the given query.
    fn contains(&self, query: Self::Query, relevance: Relevance) -> bool;
}
