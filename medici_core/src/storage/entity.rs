//! Module containing structures for storing entities.

use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use function::{Entity, EntityBuilder, Service, ZoneEnumerator};
use service::error::{MissingEntityError, OverflowError};
use storage::ZoneEntityStorage;

#[derive(Debug, Clone)]
/// Structure wrapping a [`Vec`] to provide a container for (all) entities
/// within the state machine.
pub struct EntityStorage<E, EZ>
where
    E: Entity + EntityBuilder<E> + Clone,
    <E as Entity>::ID: Into<usize> + From<usize> + Copy + Display + Debug + PartialEq + Eq + Hash,
    EZ: ZoneEnumerator + Clone + Default + Debug + PartialEq + Eq + Hash,
{
    maximum_items: usize,
    entities: Vec<E>,
    zones: ZoneEntityStorage<E, EZ>,
}

impl<E, EZ> Service for EntityStorage<E, EZ>
where
    E: Entity + EntityBuilder<E> + Clone,
    <E as Entity>::ID: Into<usize> + From<usize> + Copy + Display + Debug + PartialEq + Eq + Hash,
    EZ: ZoneEnumerator + Clone + Default + Debug + PartialEq + Eq + Hash,
{
}

impl<E, EZ> EntityStorage<E, EZ>
where
    E: Entity + EntityBuilder<E> + Clone,
    <E as Entity>::ID: Into<usize> + From<usize> + Copy + Display + Debug + PartialEq + Eq + Hash,
    EZ: ZoneEnumerator + Clone + Default + Debug + PartialEq + Eq + Hash,
{
    /// Creates a new object for storage.
    pub fn new(maximum_items: usize) -> Self {
        Self {
            entities: vec![],
            maximum_items,
            zones: ZoneEntityStorage::<E, EZ>::new(),
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

/*
 * Implementing Deref allows the compiler to invisibly coerce the EntityStorage into a
 * ZoneEntityStorage. This is allowed because the ZoneEntityStorage provides a disjunct
 * interface from EntityStorage.
 */

impl<E, EZ> Deref for EntityStorage<E, EZ>
where
    E: Entity + EntityBuilder<E> + Clone,
    <E as Entity>::ID: Into<usize> + From<usize> + Copy + Display + Debug + PartialEq + Eq + Hash,
    EZ: ZoneEnumerator + Clone + Default + Debug + PartialEq + Eq + Hash,
{
    type Target = ZoneEntityStorage<E, EZ>;

    fn deref(&self) -> &Self::Target {
        &self.zones
    }
}

impl<E, EZ> DerefMut for EntityStorage<E, EZ>
where
    E: Entity + EntityBuilder<E> + Clone,
    <E as Entity>::ID: Into<usize> + From<usize> + Copy + Display + Debug + PartialEq + Eq + Hash,
    EZ: ZoneEnumerator + Clone + Default + Debug + PartialEq + Eq + Hash,
{
    // Type is taken from Deref, because Deref must be implemented before
    // DerefMut is implemented!

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zones
    }
}
