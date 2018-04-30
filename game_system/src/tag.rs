//! Module containing the property tags which can be attributed to [`Entity`]s
//! and other game-objects.

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
/// Enumeration of all entity property keys.
pub enum EntityTags {
    /* GAME META TAGS */
    /// Amount of players registered on this game.
    MaxPlayers,
    /// Index of the player who is currently on-turn. This index
    /// starts counting at 1. eg: First-, second-, third-,.. player.
    CurrentPlayerOrd,
    /// Holds the amount of turns the current player has remaining.
    /// 0 means the next player's turn will start on turn_end.
    RemainingTurns,
    /// Holds the amount of cards you start the game with.
    StartHandSize,

    /* ENTITY OBJECT TAGS */
    /// Amount of damage an entity can do.
    Attack,
    /// Amount of health an entity has.
    Health,
    /// Amount of damage an entity has taken.
    Damage,
}
