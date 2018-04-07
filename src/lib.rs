// Linters.
#![allow(unknown_lints, dead_code, unused_mut, unused_variables, let_and_return, useless_format)]
// Prevent successful compilation when documentation is missing.
#![deny(missing_docs)]
// Unstable features.
#![feature(associated_type_defaults, try_from, never_type, proc_macro)]
// Clippy linting when building debug versions.
//#![cfg_attr(test, feature(plugin))]
//#![cfg_attr(test, plugin(clippy))]
// Linters for code residing in documentation.
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

//! Crate used to show off the power of the Medici framework.

extern crate failure;
extern crate lazy_static;
extern crate maplit;
extern crate value_from_type_macros;
extern crate value_from_type_traits;

// Medici opinionated framework.
extern crate medici_core;

pub mod card_set;
pub mod state_machine;

// Note: Keep tests during structural upgrade. These will be used
// to verify everything works as expected.

#[cfg(test)]
mod tests {
    use std::default::Default;

    use medici_core::stm::*;

    use medici::state_machine::prelude::*;
    use medici::state_machine::state::prelude::*;
    use medici::state_machine::transaction::*;

    #[test]
    fn entry() {
        let config: SetupConfig = Default::default();
        let mut game = Game::new(config).expect("Error creating new game!");

        {
            let game_entity = game.entities.entity(GAME_E_ID).unwrap();
            assert_eq!(GAME_E_ID, game_entity.into());
        }

        // Add trigger
        game.triggers.add_trigger(turn_end_trigger).unwrap();

        // Start game
        let game: Game<Wait<Input>> = game.transition(Epsilon);

        // Do stuff
        let first_turn = end_turn(game).expect("Game unexpectedly finished");
        let _second_turn = end_turn(first_turn).expect("Game unexpectedly finished");

        println!("OK - Finished");
    }

    /*
    #[test]
    fn listeners() {
        let config: SetupConfig = Default::default();
        let mut new_game = Game::new(config).expect("Error creating new game!");

        // Add trigger
        new_game.listeners.add_trigger(turn_end_trigger).unwrap();

        // Start game
        let new_game: Game<Wait<Input>> = new_game.transition(Epsilon());

        // Do stuff
        let first_turn = end_turn(new_game).expect("Game unexpectedly finished");
        let _second_turn = end_turn(first_turn).expect("Game unexpectedly finished");
    }

    #[test]
    fn entities() {
        let game_card = CardContainer::game_card();
        let mut game_entity = Entity::new(GAME_E_ID, game_card);
        game_entity
            .add_proto::<GameProto>()
            .expect("Error in proto assignment!");
        game_entity
            .as_proto::<GameProto>()
            .expect("Error in proto retrieval!");

        game_entity
            .add_proto::<GameProtoMut>()
            .expect("Error in proto assignment!");
        game_entity
            .as_proto_mut::<GameProtoMut>()
            .expect("Error in proto retrieval!");
    }
    */
}
