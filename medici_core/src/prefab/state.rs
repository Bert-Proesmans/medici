//! Module containing often used state types.

use function::{EffectState, State, TriggerState};
use marker::{ActionableMarker, TimingMarker, TopLevelMarker, TriggerMarker, WaitableMarker};
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

/// State indicating a pause until an input event has been generated.
#[derive(Debug, Clone)]
pub struct Wait<W: WaitableMarker>(W);
impl<W> State for Wait<W>
where
    W: WaitableMarker + State,
{
    type Transaction = <W as State>::Transaction;
}

impl<W> TopLevelMarker for Wait<W>
where
    W: WaitableMarker,
{
}

/// State indicating dynamic execution of the specific action is in progress.
#[derive(Debug, Clone)]
pub struct Action<A: ActionableMarker>(A);
impl<A> State for Action<A>
where
    A: ActionableMarker + State,
{
    type Transaction = <A as State>::Transaction;
}

impl<A> TopLevelMarker for Action<A>
where
    A: ActionableMarker,
{
}

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

#[derive(Debug, Clone)]
/// First state used to execute an effect chain caused by the substate ([`Actionable`]).
pub struct Effect<A: ActionableMarker>(A);
impl<A> State for Effect<A>
where
    A: ActionableMarker + State,
{
    type Transaction = <A as State>::Transaction;
}

impl<A> TopLevelMarker for Effect<A>
where
    A: ActionableMarker,
{
}

#[derive(Debug, Clone)]
/// Chained effect caused by an specific [`Actionable`].
///
/// # Note
/// [`State`] is implemented using the transaction type of [`Trigger`].
/// This is because we assume [`Timing`] will always have an irrelevant (epsilon) [`Transaction`].
pub struct RecurseEffect<TM: TimingMarker, TR: TriggerMarker>(TM, TR);
impl<TM, TR> State for RecurseEffect<TM, TR>
where
    TM: TimingMarker + State,
    TR: TriggerMarker + State,
{
    type Transaction = <TR as State>::Transaction;
}

impl<TM, TR> EffectState for RecurseEffect<TM, TR>
where
    TM: TimingMarker + State,
    TR: TriggerMarker + State,
{
}

impl<TM, TR> TopLevelMarker for RecurseEffect<TM, TR>
where
    TM: TimingMarker + State,
    TR: TriggerMarker + State,
{
}

#[derive(Debug, Clone)]
/// Specific state where [`Entity`] death processing is triggered.
///
/// # Note
/// [`State`] is implemented using the transaction type of [`Trigger`].
/// This is because we assume [`Timing`] will always have an irrelevant (epsilon) [`Transaction`].
pub struct DeathEffect<TM: TimingMarker, TR: TriggerMarker>(TM, TR);
impl<TM, TR> State for DeathEffect<TM, TR>
where
    TM: TimingMarker + State,
    TR: TriggerMarker + State,
{
    type Transaction = <TR as State>::Transaction;
}

impl<TM, TR> EffectState for DeathEffect<TM, TR>
where
    TM: TimingMarker + State,
    TR: TriggerMarker + State,
{
}

impl<TM, TR> TopLevelMarker for DeathEffect<TM, TR>
where
    TM: TimingMarker + State,
    TR: TriggerMarker + State,
{
}

#[derive(Debug, Clone)]
/// Exact state used to execute effects.
///
/// This state is reached as direct or indirect consequence of player decisions.
///
/// # Note
/// [`State`] is implemented using the transaction type of [`Trigger`].
/// This is because we assume [`Timing`] will always have an irrelevant (epsilon) [`Transaction`].
pub struct Trigger<TM: TimingMarker, TR: TriggerMarker>(TM, TR);
impl<TM, TR> State for Trigger<TM, TR>
where
    TM: TimingMarker + State,
    TR: TriggerMarker + State,
{
    type Transaction = <TR as State>::Transaction;
}

impl<TM, TR> TriggerState for Trigger<TM, TR>
where
    TM: TimingMarker + State,
    TR: TriggerMarker + State,
{
    type Timing = TM;
    type Trigger = TR;
}

impl<TM, TR> TopLevelMarker for Trigger<TM, TR>
where
    TM: TimingMarker + State,
    TR: TriggerMarker + State,
{
}
