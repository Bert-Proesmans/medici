//! (Type) Checked transitions for state machines.

use ctstack::CTStack;
use error::MachineError;
use function::{ServiceCompliance, State, StateContainer};
use marker;
use service::storage::StackStorage;

// Re-export traits and implementations from the submodule below.
pub use self::usability_impl::{PullupInto, PushdownInto, TransitionInto};

/// Trait defining the contract for one-way transitions of the state machine.
/// This kind of transition preserves the state change history.
///
/// A state machine is said to transition from A into B when the current state is A,
/// a [`Transaction`] object for state B is provided and the following transition is
/// valid [A -> B].
pub trait TransitionFrom<T, CTS>
where
    T: StateContainer<TransitionRecord = CTS> + 'static,
    CTS: CTStack + 'static,
    Self: StateContainer<TransitionRecord = CTS> + 'static,
    Self::State: State + 'static,
    <Self::State as State>::Transaction: marker::Transaction + 'static,
{
    /// Transition from the provided state into the implementing state.
    fn transition_from(_: T, _: <Self::State as State>::Transaction) -> Self;
}

/// Trait defining the contract for two-way transitions of the state machine.
/// The transition this trait defines is meant as a temporary one where the control flow
/// will result in the state machine eventually performs a [`PullupFrom`] transition.
/// Effectively reversing the Pushdown.
/// The Transaction object of the outgoing state is stored for re-use.
/// This kind of transition will write state change history into the compile-time stack ([`CTStack`]).
///
/// A state machine is said to pushdown from A into B when the current state is A,
/// a [`Transaction`] object for state B is provided and the following transition is
/// valid [A -> B].
///
/// # Note
/// [`PushdownFrom`] is designed to be used together with [`PullupFrom`] because both operations
/// store and reload the [`Transaction`] object respectively.
///
/// # See also
/// [`PullupFrom`]
pub trait PushdownFrom<T, CTS, TTC>
where
    T: StateContainer<TransitionRecord = <CTS as CTStack>::Tail> + 'static,
    CTS: CTStack<Head = <Self as StateContainer>::State> + 'static,
    TTC: marker::TransactionContainer + 'static,
    Self: StateContainer + ServiceCompliance<StackStorage<TTC>> + 'static,
    Self::State: State + 'static,
    <Self::State as State>::Transaction: marker::Transaction + 'static,
{
    /// Transition from the provided state into the implementing state.
    fn pushdown_from(_: T, _: <Self::State as State>::Transaction) -> Self;
}

/// Trait defining the contract for two-way transitions of the state machine.
/// The Transaction object of the outgoing state is stored for re-use.
/// This kind of transition will write state change history into the compile-time stack ([`CTStack`]).
///
/// A state machine is said to pullup from B into A when the current state is B,
/// a [`Transaction`] object for state A is provided and the following transition is
/// valid [A <- B].
///
/// # Note
/// [`PushdownFrom`] is designed to be used together with [`PullupFrom`] because both operations
/// store and reload the [`Transaction`] object respectively.
///
/// # See also
/// [`PushdownFrom`]
pub trait PullupFrom<T, CTS, TTC>
where
    T: StateContainer<TransitionRecord = CTS> + ServiceCompliance<StackStorage<TTC>> + 'static,
    CTS: CTStack + 'static,
    TTC: marker::TransactionContainer + 'static,
    Self: StateContainer<TransitionRecord = <CTS as CTStack>::Tail> + Sized + 'static,
    Self::State: State + 'static,
    <Self::State as State>::Transaction: marker::Transaction + 'static,
{
    /// Transition from the provided state into the implementing state.
    ///
    /// # Errors
    /// Invalid transitions are blocked according to the state history log.
    /// There is also a runtime check which tests the type of the required transaction object
    /// against the last one stored (presumably by [`PushdownFrom`]) on a stack owned by the machine.
    /// If a [`Transaction`] type is popped that's not compatible with the incoming state an error
    /// is thrown.
    fn pullup_from(_: T) -> Result<Self, MachineError>;
}

/// All implementations to make using the above defined traits straight-forward to use.
mod usability_impl {
    use super::*;

    /// Syntax simplifying trait in accordance to [`TransitionFrom`].
    pub trait TransitionInto<T, CTS>
    where
        T: StateContainer<TransitionRecord = CTS> + 'static,
        CTS: CTStack + 'static,
        Self: StateContainer<TransitionRecord = CTS> + 'static,
        T::State: State + 'static,
        <T::State as State>::Transaction: marker::Transaction + 'static,
    {
        /// In accordance with [`TransitionFrom::transition_from`].
        fn transition(self, _: <T::State as State>::Transaction) -> T;
    }

    impl<T, CTS, S> TransitionInto<T, CTS> for S
    where
        CTS: CTStack + 'static,
        S: StateContainer<TransitionRecord = CTS> + 'static,
        T: TransitionFrom<S, CTS> + StateContainer<TransitionRecord = CTS> + 'static,
        T::State: State + 'static,
        <T::State as State>::Transaction: marker::Transaction + 'static,
    {
        fn transition(self, t: <T::State as State>::Transaction) -> T {
            T::transition_from(self, t)
        }
    }

    /// Syntax simplifying trait in accordance to [`PushdownFrom`].
    pub trait PushdownInto<T, CTS, TTC>
    where
        T: StateContainer + ServiceCompliance<StackStorage<TTC>> + 'static,
        T::State: State + 'static,
        <T::State as State>::Transaction: marker::Transaction + 'static,
        CTS: CTStack<Head = <T as StateContainer>::State> + 'static,
        TTC: marker::TransactionContainer + 'static,
        Self: StateContainer<TransitionRecord = <CTS as CTStack>::Tail> + 'static,
    {
        /// In accordance with [`PushdownFrom::pushdown_from`].
        fn pushdown(self, _: <T::State as State>::Transaction) -> T;
    }

    impl<T, CTS, TTC, S> PushdownInto<T, CTS, TTC> for S
    where
        S: StateContainer<TransitionRecord = <CTS as CTStack>::Tail> + 'static,
        CTS: CTStack<Head = <T as StateContainer>::State> + 'static,
        TTC: marker::TransactionContainer + 'static,
        T: PushdownFrom<S, CTS, TTC> + StateContainer + 'static,
        T::State: State + 'static,
        <T::State as State>::Transaction: marker::Transaction + 'static,
    {
        fn pushdown(self, t: <T::State as State>::Transaction) -> T {
            T::pushdown_from(self, t)
        }
    }

    /// Syntax simplifying trait in accordance to [`PullupFrom`].
    pub trait PullupInto<T, CTS, TTC>
    where
        T: StateContainer<TransitionRecord = <CTS as CTStack>::Tail> + Sized + 'static,
        T::State: State + 'static,
        <T::State as State>::Transaction: marker::Transaction + 'static,
        CTS: CTStack + 'static,
        TTC: marker::TransactionContainer + 'static,
        Self:
            StateContainer<TransitionRecord = CTS> + ServiceCompliance<StackStorage<TTC>> + 'static,
    {
        /// In accordance with [`PullupFrom::pullup_from`].
        fn pullup(self) -> Result<T, MachineError>;
    }

    impl<T, CTS, TTC, S> PullupInto<T, CTS, TTC> for S
    where
        T: PullupFrom<S, CTS, TTC>
            + StateContainer<TransitionRecord = <CTS as CTStack>::Tail>
            + 'static,
        T::State: State + 'static,
        <T::State as State>::Transaction: marker::Transaction + 'static,
        CTS: CTStack + 'static,
        TTC: marker::TransactionContainer + 'static,
        Self:
            StateContainer<TransitionRecord = CTS> + ServiceCompliance<StackStorage<TTC>> + 'static,
    {
        fn pullup(self) -> Result<T, MachineError> {
            T::pullup_from(self)
        }
    }
}
