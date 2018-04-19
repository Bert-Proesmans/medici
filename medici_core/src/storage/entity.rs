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
    <E as Entity>::ID: Into<usize> + From<usize> + Copy,
{
    entities: Vec<E>,
    maximum_items: usize,
}

impl<E> Service for EntityStorage<E>
where
    E: Entity + EntityBuilder<E> + Clone,
    <E as Entity>::ID: Into<usize> + From<usize> + Copy,
{
}

impl<E> EntityStorage<E>
where
    E: Entity + EntityBuilder<E> + Clone,
    <E as Entity>::ID: Into<usize> + From<usize> + Debug + Display + Copy,
{
    /// Creates a new object for storage.
    pub fn new(maximum_items: usize) -> Self {
        Self {
            entities: vec![],
            maximum_items,
        }
    }

    /// Build a new entity, stored within this object.
    ///
    /// A mutable borrow to the newly created entity is returned.
    pub fn new_entity(&mut self) -> Result<&mut E, OverflowError> {
        let next_eid = self.entities.len();
        if next_eid >= self.maximum_items {
            return Err(OverflowError(self.maximum_items));
        }

        // TODO; Proper error handling.
        let new_entity = E::new_with_id(next_eid).unwrap();
        self.entities.push(new_entity);
        Ok(self.entities.last_mut().unwrap())
    }

    /// Retrieve a borrow of the requested entity, if present.
    pub fn get_entity(&self, id: E::ID) -> Result<&E, MissingEntityError<E::ID>> {
        let idx_id = id.into();
        self.entities.get(idx_id).ok_or(MissingEntityError(id))
    }

    /// Retrieces a mutable borrow of the requested entity, if present.
    pub fn get_entity_mut(&mut self, id: E::ID) -> Result<&mut E, MissingEntityError<E::ID>> {
        let idx_id = id.into();
        self.entities.get_mut(idx_id).ok_or(MissingEntityError(id))
    }
}
