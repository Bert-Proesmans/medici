// Linters.
#![allow(unknown_lints, dead_code, unused_mut, unused_variables, let_and_return, useless_format)]
// Prevent successful compilation when documentation is missing.
#![deny(missing_docs)]
// Unstable features.
#![feature(associated_type_defaults, try_from, never_type, proc_macro, nll)]
// Clippy linting when building debug versions.
//#![cfg_attr(test, feature(plugin))]
//#![cfg_attr(test, plugin(clippy))]
// Linters for code residing in documentation.
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

//! Crate used to show off the power of the Medici framework.

extern crate failure;
extern crate failure_derive;
extern crate lazy_static;
extern crate maplit;
extern crate value_from_type_macros;
extern crate value_from_type_traits;

// Medici opinionated framework.
extern crate medici_core;

pub mod card_set;
pub mod implementation;
pub mod state_machine;

// Note: Keep tests during structural upgrade. These will be used
// to verify everything works as expected.

#[cfg(test)]
mod tests {
    use std::default::Default;

    use failure::{Error, Fail};

    use medici_core::function::Entity;
    use medici_core::prefab::entity::GAME_E_ID;
    use medici_core::service::error::MissingEntityError;
    // use medici_core::stm::*;

    use super::implementation::effect::action::{end_turn, start_game};
    use super::implementation::effect::trigger::{start_game_trigger, turn_end_trigger};
    use super::implementation::entity::EntityTags;
    use super::state_machine::prelude::*;
    // use super::state_machine::state::prelude::*;
    // use super::state_machine::transaction::*;

    #[test]
    fn failure_derive() {
        let id: usize = 0;
        let error = MissingEntityError(id);
        let fail: &Fail = &MissingEntityError(id);
        let err: Error = MissingEntityError(id).into();
    }

    #[test]
    fn entry() {
        let config: SetupConfig = Default::default();
        let mut game = Machine::new(&config).expect("Error creating new game!");

        {
            let game_entity = game.entities.get(GAME_E_ID).unwrap();
            assert_eq!(GAME_E_ID, 0);
            assert_eq!(GAME_E_ID, game_entity.id());
        }

        // Add triggers
        game.triggers.add_trigger(start_game_trigger).unwrap();
        game.triggers.add_trigger(turn_end_trigger).unwrap();

        // Start game
        let first_turn = start_game(game).expect("Game unexpectedly finished");

        // Check we're currently within the turn of player 1.
        let game_entity = first_turn.entities.get(GAME_E_ID).unwrap();
        assert_eq!(
            game_entity.get_value_default(&EntityTags::CurrentPlayerOrd),
            1
        );
        let second_turn = end_turn(first_turn).expect("Game unexpectedly finished");

        // Check we're currently within the turn of player 2.
        let game_entity = second_turn.entities.get(GAME_E_ID).unwrap();
        assert_eq!(
            game_entity.get_value_default(&EntityTags::CurrentPlayerOrd),
            2
        );
        let _third_turn = end_turn(second_turn).expect("Game unexpectedly finished");
    }
}
