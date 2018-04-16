//! Module containing state machine construction methods.

use std::marker::PhantomData;

use medici_core::ctstack::EmptyStack;
use medici_core::service::storage::{EntityStorage, StackStorage};
use medici_core::service::trigger::TriggerService;

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

#[cfg(test)]
mod tests {
    use super::*;
    use medici_core::prefab::entity::GAME_E_ID;
    use std::default::Default;

    #[test]
    #[should_panic]
    fn max_entities() {
        let mut config: SetupConfig = Default::default();
        // Configure for 1 player.
        config.player_names = Default::default();
        config.player_names[0] = Some("Player 1".into());
        // Limit maximum entities to 1 (only the game entity).
        config.max_entities = 1;
        // TODO; Update when errors are properly linked!
        let machine = Machine::new(&config).unwrap();
        // If it DOESN'T panic, print all data.
        println!("Panic did NOT happen! Printing relevant structs");
        println!("{:?}", config);
        println!("{:?}", machine);
    }

    #[test]
    fn config() {
        const NUM_PLAYERS: usize = 5;
        //
        let mut config: SetupConfig = Default::default();
        // Generates 0, 1, 2, 3, ..
        for i in 0..NUM_PLAYERS {
            let player_name: String = i.to_string();
            config.player_names[i] = Some(player_name);
        }
        config.max_entities = 50;
        let machine = Machine::new(&config).unwrap();
        //
        assert_eq!(GAME_E_ID, 0);
        let game_entity = machine.entities.get(GAME_E_ID).unwrap();
        let max_players = game_entity.get_value(&EntityTags::MaxPlayers).unwrap();
        assert_eq!(NUM_PLAYERS, max_players as usize);
        // Check the name for each player entity
        for i in 0..NUM_PLAYERS {
            // Player id is 1-indexed
            let player_idx = i + 1;
            let player = machine.entities.get(player_idx).unwrap();
            assert_eq!(player.human_readable, config.player_names[i]);
        }
    }
}
