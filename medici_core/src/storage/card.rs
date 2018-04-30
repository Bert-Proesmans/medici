//! Module containing structures for storing game card objects.

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use function::{Card, Identifiable, IndexedStorageCompliance};
use marker;
use storage::trigger::UnsafeTrigger;

#[derive(Debug, Clone)]
/// Wrapper for a GameCard.
///
/// This wrapper is unsafe because working with the triggers it contain
/// is unsafe. The actual type of the state machine has been erased.
/// See [`UnsafeTrigger`].
pub struct UnsafeCardEntry<C>
where
    C: Card,
    C::TimingEnum: marker::TimingEnumerator + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Copy,
{
    card: C,
    triggers: Vec<UnsafeTrigger<C::TimingEnum, C::TriggerEnum>>,
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
    <C as Identifiable>::ID: Debug + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Copy,
{
    /// Creates a new object for card storage.
    pub fn new() -> Self {
        Self { cards: hashmap!{} }
    }
}
