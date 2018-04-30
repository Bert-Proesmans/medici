//! Module containing items to work with game cards.
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use error::custom_type::MissingPropertyError;
use function::{self, CardBuilder, CardId, ServiceCompliance};
use marker;
use prefab::{timing::TimingItem, trigger::TriggerItem};
use service::TriggerService;

/// Reserved identifier for Game cards.
pub const GAME_CARD_ID: CardId = CardId::new(0, 1);
/// Reserved identifier for Player cards.
pub const PLAYER_CARD_ID: CardId = CardId::new(0, 2);

/// Card structure which makes use of the items defined by
/// the medici_core::prefab module.
pub type Card = CardStruct<i32, TimingItem, TriggerItem>;

#[derive(Debug, Clone)]
/// Type representing a game card.
pub struct CardStruct<S, ETM, ETR>
where
    S: Clone + Eq + Hash,
    ETM: marker::TimingEnumerator + PartialEq + Copy,
    ETR: marker::TriggerEnumerator + PartialEq + Copy,
{
    uid: CardId,
    /// Holds the name of this card.
    pub name: &'static str,
    /// Holds the properties attributed to this card.
    pub state: HashMap<S, u32>,
    /// Holds all triggers registered on this card.
    pub triggers: TriggerService<ETM, ETR>,
    //
    _phantom: PhantomData<(ETM, ETR)>,
}

impl<S, ETM, ETR> function::Identifiable for CardStruct<S, ETM, ETR>
where
    S: Clone + Eq + Hash,
    ETM: marker::TimingEnumerator + PartialEq + Copy,
    ETR: marker::TriggerEnumerator + PartialEq + Copy,
{
    type ID = CardId;

    fn id(&self) -> CardId {
        self.uid
    }
}

impl<S, ETM, ETR> function::Card for CardStruct<S, ETM, ETR>
where
    S: Clone + Eq + Hash,
    ETM: marker::TimingEnumerator + PartialEq + Copy,
    ETR: marker::TriggerEnumerator + PartialEq + Copy,
{
    type TimingEnum = ETM;
    type TriggerEnum = ETR;
}

impl<S, ETM, ETR> CardBuilder<CardStruct<S, ETM, ETR>> for CardStruct<S, ETM, ETR>
where
    S: Clone + Eq + Hash,
    ETM: marker::TimingEnumerator + PartialEq + Copy,
    ETR: marker::TriggerEnumerator + PartialEq + Copy,
{
    fn new_with_id<I: Into<CardId>>(id: I) -> Self {
        Self {
            uid: id.into(),
            name: "[MISSING NAME]",
            state: hashmap!{},
            triggers: TriggerService::new(),
            _phantom: PhantomData,
        }
    }
}

impl<S, ETM, ETR> CardStruct<S, ETM, ETR>
where
    S: Debug + Clone + Eq + Hash,
    ETM: marker::TimingEnumerator + PartialEq + Copy,
    ETR: marker::TriggerEnumerator + PartialEq + Copy,
{
    /// Retrieves the value of the requested property defined within this card.
    /// 0 is returned as default value when the property key was not found!
    pub fn get_value_default(&self, key: &S) -> u32 {
        self.state.get(key).cloned().or(Some(0)).unwrap()
    }

    /// Retrieves the value of the requested property defined within this card.
    pub fn get_value(&self, key: &S) -> Result<u32, MissingPropertyError<CardId, S>> {
        self.state
            .get(key)
            .cloned()
            .ok_or_else(|| MissingPropertyError(self.uid, key.clone()))
    }

    /// Store the provided property key with corresponding value into this card.
    ///
    /// The old value is returned if the key was already known within this card.
    pub fn set_value(&mut self, key: S, value: u32) -> Option<u32> {
        self.state.insert(key, value)
    }
}

impl<S, ETM, ETR> ServiceCompliance<TriggerService<ETM, ETR>> for CardStruct<S, ETM, ETR>
where
    S: Clone + Eq + Hash,
    ETM: marker::TimingEnumerator + PartialEq + Copy,
    ETR: marker::TriggerEnumerator + PartialEq + Copy,
{
    fn get(&self) -> &TriggerService<ETM, ETR> {
        &self.triggers
    }

    fn get_mut(&mut self) -> &mut TriggerService<ETM, ETR> {
        &mut self.triggers
    }
}
