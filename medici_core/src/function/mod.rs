//! Contains the core functionality items for our system.

pub mod helper;

use marker::Service;

/// Trait generalizing over any structure that could act as a container of states.
///
/// This container of states could be reworded as 'the state machine' itself.
pub trait StateContainer {
    /// Type of the current state held by the state machine.
    type State;
}

/// Trait generalizing over any state that's present in the state machine.
pub trait State {
    /// Type of structure which must be provided when transitioning into the state
    /// represented by the enclosing type.
    type Transaction;
}

/// Trait for implementing a certain service on the state machine.
///
/// Because of this design exactly one object of each service type can be hooked onto
/// the same state machine.
pub trait ServiceCompliance<S>
where
    S: Service,
    Self: StateContainer,
{
    /// Retrieves an immutable reference to service `S`.
    fn get(&self) -> &S;
    /// Retrieves a mutable reference to service `S`.
    fn get_mut(&mut self) -> &mut S;
}
