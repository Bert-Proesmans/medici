//! (Type) Unchecked transitions for state machines.

use error::MachineError;
use function::{ServiceCompliance, State, StateContainer};
use marker;
use service::storage::StackStorage;

/// Types, state machines residing in a certain state, which transform one-sided
/// into a next Type.
///
/// A state machine is said to transition from A into B when the current state is A,
/// a Transaction object for state B is provided and the following transition is
/// valid [A -> B].
pub trait TransitionFrom<T>
where
    T: StateContainer + 'static,
    Self: StateContainer + 'static,
    Self::State: State + 'static,
    <Self::State as State>::Transaction: marker::Transaction + 'static,
{
    /// Transition from the provided state into the implementing state.
    fn transition_from(_: T, _: <Self::State as State>::Transaction) -> Self;
}

/// Syntax simplifying trait in accordance to [`TransitionFrom`].
pub trait TransitionInto<T>
where
    T: StateContainer + 'static,
    Self: StateContainer + 'static,
    T::State: State + 'static,
    <T::State as State>::Transaction: marker::Transaction + 'static,
{
    /// Transition from Self into the desired state.
    fn transition(self, _: <T::State as State>::Transaction) -> T;
}

impl<T, S> TransitionInto<T> for S
where
    S: StateContainer + 'static,
    T: TransitionFrom<S> + StateContainer,
    T::State: State + 'static,
    <T::State as State>::Transaction: marker::Transaction + 'static,
{
    fn transition(self, t: <T::State as State>::Transaction) -> T {
        // self is of type S.
        T::transition_from(self, t)
    }
}

/// Types, state machines residing in a certain state, which transform one-sided
/// into a next Type. The Transaction object of the previous state is stored for re-use.
///
/// [`PushdownFrom`] is designed to be used together with [`PullupFrom`] because one part of
/// it's functionality is to store the previous state's Transaction onto a stack.
/// Generally each [`PushDown`] must be followed with a matching Pullup operation to
/// correctly push onto and pop from the stackstorage.
///
/// A state machine is said to pushdown from A into B when the current state is A,
/// a Transaction object for state B is provided and the following transition is
/// valid [A -> B].
pub trait PushdownFrom<T, TTC>
where
    TTC: marker::TransactionContainer + 'static,
    T: StateContainer + 'static,
    Self: StateContainer + ServiceCompliance<StackStorage<TTC>> + 'static,
    Self::State: State + 'static,
    <Self::State as State>::Transaction: marker::Transaction + 'static,
{
    /// Transition from the provided state into the implementing state.
    fn pushdown_from(_: T, _: <Self::State as State>::Transaction) -> Self;
}

/// Syntax simplifying trait in accordance to [`PushdownFrom`].
pub trait PushdownInto<T, TTC>
where
    TTC: marker::TransactionContainer + 'static,
    T: StateContainer + 'static,
    T::State: State + 'static,
    <T::State as State>::Transaction: marker::Transaction + 'static,
    Self: StateContainer + 'static,
{
    /// Transition from Self into the desired state.
    fn pushdown(self, _: <T::State as State>::Transaction) -> T;
}

impl<T, TTC, S> PushdownInto<T, TTC> for S
where
    S: StateContainer + 'static,
    TTC: marker::TransactionContainer + 'static,
    T: PushdownFrom<S, TTC> + StateContainer + 'static,
    T::State: State + 'static,
    <T::State as State>::Transaction: marker::Transaction + 'static,
{
    fn pushdown(self, t: <T::State as State>::Transaction) -> T {
        // self is of type S.
        T::pushdown_from(self, t)
    }
}

/// Types, state machines residing in a certain state, which transform one-sided
/// into a previous Type. The Transaction object of the next state is loaded
/// and restored.
///
/// [`PullupFrom`] is designed to be used together with [`PushdownFrom`] because one part of
/// it's functionality is to restore the next state's Transaction from a stack.
/// Generally each [`PushDown`] must be followed with a matching Pullup operation to
/// correctly push onto and pop from the stackstorage.
///
/// A state machine is said to pullup from B into A when the current state is B
/// and the following transition is valid [A <- B].
pub trait PullupFrom<T, TTC>
where
    TTC: marker::TransactionContainer + 'static,
    T: StateContainer + ServiceCompliance<StackStorage<TTC>> + 'static,
    Self: StateContainer + Sized + 'static,
    Self::State: State + 'static,
    <Self::State as State>::Transaction: marker::Transaction + 'static,
{
    /// Transition from the provided state into the implementing state.
    ///
    /// # Errors
    /// There is a check at runtime which prevents a Pullup transition if it doesn't match
    /// the correct PushDown transition in a First In, Last Out (FILO) manner.
    /// Note: This part CANNOT be statically verified as far as I know?
    fn pullup_from(_: T) -> Result<Self, MachineError>;
}

/// Syntax sumplifying trait in accordance to [`PullupFrom`].
pub trait PullupInto<T, TTC>
where
    TTC: marker::TransactionContainer + 'static,
    T: StateContainer + 'static,
    T::State: State + 'static,
    <T::State as State>::Transaction: marker::Transaction + 'static,
    Self: StateContainer + ServiceCompliance<StackStorage<TTC>> + Sized + 'static,
{
    /// Transition from Self into the desired state.
    fn pullup(self) -> Result<T, MachineError>;
}

impl<T, TTC, S> PullupInto<T, TTC> for S
where
    S: StateContainer + ServiceCompliance<StackStorage<TTC>> + 'static,
    TTC: marker::TransactionContainer + 'static,
    T: PullupFrom<S, TTC> + StateContainer + 'static,
    T::State: State + 'static,
    <T::State as State>::Transaction: marker::Transaction + 'static,
{
    fn pullup(self) -> Result<T, MachineError> {
        // self if of type S.
        T::pullup_from(self)
    }
}
