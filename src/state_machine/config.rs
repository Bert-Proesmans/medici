//! Module containing types used to setup a new state-machine.

use std::default::Default;

/// Notifies the codebase about the maximum players our game can
/// support.
pub const MAX_PLAYERS: usize = 5;

#[derive(Debug)]
/// Structure used for setting up a new [`Machine`].
pub struct SetupConfig {
    /// Name for each player.
    ///
    /// This array is also used to calculate the amount of players to initialise.
    /// Make entries None to skip generation of a new player entity.
    ///
    /// The index within this array corresponds to the PlayerID. Do not
    /// confuse with EntityID. PlayerID is a 1-indexed ordinal number.
    pub player_names: [Option<&'static str>; MAX_PLAYERS],
    /// Maximum amount of entities to be stored inside this machine.
    pub max_entities: usize,
}

impl Default for SetupConfig {
    fn default() -> Self {
        // This will set-up a game with only 2 players
        let mut player_names = [None; MAX_PLAYERS];
        player_names[0] = Some("Player 1");
        player_names[1] = Some("Player 2");
        //
        SetupConfig {
            player_names,
            max_entities: usize::max_value(),
        }
    }
}
