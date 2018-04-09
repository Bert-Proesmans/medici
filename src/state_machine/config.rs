//! Module containing types used to setup a new state-machine.

use std::default::Default;
use std::marker::PhantomData;

use medici_core::service::storage::{EntityStorage, StackStorage};
use medici_core::service::trigger::TriggerService;

use state_machine::machine::Machine;
use state_machine::state::leaf::triggerable::Start;
use state_machine::state::toplevel::Wait;
use state_machine::transaction::Epsilon;

/// Notifies the codebase about the maximum players our game can
/// support.
pub const MAX_PLAYERS: usize = 2;

pub mod error {
    //! Definitions for all possible errors thrown when setting up a
    //! new state machine.

    use failure::Fail;

    #[derive(Debug, Fail)]
    /// Enumeration of possible errors when setting up a new game.
    pub enum SetupError {
        #[fail(display = "Player with id {:} has no valid name", _0)]
        /// Thrown when one of the provided player names is invalid.
        InvalidNameError(usize),
    }
}
use self::error::SetupError;

#[derive(Debug)]
/// Structure used for setting up a new [`Machine`].
pub struct SetupConfig {
    /// Name for each player.
    ///
    /// The index within this array corresponds to the PlayerID. Do not
    /// confuse with EntityID. PlayerID is a 1-indexed ordinal number.
    pub player_names: [&'static str; MAX_PLAYERS],
    /// Maximum amount of entities to be stored inside this machine.
    pub max_entities: usize,
}

impl Default for SetupConfig {
    fn default() -> Self {
        SetupConfig {
            player_names: ["Player 1", "Player 2"],
            max_entities: usize::max_value(),
        }
    }
}

impl Machine<Wait<Start>> {
    /// Creates a new state machine ready to be started.
    pub fn new(cfg: &SetupConfig) -> Result<Self, SetupError> {
        let mut game = Self {
            state: PhantomData,
            transaction: Epsilon,
            //
            transactions: StackStorage::new(),
            entities: EntityStorage::new(cfg.max_entities),
            triggers: TriggerService::new(),
        };
        game = game.setup_game(cfg)?;
        game = game.setup_players(cfg)?;
        Ok(game)
    }

    fn setup_game(self, cfg: &SetupConfig) -> Result<Self, SetupError> {
        Ok(self)
    }

    fn setup_players(self, cfg: &SetupConfig) -> Result<Self, SetupError> {
        Ok(self)
    }
}
