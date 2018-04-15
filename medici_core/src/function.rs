//! Contains the core functionality items for our system.

use ctstack::CTStack;
use marker;

/// Trait generalizing over any structure that could act as a container of states.
///
/// This container of states could be reworded as 'the state machine' itself.
pub trait StateContainer {
    /// Type of the current state held by the state machine.
    type State: State;
    /// Type of transaction object necessary to transition into the
    /// current state of the machine.
    type Transaction: marker::Transaction = <Self::State as State>::Transaction;
    /// Type which enumerates all possible timings contained by the machine.
    type TimingEnum: marker::TimingEnumerator;
    /// Type which enumerates all possible triggers contained by the machine.
    type TriggerEnum: marker::TriggerEnumerator;
    /// Type representing the stack of types where the container state was
    /// transitioned in a pushdown manner.
    type TransitionRecord: CTStack;
}

/// Trait generalizing over any state that's present in the state machine.
pub trait State {
    /// Type of structure which must be provided when transitioning into the state
    /// represented by the enclosing type.
    type Transaction: marker::Transaction;
}

/// Trait generalizing over any state which is used to bootstrap an execution of triggers.
pub trait EffectState: State {}

/// Trait generalizing over any state that's used to pass into trigger callbacks
/// when trigger conditions are met.
pub trait TriggerState: State {
    /// Encoded type value representing the timing (related to triggers) of the
    /// current state.
    type Timing: marker::Timing;
    /// Encoded type value representing the trigger of the current state.
    type Trigger: marker::Triggerable;
}

/// Types which attribute functionality to state machines.
///
/// A Service is kind-of like a Trait (language item), but is used in a dynamic
/// way to quickly de-/construct state machines with various functional methods.
pub trait Service {
    // Note; It's quite possible this trait will receive methods later on.
}

/// Type that's generally used to identify and order [`Entity`] objects.
///
/// Throughout medici-core it's assumed this type is an alias for a numeric
/// type!
pub type EntityId = usize;

/// Trait representing an object which properties can be altered dynamically (at runtime).
///
/// # Note
/// This trait MUST ALWAYS be object safe!
/// This provides the flexibility to store a bunch of [`Entity`]s into one container.
pub trait Entity {
    /// Type used to identify an Entity.
    type ID: Copy;

    /// Returns the unique identifier of this specific entity.
    fn id(&self) -> Self::ID;
}

/// Trait used to create a new [`Entity`] object.
pub trait EntityBuilder<E: Entity> {
    /// Build a new [`Entity`] with the provided identifier.
    fn new_with_id(id: E::ID) -> E;
}

/// Type thet's generally used to identify and order [`Card`] objects.
///
/// Throughout medici-core it's assumed this type is an alias for a numeric
/// type!
pub type CardId = usize;

/// Trait representing an actual game card.
///
/// A card is an [`Entity`] but it's usage is semantically disjunct enough to warrant
/// a seperate type.
///
/// # Note
/// This trait MUST ALWAYS be object safe!
/// This provides the flexibility to store a bunch of [`Card`]s into one container.
pub trait Card {
    /// Type used to identify a Card.
    ///
    /// # Note
    /// Do NOT confuse this UID with [`Entity::ID`]!
    ///     - UID is constant, global ID
    ///     - Entity::ID is a local ID that's only valid for the lifetime
    ///     of the state-machine containing that entity object.
    type UID: Copy;
    /// All timing types this card holds listeners for.
    type TimingEnum: marker::TimingEnumerator;
    /// All trigger types this card holds listeners for.
    type TriggerEnum: marker::TriggerEnumerator;

    /// Returns the globally unique identifier of this specific card.
    fn uid(&self) -> Self::UID;
}

/// Trait used to create a new [`Card`] object.
pub trait CardBuilder<C: Card> {}

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