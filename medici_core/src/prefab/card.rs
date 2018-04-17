//! Module containing items to work with game cards.

use std::collections::HashMap;
use std::hash::Hash;

use function::{self, CardId};
use prefab::{timing::TimingItem, trigger::TriggerItem};

/// Card structure which makes use of the items defined by
/// the medici_core::prefab module.
pub type Card = CardStruct<i32>;

#[derive(Debug, Clone)]
/// Type representing a game card.
pub struct CardStruct<S>
where
    S: Clone + Eq + Hash,
{
    uid: CardId,
    /// Holds the name of this card.
    pub name: &'static str,
    /// Holds the properties attributed to this card.
    pub properties: HashMap<S, u32>,
}

impl<S> function::Card for CardStruct<S>
where
    S: Clone + Eq + Hash,
{
    type UID = CardId;
    type TimingEnum = TimingItem;
    type TriggerEnum = TriggerItem;

    fn uid(&self) -> CardId {
        self.uid
    }
}
