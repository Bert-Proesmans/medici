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
    pub use medici_core::stm::checked::{PullupFrom, PushdownFrom, TransitionFrom};
    pub use medici_core::storage;
    pub use medici_core::service::error::*;
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
    pub use medici_core::function::{Service, Entity, EntityBuilder, Card, CardBuilder, ServiceCompliance};

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

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use failure::{Error, Fail};
    use re_export::*;
    use prelude::*;

    #[test]
    fn failure_derive() {
        let id: usize = 0;
        let error = MissingEntityError(id);
        let fail: &Fail = &MissingEntityError(id);
        let err: Error = MissingEntityError(id).into();
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
