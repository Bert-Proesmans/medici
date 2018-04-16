//! Module for leaf state types.
//!
//! Leaf types are the innermost types for any given state
//! of the machine.

use value_from_type_macros::value_from_type;

use medici_core::function::State;
use medici_core::marker;
/// Export the prefab timing items.
pub use medici_core::prefab::timing::{Peri, Post, Pre, TimingItem};
use medici_core::prefab::transaction::Epsilon;

use state_machine::transaction::PrintTransaction;

/* All imports are grouped above so we can simply import all by using
 `super::*` in child modules.
 */

pub mod triggerable {
    #![value_from_type(TriggerItem)]
    //! All types which can be used to activate triggers awaiting activation.
    //!
    //! A matching [`TriggerItem`] is on of the requirements to activate pending triggers.

    use super::*;

    // Necessary implementation because value_from_type cannot automatically generate
    // this impl automatically for [`TriggerItem`].
    impl marker::TriggerEnumerator for TriggerItem {}

    /// Wait condition state until the game has been started.
    #[derive(Debug, Clone)]
    pub struct Start();
    impl State for Start {
        type Transaction = Epsilon;
    }
    impl marker::Waitable for Start {}
    impl marker::Triggerable for Start {}
    impl marker::Actionable for Start {}

    /// Wait condition state until the user has provided input.
    #[derive(Debug, Clone)]
    pub struct Input();
    impl State for Input {
        type Transaction = Epsilon;
    }
    impl marker::Waitable for Input {}

    /// Action condition state until the user has provided input.
    #[derive(Debug, Clone)]
    pub struct EndTurn();
    impl State for EndTurn {
        type Transaction = Epsilon;
    }
    impl marker::Actionable for EndTurn {}
    impl marker::Triggerable for EndTurn {}

    /// Action condition state indicating a card will be played.
    #[derive(Debug, Clone)]
    pub struct PlayCard();
    impl State for PlayCard {
        type Transaction = Epsilon;
    }
    impl marker::Actionable for PlayCard {}
    impl marker::Triggerable for PlayCard {}

    /// Action condition state indicating an attack will commence.
    #[derive(Debug, Clone)]
    pub struct Attack();
    impl State for Attack {
        type Transaction = Epsilon;
    }
    impl marker::Actionable for Attack {}
    impl marker::Triggerable for Attack {}

    /// Trigger condition for taken damage.
    #[derive(Debug, Clone)]
    pub struct Damage();
    impl State for Damage {
        // !-- See below *Transactions --!
        type Transaction = PrintTransaction;
    }
    impl marker::Triggerable for Damage {}
}
