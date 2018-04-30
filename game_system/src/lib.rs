// Linters.
#![allow()]
// Prevent successful compilation when documentation is missing.
#![deny(missing_docs)]
// Unstable features.
#![feature(
    associated_type_defaults, try_from, never_type, proc_macro, proc_macro_mod,
    proc_macro_path_invoc, nll
)]
// Clippy linting when building debug versions.
//#![cfg_attr(test, feature(plugin))]
//#![cfg_attr(test, plugin(clippy))]
// Linters for code residing in documentation.
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

//! # Example crate
//! Implements the state machine for a board game.
//! Any type which interacts with the built state machine MUST be exported
//! from this crate.
//! Any type that's directly used from medici_core MUST transitively be re-exported here.
//! Re-exporting allows the downstream crates to have a single dependancy, this crate.
//!
//! # See also
//! [`game_rules`] for an example how to implement game rules which use the state machine
//! defined within this crate.

extern crate failure;
extern crate failure_derive;
extern crate maplit;
extern crate value_from_type_macros;
extern crate value_from_type_traits;
#[macro_use]
extern crate lazy_static;

// Medici opinionated framework.
extern crate medici_core;

#[macro_use]
pub mod card;
pub mod entity;
pub mod prototype;
pub mod runtime;
pub mod setup;
pub mod state_machine;
pub mod tag;

/// Exported types from [`medici_core`].
///
/// This crate declares macros which make direct use of the exported types so we make
/// them available through this crate to keep the amount of dependancies small.
/// This effectively removes the dependancy on [`medici_core`] for any downstream crate.
pub mod re_export {
    pub use medici_core::function;
    pub use medici_core::marker;
    pub use medici_core::service;
    pub use medici_core::stm::checked::{PullupFrom, PushdownFrom, TransitionFrom};
    pub use medici_core::storage;
    // Macro re-exported
    pub use medici_core::ct;
}

/// Often used types exported together for ease of use.
///
/// This module can be imported like so
/// ```
/// # #![allow(unused_imports)]
/// extern crate game_system;
/// use game_system::prelude::*;
/// ```
/// and will cause all exported types to be injected in the declared scope.
pub mod prelude {
    // These traits must be in scope to properly use [`PullupInto::pullup`],
    // [`PushdownInto::pushdown`] and [`TransitionInto::transition`].
    pub use medici_core::ctstack::*;
    pub use medici_core::error::{self, ErrorKind, FrontendErrorExt, HydratedErrorExt, MachineError};
    pub use medici_core::function::{self, ArrayStorageCompliance, Card as CardTrait, CardBuilder,
                                    CardId, Entity as EntityTrait, EntityBuilder, EntityId,
                                    Identifiable, IndexedStorageCompliance, ServiceCompliance,
                                    StackStorageCompliance};
    pub use medici_core::stm::checked::{PullupInto, PushdownInto, TransitionInto};
    pub use medici_core::transaction::{pack_transaction, unpack_transaction};

    /* Macros */
    // Error handling macro's
    pub use medici_core::{ctxt, hydrate};
    // Custom card implementation macro.
    // FAILS TO BE FOUND!
    // pub use super::card_impl;

    pub use card::Card;
    pub use entity::{Entity, GAME_E_ID};
    pub use state_machine::config::SetupConfig;
    pub use state_machine::machine::Machine;
    pub use state_machine::state::leaf::triggerable::*;
    pub use state_machine::state::leaf::*;
    pub use state_machine::state::toplevel::*;
    pub use tag::EntityTags;

    // Transactions and Prototypes are NOT re-exported within the module
    // because their names could clash with States.
    // Users are anyway encouraged to prefix these types with an explicit `transaction::`
    // and `prototype::` for better code readability.
    pub use prototype;
    pub use state_machine::transaction;
}

#[cfg(test)]
mod tests {
    use failure::{Error, Fail};
    use prelude::error::custom_type::MissingEntityError;
    use prelude::*;
    use re_export::*;
    use std::marker::PhantomData;

    #[test]
    fn failure_derive() {
        //
        let id: usize = 0;
        let _error = MissingEntityError(id);
        let _fail: &Fail = &MissingEntityError(id);
        let _err: Error = MissingEntityError(id).into();
    }

    #[test]
    fn checked_transitions() {
        // Build a new machine to reuse internal parts to build a custom one.
        let machine = Machine::new(&Default::default()).expect("Error building machine");
        let machine: Machine<Action<Start>, EmptyStack> = Machine {
            state: PhantomData,
            history: PhantomData,
            transaction: transaction::Epsilon,
            //
            transactions: machine.transactions,
            entities: machine.entities,
            triggers: machine.triggers,
        };

        println!("START\n{:?}\n", machine);
        let push: Machine<Effect<Start>, _> =
            PushdownFrom::pushdown_from(machine, transaction::Epsilon);
        println!("PUSHED DOWN\n{:?}\n", push);
        let pull: Machine<Action<Start>, _> =
            PullupFrom::pullup_from(push).expect("Failed to pullup!");
        println!("PULLED UP\n{:?}\n", pull);
    }

    #[test]
    fn invalid_transition() {
        // Build a new machine to reuse internal parts to build a custom one.
        let machine = Machine::new(&Default::default()).expect("Error building machine");
        let machine: Machine<Effect<Start>, EmptyStack> = Machine {
            state: PhantomData,
            history: PhantomData,
            transaction: transaction::Epsilon,
            //
            transactions: machine.transactions,
            entities: machine.entities,
            triggers: machine.triggers,
        };
        // This is an invalid pullup because the transition history is empty.
        let pull: Result<Machine<Action<Start>, _>, _> = PullupFrom::pullup_from(machine);
        assert!(pull.is_err());
    }
}
