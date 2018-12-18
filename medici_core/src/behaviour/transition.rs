//! (Type) Checked transitions for state machines.

use crate::behaviour::function::{State, StateMachine};
use crate::behaviour::marker;
use crate::compile_tools as CT;

/// Trait defining the contract for one-way transitions of a state machine.
/// This kind of transition preserves the current state change history, no aditions are made.
///
/// A state machine is said to transition from A into B if the following conditions are met:
/// * the current state is A;
/// * A [`Transaction`] object for state B is provided;
/// * The following transition is valid [A -> B].
pub trait TransitionFrom<T>
where
    T: StateMachine,
    Self: StateMachine,
    Self::State: State,
    <Self::State as State>::Transaction: marker::Transaction,
{
    /// Transition from the provided state into the implementing state.
    fn transition_from(_: T, _: <Self::State as State>::Transaction) -> Self;
}

/// Trait defining one part of two-way transitions of the state machine.
/// A pushdown operation is used where next transitions will eventually loop around and perform an
/// equivalent [`PullupFrom`] transition.
///
/// The amount of [`PullupFrom`] and [`PushdownFrom`] transitions must always match! The compile time
/// stack guarantees that no out-of-order transition happens or a Pullup without a PushDown.
/// The Transaction object that was used to enter the current state (before pushing down) is stored and
/// will be re-used when a pullup transition happens.
///
/// A state machine is said to pushdown from A into B if the following conditions are met:
/// * the current state is A;
/// * A [`Transaction`] object for state B is provided;
/// * The following transition is valid [A -> B].
///
/// # Note
/// [`PushdownFrom`] is designed to be used together with [`PullupFrom`] because both operations
/// store and reload the [`Transaction`] object respectively (if applicable).
///
/// # See also
/// [`PullupFrom`]
pub trait PushdownFrom<T, CTS>
where
    T: StateMachine<TransitionRecord = <CTS as CT::Stack>::Tail>,
    CTS: CT::Stack<Head = <Self as StateMachine>::State>,
    Self: StateMachine,
    Self::State: State,
    <Self::State as State>::Transaction: marker::Transaction,
{
    /// Transition from the provided state into the implementing state.
    fn pushdown_from(_: T, _: <Self::State as State>::Transaction) -> Self;
}

/// Trait defining one part of two-way transitions of the state machine.
/// A pullup operation is used within a looping transition chain. The exact point of usage
/// is at a point where an earlier matching [`PushdownFrom`] transition happened.
///
/// A state machine is said to pullup from B into A if the following conditions are met:
/// * the current state is B;
/// * A [`Transaction`] object for state A is provided;
/// The following transition is valid [A <- B].
///
/// # Note
/// [`PushdownFrom`] is designed to be used together with [`PullupFrom`] because both operations
/// store and reload the [`Transaction`] object respectively.
///
/// # See also
/// [`PushdownFrom`]
pub trait PullupFrom<T, CTS>
where
    T: StateMachine<TransitionRecord = CTS>,
    CTS: CT::Stack,
    Self: StateMachine<TransitionRecord = <CTS as CT::Stack>::Tail>,
    Self::State: State,
    <Self::State as State>::Transaction: marker::Transaction,
{
    /// Transition from the provided state into the implementing state.
    fn pullup_from(_: T, _: <Self::State as State>::Transaction) -> Self;
}
