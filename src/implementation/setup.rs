//! Module containing state machine construction methods.

use std::marker::PhantomData;

use medici_core::service::storage::{EntityStorage, StackStorage};
use medici_core::service::trigger::TriggerService;

use state_machine::config::SetupConfig;
use state_machine::machine::Machine;
use state_machine::state::leaf::triggerable::Start;
use state_machine::state::toplevel::Wait;
use state_machine::transaction::Epsilon;

use implementation::prototype::Game as GameProto;
use implementation::prototype::Player as PlayerProto;

pub mod error {
    //! Definitions for all possible errors thrown when setting up a
    //! new state machine.

    use failure::Fail;
    use medici_core::service::error::OverflowError;

    #[derive(Debug, Fail)]
    /// Enumeration of possible errors when setting up a new game.
    pub enum SetupError {
        #[fail(display = "Player with id {:} has no valid name", _0)]
        /// Thrown when one of the provided player names is invalid.
        InvalidNameError(usize),

        #[fail(display = "Maximum entity limit reached. Increase the allowed amount of entities!")]
        /// Thrown when max entities is overflown.
        EntityOverflow(#[fail(cause)] OverflowError),
    }

    impl From<OverflowError> for SetupError {
        fn from(x: OverflowError) -> Self {
            SetupError::EntityOverflow(x)
        }
    }
}
use self::error::SetupError;

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

    fn setup_game(mut self, cfg: &SetupConfig) -> Result<Self, SetupError> {
        let game_entity = self.entities.new_entity()?;
        game_entity.add_proto::<GameProto>();
        // TODO; Other game setup
        Ok(self)
    }

    fn setup_players(mut self, cfg: &SetupConfig) -> Result<Self, SetupError> {
        let players_to_setup = cfg.player_names.iter().filter(|e| e.is_some());
        for player in players_to_setup {
            let player_entity = self.entities.new_entity()?;
            player_entity.add_proto::<PlayerProto>();
            // TODO; Other player setup
        }
        Ok(self)
    }
}
