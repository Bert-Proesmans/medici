//! Module containing state machine construction methods.

use std::marker::PhantomData;

use medici_core::ctstack::EmptyStack;
use medici_core::service::trigger::TriggerService;
use medici_core::storage::{EntityStorage, StackStorage};

use state_machine::config::SetupConfig;
use state_machine::machine::Machine;
use state_machine::state::leaf::triggerable::Start;
use state_machine::state::toplevel::Wait;
use state_machine::transaction::Epsilon;

use entity::EntityTags;
use prototype::Game as GameProto;
use prototype::Player as PlayerProto;

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

impl Machine<Wait<Start>, EmptyStack> {
    /// Creates a new state machine ready to be started.
    pub fn new(cfg: &SetupConfig) -> Result<Self, SetupError> {
        let mut game = Self {
            state: PhantomData,
            history: PhantomData,
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
        // Count the amount of players provided by the config.
        // MaxPlayers is used as tag name because the game starts with this amount.
        // Within last-man-standing games the amount of 'alive' players is equal to or
        // smaller than the MaxPlayers amount.
        let num_players = cfg.player_names.iter().filter(|p| p.is_some()).count();
        game_entity.set_value(EntityTags::MaxPlayers, num_players as u32);
        // TODO; Other game setup steps
        Ok(self)
    }

    fn setup_players(mut self, cfg: &SetupConfig) -> Result<Self, SetupError> {
        let players_names = cfg.player_names
            .iter()
            .filter(|e| e.is_some())
            .map(|n| n.clone());
        for player_name in players_names {
            let mut player_entity = self.entities.new_entity()?;
            // Add name of player to entity.
            player_entity.human_readable = player_name;
            player_entity.add_proto::<PlayerProto>();

            // TODO; Other player setup steps
        }
        Ok(self)
    }
}
