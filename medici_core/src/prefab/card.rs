//! Module containing items to work with game cards.

use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use function::{self, CardBuilder, CardId};
use marker;
use prefab::{timing::TimingItem, trigger::TriggerItem};

/// Reserved identifier for Game cards.
pub const GAME_CARD_ID: CardId = (0, 1);
/// Reserved identifier for Player cards.
pub const PLAYER_CARD_ID: CardId = (0, 2);

/// Card structure which makes use of the items defined by
/// the medici_core::prefab module.
pub type Card = CardStruct<i32, TimingItem, TriggerItem>;

#[derive(Debug, Clone)]
/// Type representing a game card.
pub struct CardStruct<S, ETM, ETR>
where
    S: Clone + Eq + Hash,
    ETM: marker::TimingEnumerator,
    ETR: marker::TriggerEnumerator,
{
    uid: CardId,
    /// Holds the name of this card.
    pub name: &'static str,
    /// Holds the properties attributed to this card.
    pub properties: HashMap<S, u32>,
    //
    _phantom: PhantomData<(ETM, ETR)>,
}

impl<S, ETM, ETR> function::Identifiable for CardStruct<S, ETM, ETR>
where
    S: Clone + Eq + Hash,
    ETM: marker::TimingEnumerator,
    ETR: marker::TriggerEnumerator,
{
    type ID = CardId;

    fn id(&self) -> CardId {
        self.uid
    }
}

impl<S, ETM, ETR> function::Card for CardStruct<S, ETM, ETR>
where
    S: Clone + Eq + Hash,
    ETM: marker::TimingEnumerator,
    ETR: marker::TriggerEnumerator,
{
    type TimingEnum = ETM;
    type TriggerEnum = ETR;    
}

impl<S, ETM, ETR> CardBuilder<CardStruct<S, ETM, ETR>> for CardStruct<S, ETM, ETR>
where
    S: Clone + Eq + Hash,
    ETM: marker::TimingEnumerator,
    ETR: marker::TriggerEnumerator,
{
    fn new_with_id<I: Into<CardId>>(id: I) -> Self {
        Self {
            uid: id.into(),
            name: "[MISSING NAME]",
            properties: hashmap!{},
            _phantom: PhantomData,
        }
    }
}
