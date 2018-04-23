//! Contains functionality to work with [`Entity`]s.

use std::fmt::{Debug, Display};

use error::custom_type::{MissingEntityError, OverflowError};
use function::{Entity, EntityBuilder, IndexedStorageCompliance};
use marker;
use storage::EntityStorage;

#[derive(Debug, Clone)]
/// Structure for working with [`Entity`] objects.
pub struct EntityService<E>
where
    E: Entity + EntityBuilder<E> + Clone,
    E::ID: Into<usize> + From<usize> + Debug + Display,
{
    storage: EntityStorage<E>,
    maximum_items: usize,
}

impl<E> marker::Service for EntityService<E>
where
    E: Entity + EntityBuilder<E> + Clone,
    E::ID: Into<usize> + From<usize> + Debug + Display,
{
}

impl<E> EntityService<E>
where
    E: Entity + EntityBuilder<E> + Clone,
    E::ID: Into<usize> + From<usize> + Debug + Display,
{
    /// Creates a new object for storage.
    pub fn new<M: Into<usize>>(maximum_items: M) -> Self {
        Self {
            storage: EntityStorage::new(),
            maximum_items: maximum_items.into(),
        }
    }

    /// Build a new entity which is kept inside this storage object.
    ///
    /// # Returns
    /// A mutable reference to the new entity is returned upon successful creation.
    pub fn new_entity(&mut self) -> Result<&mut E, OverflowError> {
        let next_eid = self.storage.as_slice().len();
        if next_eid >= self.maximum_items {
            return Err(OverflowError(self.maximum_items));
        }

        let new_entity = E::new_with_id(next_eid.into());
        self.storage.push(new_entity);
        Ok(self.storage.as_slice_mut().last_mut().unwrap())
    }

    /// Retrieves a reference to the entity matching the id.
    pub fn get(&self, id: E::ID) -> Result<&E, MissingEntityError<E::ID>> {
        let idx_id = id.into();
        self.storage
            .as_slice()
            .get(idx_id)
            .ok_or(MissingEntityError(id))
    }

    /// Retrieves a mutable reference to the entity matching the id.
    pub fn get_mut(&mut self, id: E::ID) -> Result<&mut E, MissingEntityError<E::ID>> {
        let idx_id = id.into();
        self.storage
            .as_slice_mut()
            .get_mut(idx_id)
            .ok_or(MissingEntityError(id))
    }
}
