//! (Type) Checked transitions for state machines.

use crate::behaviours::functions::{MachineContainer, MachineWrapper, State, StateMachine};
use crate::behaviours::markers;
use crate::compile_tools as ct;

/// Trait defining the contract for one-way transitions of a state machine.
/// This kind of transition preserves the current state change history, no aditions are made.
///
/// A state machine is said to transition from A into B if the following conditions are met:
/// * the current state is A;
/// * A [`Transaction`] object for state B is provided;
/// * The following transition is valid [A -> B].
pub trait Transition<Previous, Next, W>
where
    Next: StateMachine,
    Next::State: State,
    <Next::State as State>::Transaction: markers::Transaction,
    W: MachineWrapper<Next>,
{
    /// Transition from the provided state into the implementing state.
    fn transition(
        self,
        _: <Next::State as State>::Transaction,
    ) -> <W as MachineWrapper<Next>>::Output;
}

/// Trait defining one part of two-way transitions of the state machine.
/// A pushdown operation is used where next transitions will eventually loop around and perform an
/// equivalent [`Pullup`] transition.
///
/// The amount of [`Pullup`] and [`Pushdown`] transitions must always match! The compile time
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
/// [`Pushdown`] is designed to be used together with [`Pullup`] because both operations
/// store and reload the [`Transaction`] object respectively (if applicable).
///
/// # See also
/// [`Pullup`]
pub trait Pushdown<Previous, Next, CTS, W>
where
    Next: StateMachine<TransitionRecord = CTS>,
    Next::State: State,
    Previous: StateMachine<TransitionRecord = CTS::Tail>,

    W: MachineWrapper<Next>,
    CTS: ct::Stack<Head = <Next as StateMachine>::State>,
{
    /// Transition from the provided state into the implementing state.
    fn pushdown(
        self,
        _: <Next::State as State>::Transaction,
    ) -> <W as MachineWrapper<Next>>::Output;
}

/// Trait defining one part of two-way transitions of the state machine.
/// A pullup operation is used within a looping transition chain. The exact point of usage
/// is at a point where an earlier matching [`Pushdown`] transition happened.
///
/// A state machine is said to pullup from B into A if the following conditions are met:
/// * the current state is B;
/// * A [`Transaction`] object for state A is provided;
/// The following transition is valid [A <- B].
///
/// # Note
/// [`Pushdown`] is designed to be used together with [`Pullup`] because both operations
/// store and reload the [`Transaction`] object respectively.
///
/// # See also
/// [`Pushdown`]
pub trait Pullup<Previous, Next, CTS, W>
where
    Next: StateMachine<TransitionRecord = <CTS as ct::Stack>::Tail>,
    Next::State: State,
    Previous: StateMachine<TransitionRecord = CTS>,

    W: MachineWrapper<Next>,
    CTS: ct::Stack,
{
    /// Transition from the provided state into the implementing state.
    fn pullup(self, _: <Next::State as State>::Transaction) -> <W as MachineWrapper<Next>>::Output;
}
