//! Specialized implementation of [`EntityStruct`] for our state machine.

use medici_core::prefab::entity::EntityStruct;

use implementation::prototype::ProtoItem;

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

/// The specialized entity structure for our state machine.
pub type Entity = EntityStruct<EntityTags, ProtoItem>;
