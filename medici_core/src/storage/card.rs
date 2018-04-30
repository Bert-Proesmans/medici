//! Module containing structures for storing game card objects.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use error::custom_type::IDCollisionError;
use function::{Card, Identifiable, IndexedStorageCompliance};
use marker;

#[derive(Debug, Clone)]
/// Structure holding onto all cards defined for a specific machine.
pub struct CardStorage<C>
where
    C: Card,
    <C as Identifiable>::ID: Debug + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Copy,
{
    /// Contains the cards.
    pub cards: HashMap<C::ID, C>,
}

impl<C> IndexedStorageCompliance for CardStorage<C>
where
    C: Card,
    <C as Identifiable>::ID: Debug + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Copy,
{
    type Item = C;

    fn get(&self, identifier: <Self::Item as Identifiable>::ID) -> Option<&Self::Item> {
        self.cards.get(&identifier)
    }

    fn get_mut(&mut self, identifier: <Self::Item as Identifiable>::ID) -> Option<&mut Self::Item> {
        self.cards.get_mut(&identifier)
    }
}

impl<C> CardStorage<C>
where
    C: Card,
    <C as Identifiable>::ID: Debug + Display + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Copy,
{
    /// Creates a new object for card storage.
    pub fn new() -> Self {
        Self { cards: hashmap!{} }
    }

    /// Tries to insert the provided card into this storage object.
    ///
    /// This method first checks if the provided identifier is a known key. An [`IDCollissionError`]
    /// is returned if there is already an entry matching the key.
    /// Otherwise the new card is inserted.
    pub fn try_insert_card(&mut self, card: C) -> Result<&mut C, IDCollisionError<C::ID>> {
        match self.cards.entry(card.id()) {
            Entry::Occupied(_) => Err(IDCollisionError(card.id())),
            Entry::Vacant(entry) => Ok(entry.insert(card)),
        }
    }
}
