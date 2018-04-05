//! Module containing structures for storing game card objects.

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use function::Card;
use marker::{Service, TimingEnumerator, TriggerEnumerator};
use service::storage::UnsafeTrigger;

#[derive(Debug, Clone)]
/// Structure serializing/deserializing a game card.
pub struct UnsafeCardEntry<C>
where
    C: Card,
    C::TimingEnum: TimingEnumerator + Copy,
    C::TriggerEnum: TriggerEnumerator + Copy,
{
    card: C,
    triggers: Vec<UnsafeTrigger<C::TimingEnum, C::TriggerEnum>>,
}

#[derive(Debug, Clone)]
/// Structure holding onto all cards defined for a specific machine.
pub struct CardStorage<C>
where
    C: Card,
    C::UID: Copy + Eq + Hash,
    C::TimingEnum: TimingEnumerator + Debug + Copy,
    C::TriggerEnum: TriggerEnumerator + Debug + Copy,
{
    /// Contains unsafe versions of implemented cards.
    pub cards: HashMap<C::UID, UnsafeCardEntry<C>>,
}

impl<C> Service for CardStorage<C>
where
    C: Card,
    C::UID: Copy + Eq + Hash,
    C::TimingEnum: TimingEnumerator + Debug + Copy,
    C::TriggerEnum: TriggerEnumerator + Debug + Copy,
{
}

impl<C> CardStorage<C>
where
    C: Card,
    C::UID: Copy + Eq + Hash,
    C::TimingEnum: TimingEnumerator + Debug + Copy,
    C::TriggerEnum: TriggerEnumerator + Debug + Copy,
{
    /// Creates a new object for card storage.
    pub fn new() -> Self {
        Self { cards: hashmap!{} }
    }
}