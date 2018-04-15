//! (Type) Checked transitions for state machines.

use ctstack::CTStack;
use function::{ServiceCompliance, State, StateContainer};
use marker;
use service::storage::StackStorage;

/// TODO; Document
pub trait PushdownFrom<T, CTS, TTC>
where
    T: StateContainer<TransitionRecord = CTS> + 'static,
    CTS: CTStack,
    TTC: marker::TransactionContainer + 'static,
    Self: StateContainer + ServiceCompliance<StackStorage<TTC>> + 'static,
    Self::State: State + 'static,
    <Self::State as State>::Transaction: marker::Transaction + 'static,
{
    /// TODO; Document
    fn pushdown_from(_: T, _: <Self::State as State>::Transaction) -> Self;
}

/// TODO; Document
pub trait PullupFrom<T, CTS, TTC>
where
	T: StateContainer + ServiceCompliance<StackStorage<TTC>> + 'static,
	CTS: CTStack,
	TTC: marker::TransactionContainer + 'static,
	Self: StateContainer + Sized + 'static,
	Self::State: State + 'static,
	<Self::State as State>::Transaction: marker::Transaction + 'static,
{
	/// TODO; Document
	fn pullup_from(_: T) -> Result<Self, String>;
}
