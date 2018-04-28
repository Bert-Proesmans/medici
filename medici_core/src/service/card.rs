//! Contains functionality to work with [`Card`]s.

use std::fmt::{Debug, Display};
use std::hash::Hash;

use value_from_type_traits::IntoEnum;

use error::custom_type::{IDCollisionError, MissingCardError};
use function::{Card, StateContainer, TriggerState};
use marker;
use service::trigger::{TriggerWrapper, _FNTrigger};
use storage::trigger::UnsafeTrigger;
use storage::CardStorage;

#[derive(Debug)]
/// Object allowing manipulation of game [`Card`]s.
pub struct CardService<C>
where
    C: Card,
    C::UID: Debug + Display + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Display + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Display + Copy,
{
    storage: CardStorage<C>,
}

impl<C> marker::Service for CardService<C>
where
    C: Card,
    C::UID: Debug + Display + Copy + Eq + Hash,
    C::TimingEnum: marker::TimingEnumerator + Debug + Display + Copy,
    C::TriggerEnum: marker::TriggerEnumerator + Debug + Display + Copy,
{
}

impl<C> CardService<C>
where
    C: Card,
    C::UID: Debug + Display + Copy + Eq + Hash,
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
    pub fn register_card(&mut self, card: C) -> Result<&mut C, IDCollisionError<C::UID>> {
        self.storage
            .add_unsafe_card(card.into())
            .map(|unsafe_card| &mut unsafe_card.card)
    }

    /// Fetches the card matching the provided identifier from this storage object.
    pub fn get_card(&self, id: C::UID) -> Result<&C, MissingCardError<C::UID>> {
        self.storage
            .get_unsafe_card(id)
            .map(|unsafe_card| &unsafe_card.card)
    }

    /// Adds a new trigger to the provided card.
    pub fn store_trigger<M>(
        &mut self,
        id: C::UID,
        trigger: _FNTrigger<M>,
    ) -> Result<(), MissingCardError<C::UID>>
    where
        M: StateContainer,
        M::State: TriggerState,
        <M::State as TriggerState>::Timing: marker::Timing + IntoEnum<C::TimingEnum>,
        <M::State as TriggerState>::Trigger: marker::Triggerable + IntoEnum<C::TriggerEnum>,
    {
        let card_entry = self.storage.get_unsafe_card_mut(id)?;
        let safe_wrapper = TriggerWrapper::<M, C::TimingEnum, C::TriggerEnum>::new(trigger);
        card_entry.triggers.push(safe_wrapper.into());
        Ok(())
    }

    /// Retrieves an iterator of all triggers assigned to a specific card.
    pub fn retrieve_triggers(
        &self,
        id: C::UID,
    ) -> Result<
        impl Iterator<Item = &UnsafeTrigger<C::TimingEnum, C::TriggerEnum>>,
        MissingCardError<C::UID>,
    > {
        Ok(self.storage.get_unsafe_card(id)?.triggers.iter())
    }
}
