//! Contains the core functionality items for our system.

pub mod helper;

use marker::{Service, Timing, Trigger};

/// Type that's generally used to identify and order [`Entity`] objects.
///
/// Throughout medici-core it's assumed this type is an alias for a numeric
/// type!
pub type EntityId = usize;

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

/// Trait generalizing over any state that's used to pass into trigger callbacks
/// when trigger conditions are met.
pub trait TriggerState: State {
    /// Encoded type value representing the timing (related to triggers) of the
    /// current state.
    type Timing: Timing;
    /// Encoded type value representing the trigger of the current state.
    type Trigger: Trigger;
}

/// Trait representing an object which properties can be altered dynamically (at runtime).
///
/// # Note
/// This trait MUST ALWAYS be object safe!
pub trait Entity {
    /// Type used to identify an Entity.
    type ID;

    /// Returns the unique identifier of this specific entity.
    fn id(&self) -> Self::ID;
}

/// Trait used to create a new [`Entity`] object.
pub trait EntityBuilder<E: Entity> {
    /// Build a new [`Entity`] with the provided identifier.
    fn new_with_id(id: E::ID) -> E;
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
