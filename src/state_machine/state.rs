//! Types which encode the states to be used by a state machine.

use medici_core::function::State;
use medici_core::marker::{ActionableMarker, TopLevelMarker, WaitableMarker};
use medici_core::transaction::Epsilon;
// Re-export prefab Timing items
pub use medici_core::prefab::timing::*;

pub mod leaves {
    //! Module defining types representing leaf states of the state machine.

    use value_from_type_macros::value_from_type;

    #[value_from_type(TriggerItem)]
    pub mod triggerable {
        //! All types which can be used to activate triggers awaiting activation.
        //!
        //! A matching [`TriggerItem`] is on of the requirements to activate pending triggers.

        use medici_core::function::State;
        use medici_core::marker::{ActionableMarker, TriggerEnumerator, TriggerableMarker,
                                  WaitableMarker};
        use medici_core::transaction::Epsilon;

        use state_machine::transaction::PrintTransaction;

        // Necessary implementation because value_from_type cannot automatically generate
        // this impl automatically for [`TriggerItem`].
        impl TriggerEnumerator for TriggerItem {}

        /// Wait condition state until the game has been started.
        #[derive(Debug, Clone)]
        pub struct Start();
        impl State for Start {
            type Transaction = Epsilon;
        }

        impl WaitableMarker for Start {}

        /// Wait condition state until the user has provided input.
        #[derive(Debug, Clone)]
        pub struct Input();
        impl State for Input {
            type Transaction = Epsilon;
        }

        impl WaitableMarker for Input {}

        /// Action condition state indicating loading is in progress.
        #[derive(Debug, Clone)]
        pub struct Load();
        impl State for Load {
            type Transaction = Epsilon;
        }

        impl ActionableMarker for Load {}
        impl TriggerableMarker for Load {}

        /// Action condition state indicating printing is in progress.
        #[derive(Debug, Clone)]
        pub struct Print();
        impl State for Print {
            // !-- See below *Transactions --!
            type Transaction = PrintTransaction;
        }

        impl ActionableMarker for Print {}
        impl TriggerableMarker for Print {}

    }

}

///////////////////
// Toplevel WAIT //
///////////////////

/// State indicating a pause until an input event has been generated.
#[derive(Debug, Clone)]
pub struct Wait<W: WaitableMarker>(W);
impl<W> State for Wait<W>
where
    W: WaitableMarker + State,
{
    type Transaction = W::Transaction;
}

impl<W> TopLevelMarker for Wait<W>
where
    W: WaitableMarker,
{
}

/////////////////////
// Toplevel ACTION //
/////////////////////

/// State indicating dynamic execution of the specific action is in progress.
#[derive(Debug, Clone)]
pub struct Action<A: ActionableMarker>(A);
impl<A> State for Action<A>
where
    A: ActionableMarker + State,
{
    type Transaction = A::Transaction;
}

impl<A> TopLevelMarker for Action<A>
where
    A: ActionableMarker,
{
}

///////////////////////
// Toplevel FINISHED //
///////////////////////

/// State indicating finalization of the state machine.
///
/// Finished CAN NOT have any outgoing transitions, since it's intended
/// to be a terminal state.
#[derive(Debug, Clone)]
pub struct Finished();
impl State for Finished {
    type Transaction = Epsilon;
}

impl TopLevelMarker for Finished {}
