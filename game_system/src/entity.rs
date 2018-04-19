//! Specialized implementation of [`EntityStruct`] for our state machine.

use medici_core::prefab::entity::EntityStruct;
use prototype::ProtoItem;

use medici_core::function::ZoneEnumerator;
/// Unique ID value for the Game entity.
pub use medici_core::prefab::entity::GAME_E_ID;
/// The specialized entity structure for our state machine.
pub type Entity = EntityStruct<EntityTags, ProtoItem>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
/// Enumeration of all conceptual positions an entity can be found at.
///
/// A zone is an abstraction of fysical locations within the context of a board game.
/// When a player's pawn is on the board, it could be in the PLAY-zone. When a card
/// is within a deck, it could be in the DECK-zone. Each player could have a HAND-zone.
///
/// Zones are isolated per player or shared for all players.
pub enum Zones {
    /// Isolated zone per player representing a void zone which can hold an 'unlimited'
    /// amount of entities.
    /// This zone has no function, entities just exist there.
    Void,
    /// Isolated zone per player representing it's deck (of card entities).
    Deck,
    /// Isolated zone per player representing it's hand (of card entities).
    Hand,
}

impl ZoneEnumerator for Zones {
    fn max_entities(&self) -> usize {
        match *self {
            Zones::Deck => 100,
            Zones::Hand => 10,
            _ => usize::max_value(),
        }
    }
}

impl Default for Zones {
    fn default() -> Zones {
        Zones::Void
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
/// Enumeration of all entity property keys.
pub enum EntityTags {
    /// Amount of players registered on this game.
    MaxPlayers,
    /// Index of the player who is currently on-turn. This index
    /// starts counting at 1. eg: First-, second-, third-,.. player.
    CurrentPlayerOrd,
    /// Holds the amount of turns the current player has remaining.
    /// 0 means the next player's turn will start on turn_end.
    RemainingTurns,
}
