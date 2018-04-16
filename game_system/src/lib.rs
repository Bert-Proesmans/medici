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

//! Crate to implement the state machine for a specific kind of game.
//! Any type which is used in managing the built state machine MUST be exported
//! from this crate.
//! Any type that's directly used from medici_core MUST transitively be re-exported here.
//! Re-exporting allows the downstream crates to have a single dependancy, this crate.

extern crate failure;
extern crate failure_derive;
extern crate maplit;
extern crate value_from_type_macros;
extern crate value_from_type_traits;

// Medici opinionated framework.
extern crate medici_core;

pub mod entity;
pub mod prototype;
pub mod runtime;
pub mod setup;
pub mod state_machine;

/// Re-export these types because macros defined within this crate need access to them.
pub mod re_export {
    pub use medici_core::function;
    pub use medici_core::marker;
    pub use medici_core::service;
    pub use medici_core::storage;
    pub use medici_core::stm::checked::{PullupFrom, PushdownFrom, TransitionFrom};
    // Macro re-exported
    pub use medici_core::ct;
}

/// Re-exports and new types often used when interacting with the built state machine.
pub mod prelude {
    // These traits must be in scope to properly use [`PullupInto::pullup`] and
    // [`PushdownInto::pushdown`], ..
    pub use medici_core::ctstack::*;
    pub use medici_core::error::*;
    pub use medici_core::stm::checked::{PullupInto, PushdownInto, TransitionInto};
    pub use medici_core::transaction::{pack_transaction, unpack_transaction};

    pub use entity::*;
    pub use state_machine::config::SetupConfig;
    pub use state_machine::machine::Machine;
    pub use state_machine::state::leaf::triggerable::*;
    pub use state_machine::state::leaf::*;
    pub use state_machine::state::toplevel::*;

    // Transactions and Prototypes are NOT re-exported within the module
    // because their names could clash with States.
    // Users are anyway encouraged to prefix these types with an explicit `transaction::`
    // and `prototype::` for better code readability.
    pub use prototype;
    pub use state_machine::transaction;
}

/*
#[cfg(test)]
mod tests {
    use std::default::Default;

    use failure::{Error, Fail};

    use medici_core::ctstack::AnyStack;
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

    /*
    #[test]
    fn transition() {
        let config: SetupConfig = Default::default();
        let game: Machine![Wait<Start>] = Machine::new(&config).expect("Error creating new game!");
        let game: Machine![Action<Start>] = game.transitio(Epsilon).expect("Game enexpectedly finished");
        let game: Machine![Effect<Start>] = game.pushdown(Epsilon).expect("Game enexpectedly finished");
    }
    */

    #[test]
    fn entry() {
        let config: SetupConfig = Default::default();
        let mut game = Machine::new(&config).expect("Error creating new game!");

        {
            let game_entity = game.entities.get(GAME_E_ID).unwrap();
            assert_eq!(GAME_E_ID, 0);
            assert_eq!(GAME_E_ID, game_entity.id());
        }

        // Add triggers.
        // These triggers are specialized to use AnyStack for the compile-time stack
        // generic parameter. This is allowed because the size of any CTStack within
        // our state machine is 0.
        game.triggers
            .add_trigger(start_game_trigger::<AnyStack>)
            .unwrap();
        game.triggers
            .add_trigger(turn_end_trigger::<AnyStack>)
            .unwrap();

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
*/
