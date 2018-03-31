//! State machine implementation for the game: Goose game.

pub mod marker;
pub mod state;
pub mod transaction;
pub mod transitions;

use std::marker::PhantomData;

use medici_core::function::{ServiceCompliance, State, StateContainer};
use medici_core::marker::TopLevelMarker;
use medici_core::service::storage::StackStorage;

use self::transaction::*;
use self::state::*;

/// The state machine.
///
/// The developer is encouraged to design this structure in any desired
/// way by storing services into it's members.
/// Each state machine MUST have a `state` and `transaction` field AT
/// MINIMUM.
#[derive(Debug, Clone)]
pub struct Machine<X>
where
    X: TopLevelMarker + State,
{
    /* Absolute minimum variables */
    /// Field to encode the current state of the machine.
    ///
    /// This field is present to utilize the type system to statically verify
    /// legal transitions of the machine. This field has no (/zero) size
    /// at runtime.
    state: PhantomData<X>,
    /// Field to store the provided Transaction object as rquired by the
    /// current state.
    pub transaction: X::Transaction,

    /* Optionals */
    /// Stack storage service to allow PushDown and Pullup behaviour to be
    /// implemented.
    storage: StackStorage<TransactionItem>,
}

impl Machine<Wait<Start>> 
{
    /// Creates a new state machine ready to be started.
    pub fn new() -> Self {
        Self {
            state: PhantomData,
            transaction: Epsilon,
            storage: StackStorage::new(),
        }
    }
}

impl<X> StateContainer for Machine<X>
where
    X: TopLevelMarker + State,
{
    type State = X;
}

impl<X> ServiceCompliance<StackStorage<TransactionItem>> for Machine<X>
where
    X: TopLevelMarker + State,
{
    fn get(&self) -> &StackStorage<TransactionItem> {
        &self.storage
    }

    fn get_mut(&mut self) -> &mut StackStorage<TransactionItem> {
        &mut self.storage
    }
}
