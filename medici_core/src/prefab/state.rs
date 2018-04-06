//! Module containing often used state types.

use function::State;
use marker::{ActionableMarker, TopLevelMarker, WaitableMarker};
use prefab::transaction::Epsilon;

/* Clone implementation on states is necessary because the auto-derive
Clone forces any type within the state machine to implement Clone.
This is the cost of the automatic trivial implementation, which we could
manually implement but that's a hassle.
Adding a trivial Clone implementation is a no-brainer since the states 
themselves hold no data.

Cloning the state machine is necessary for duplication and error reporting
to work properly.
*/

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
