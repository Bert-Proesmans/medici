//! Module containing structures for storing game card objects.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use function::{Card, Identifiable, IndexedStorageCompliance};
use error::custom_type::{IDCollisionError, MissingCardError};
use function::Card;
use marker;
use storage::trigger::UnsafeTrigger;

#[derive(Debug, Clone)]
/// Structure wrapping a game card.
///
/// # Unsafe
/// This type is unsafe because it holds type erased trigger methods ([`UnsafeTrigger`]).
/// All safely accessible data is stored within the [`UnsafeCardEntry::card`] field.
pub struct UnsafeCardEntry<C>
where
    C: Card,
    C::TimingEnum: marker::TimingEnumerator + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Copy,
{
    /// Data specific to each card.
    ///
    /// This is a seperate field because any state machine functionlity "attached" to this
    /// card is contained by [`UnsafeCardEntry`] itself.
    pub card: C,
    /// Vector of type erased [`Trigger`] methods.
    pub triggers: Vec<UnsafeTrigger<C::TimingEnum, C::TriggerEnum>>,
}

impl<C> From<C> for UnsafeCardEntry<C>
where
    C: Card,
    C::TimingEnum: marker::TimingEnumerator + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Copy,
{
    fn from(card: C) -> Self {
        Self {
            card,
            triggers: vec![],
        }
    }
}

impl<C> Identifiable for UnsafeCardEntry<C>
where
    C: Card,
    C::TimingEnum: marker::TimingEnumerator + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Copy,
{
    type ID = <C as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.card.id()
    }
}

#[derive(Debug, Clone)]
/// Structure holding onto all cards defined for a specific machine.
pub struct CardStorage<C>
where
    C: Card,
    <C as Identifiable>::ID: Debug + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Copy,
{
    /// Contains unsafe versions of implemented cards.
    pub cards: HashMap<<C as Identifiable>::ID, UnsafeCardEntry<C>>,
}

impl<C> IndexedStorageCompliance for CardStorage<C>
where
    C: Card,
    <C as Identifiable>::ID: Debug + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Copy,
{
    type Item = UnsafeCardEntry<C>;

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

    /// Adds a the provided card
    pub fn add_unsafe_card(
        &mut self,
        unsafe_card: UnsafeCardEntry<C>,
    ) -> Result<&mut UnsafeCardEntry<C>, IDCollisionError<C::UID>> {
        match self.cards.entry(unsafe_card.card.uid()) {
            Entry::Occupied(_) => Err(IDCollisionError(unsafe_card.card.uid())),
            // The map contains unsafe Card entries, but we return the safe variant.
            // This means a reference from the entry into the card field is passed as result.
            Entry::Vacant(v_v) => Ok(v_v.insert(unsafe_card)),
        }
    }

    /// Retrieves the stored [`UnsafeCardEntry`] matching the provided identifier.
    pub fn get_unsafe_card(
        &self,
        id: C::UID,
    ) -> Result<&UnsafeCardEntry<C>, MissingCardError<C::UID>> {
        self.cards.get(&id).ok_or_else(|| MissingCardError(id))
    }

    /// Retrieves the stored [`UnsafeCardEntry`] matching the provided identifier.
    pub fn get_unsafe_card_mut(
        &mut self,
        id: C::UID,
    ) -> Result<&mut UnsafeCardEntry<C>, MissingCardError<C::UID>> {
        self.cards.get_mut(&id).ok_or_else(|| MissingCardError(id))
    }
}
