//! Contains functionality to work with [`Card`]s.

use std::fmt::{Debug, Display};
use std::hash::Hash;

use error::custom_type::{IDCollisionError, MissingCardError};
use function::{Card, Identifiable};
use marker;
use storage::CardStorage;

#[derive(Debug)]
/// Object allowing manipulation of game [`Card`]s.
pub struct CardService<C>
where
    C: Card,
    <C as Identifiable>::ID: Debug + Display + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Display + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Display + Copy,
{
    storage: CardStorage<C>,
}

impl<C> marker::Service for CardService<C>
where
    C: Card,
    <C as Identifiable>::ID: Debug + Display + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Display + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Display + Copy,
{
}

impl<C> CardService<C>
where
    C: Card,
    <C as Identifiable>::ID: Debug + Display + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Display + PartialEq + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Display + PartialEq + Copy,
{
    /// Creates a new object of this service.
    pub fn new() -> Self {
        Self {
            storage: CardStorage::new(),
        }
    }

    /// Stores the provided card.
    pub fn register_card(&mut self, card: C) -> Result<&mut C, IDCollisionError<C::ID>> {
        self.storage.try_insert_card(card)
    }

    /// Fetches the card matching the provided identifier from this storage object.
    pub fn get_card(&self, id: C::ID) -> Result<&C, MissingCardError<C::ID>> {
        self.storage
            .cards
            .get(&id)
            .ok_or_else(|| MissingCardError(id))
    }
}
