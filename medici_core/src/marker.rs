//! Primitive traits which can be used as constraints by the core components.
//!
//! Marker Traits are usefull because the can be used as generic bounds. This allows
//! for decoupling hidden code from developer created code.
//! Correct understanding of what each trait encompasses is necessary!

/// Types used to transition between state machine States.
pub trait Transaction {}
/// Types which generalize multiple transactions into 1 [`Sized`] structure
/// so the transactions themselves can be safely stored in memory.
pub trait TransactionContainer {}
/// Types which attribute functionality to state machines.
///
/// A Service is kind-of like a Trait (language item), but is used in a dynamic
/// way to quickly de-/construct state machines with various functional methods.
pub trait Service {}

/// (State) Types which are directly contained by the state machine.
///
/// Note: States can be nested!
pub trait TopLevelMarker {}
/// (State) Types which represent a condition for when the state machine itself
/// should resume execution.
///
/// The semantics are limited to the set of input events a user can generate.
pub trait WaitableMarker {}
/// (State) Types which represent a condition for when the state machine itself
/// should resume execution.
///
/// The semantics are limited to the set of action events a user can generate.
pub trait ActionableMarker {}
