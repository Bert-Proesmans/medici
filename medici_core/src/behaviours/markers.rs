//! Traits to be used as constraints by the core components.
//!
//! Marker Traits are useful because they can be used as generic bounds. This allows
//! for decoupling developer-hidden code from game behaviour code.
//! Correct understanding of what each trait encompasses is necessary!

/// Supertrait of states that aren't nested within other states to be useful.
/// Toplevel state types are owned directly by the state machine.
///
/// Note: States can be nested!
pub trait TopLevelState {}

/// Types used to transition between state machine States.
pub trait Transaction: Copy + 'static {}

/// Types which generalize multiple transactions into 1 [`Sized`] structure
/// so the transactions themselves can be safely stored in memory.
pub trait TransactionContainer {}

/// Types which enumerate all possible timings at which moment a trigger can be
/// executed.
///
/// Most of the time these are limited to Pre (before), Peri (during/at), Post (after)
/// a specific trigger.
pub trait TimingComparator: Eq {}

/// Types which enumerate all possible triggers which the machine facilitates reacting to.
pub trait TriggerComparator: Eq {}
