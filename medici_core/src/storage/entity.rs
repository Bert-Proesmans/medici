//! Module containing structures for storing entities.

use function::{Entity, IndexedStorageCompliance};

#[derive(Debug, Clone)]
/// Structure wrapping a [`Vec`] to provide a container for (all) entities
/// within the state machine.
pub struct EntityStorage<E>
where
    E: Entity + Clone,
    E::ID: Into<usize> + From<usize>,
{
    entities: Vec<E>,
}

impl<E> EntityStorage<E>
where
    E: Entity + Clone,
    E::ID: Into<usize> + From<usize>,
{
    /// Creates a new object for storage.
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    /// Stores the provided item into this object.
    pub fn push(&mut self, item: E) {
        self.entities.push(item)
    }
}

impl<E> IndexedStorageCompliance for EntityStorage<E>
where
    E: Entity + Clone,
    E::ID: Into<usize> + From<usize>,
{
    type Item = E;

    fn as_slice(&self) -> &[Self::Item] {
        &self.entities
    }

    fn as_slice_mut(&mut self) -> &mut [Self::Item] {
        &mut self.entities
    }
}
