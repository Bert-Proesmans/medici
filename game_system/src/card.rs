//! Specialized implementation of the [`Card`] game-object.

use medici_core::prefab::card::CardStruct;

use state_machine::state::prelude::{TimingItem, TriggerItem};
use tag::EntityTags;

/// [Re-export] Identifier for referencing Game cards.
pub use medici_core::prefab::card::GAME_CARD_ID;
/// [Re-export] Identifier for referencing Player cards.
pub use medici_core::prefab::card::PLAYER_CARD_ID;

/// The specialized [`Card`] structure.
pub type Card = CardStruct<EntityTags, TimingItem, TriggerItem>;
