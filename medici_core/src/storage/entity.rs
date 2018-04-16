//! Module containing structures for storing entities.

use std::convert::TryFrom;
use std::fmt::{Debug, Display};

use function::{Entity, EntityBuilder, Service};
use service::error::{MissingEntityError, OverflowError};

#[derive(Debug, Clone)]
/// Structure wrapping a [`Vec`] to provide a container for (all) entities
/// within the state machine.
pub struct EntityStorage<E>
where
    E: Entity + EntityBuilder<E> + Clone,
    E::ID: Into<usize> + TryFrom<usize> + Copy,
{
    entities: Vec<E>,
    maximum_items: usize,
}

impl<E> Service for EntityStorage<E>
where
    E: Entity + EntityBuilder<E> + Clone,
    E::ID: Into<usize> + TryFrom<usize> + Copy,
{
}

impl<E> EntityStorage<E>
where
    E: Entity + EntityBuilder<E> + Clone,
    E::ID: Into<usize> + TryFrom<usize> + Debug + Display + Copy,
{
    /// Creates a new object for storage.
    pub fn new(maximum_items: usize) -> Self {
        Self {
            entities: vec![],
            maximum_items,
        }
    }

    /// Build a new entity which is kept inside this storage object.
    ///
    /// # Returns
    /// A mutable reference to the new entity is returned upon successful creation.
    pub fn new_entity(&mut self) -> Result<&mut E, OverflowError> {
        let next_eid = self.entities.len();
        if next_eid >= self.maximum_items {
            return Err(OverflowError(self.maximum_items));
        }

        let next_eid = E::ID::try_from(next_eid).map_err(|_| OverflowError(self.maximum_items))?;
        let new_entity = E::new_with_id(next_eid);
        self.entities.push(new_entity);
        Ok(self.entities.last_mut().unwrap())
    }

    /// Retrieves a reference to the entity matching the id.
    pub fn get(&self, id: E::ID) -> Result<&E, MissingEntityError<E::ID>> {
        let idx_id = id.into();
        self.entities.get(idx_id).ok_or(MissingEntityError(id))
    }

    /// Retrieves a mutable reference to the entity matching the id.
    pub fn get_mut(&mut self, id: E::ID) -> Result<&mut E, MissingEntityError<E::ID>> {
        let idx_id = id.into();
        self.entities.get_mut(idx_id).ok_or(MissingEntityError(id))
    }
}
