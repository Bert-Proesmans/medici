//! Primitive traits which can be used as constraints by the core components.
//!
//! Marker Traits are usefull because the can be used as generic bounds. This allows
//! for decoupling hidden code from developer created code.
//! Correct understanding of what each trait encompasses is necessary!

/// Types used to transition between state machine States.
pub trait TransactionMarker: Copy + 'static {}

/// Types which generalize multiple transactions into 1 [`Sized`] structure
/// so the transactions themselves can be safely stored in memory.
pub trait TransactionContainer {}

/// Types which attribute functionality to state machines.
///
/// A Service is kind-of like a Trait (language item), but is used in a dynamic
/// way to quickly de-/construct state machines with various functional methods.
pub trait Service {}

/// Types which enumerate all possible timings at which moment a trigger can be
/// executed.
///
/// Most of the time these are limited to Pre (before), Peri (during/at), Post (after)
/// a specific trigger.
pub trait TimingEnumerator {}

/// Types which reflect the timing when a [`Trigger`] should be executed.
pub trait TimingMarker {}

/// Types which enumerate all possible triggers which the machine facilitates reacting to.
pub trait TriggerEnumerator {}

/// Types which reflect an event after which functionality awaiting these events
/// will be executed.
pub trait TriggerMarker {}

/// Types which enumerate all known [`Prototype`]s.
pub trait ProtoEnumerator {}

/// Types which attribute functionality to [`Entity`]s within the machine.
pub trait PrototypeMarker {}

/// (State) Types which are directly contained by the state machine.
///
/// Note: States can be nested!
pub trait TopLevelMarker {}

/// (State) Types which represent a condition for when the state machine itself
/// should resume execution.
///
/// The semantics are limited to the set of input types a user can generate.
pub trait WaitableMarker {}

/*
/// (State) Types which represent a condition for when the state machine itself
/// should iterate and evaluate all registered [`Trigger`]s and execute them.
pub trait TriggerableMarker {}
*/

/// (State) Types which represent actions a user want to perform that activate
/// the machine to work out its effects.
///
/// The semantics are limited to the set of choices a player is presented with
/// according to the state of the machine.
pub trait ActionableMarker: TriggerMarker {}
