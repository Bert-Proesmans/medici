//! Module containing types used to setup a new state-machine.

use std::default::Default;

/// Constant defining how much memory at minimum MUST be (statically) allocated
/// to support all players that joined the game.
pub const MAX_PLAYERS: usize = 5;

#[derive(Debug)]
/// Structure used for setting up a new [`Machine`].
pub struct SetupConfig {
    /// Name for each player.
    ///
    /// This array is also used to calculate the amount of players to initialise.
    /// Make entries None to skip generation of a new player entity.
    ///
    /// # Note
    /// PlayerID is a 1-indexed ordinal number. The first player defined as [`Some(String)`]
    /// will receive PlayerID 1.
    pub player_names: [Option<String>; MAX_PLAYERS],
    /// Maximum amount of entities to be stored inside this machine.
    pub max_entities: usize,
}

impl Default for SetupConfig {
    fn default() -> Self {
        // This will set-up a game with only 2 players
        let mut player_names: [_; MAX_PLAYERS] = Default::default();
        player_names[0] = Some("Player 1".into());
        player_names[1] = Some("Player 2".into());
        //
        SetupConfig {
            player_names,
            max_entities: usize::max_value(),
        }
    }
}
