//! Defines the state machine itself.

use std::marker::PhantomData;

use medici_core::ctstack::CTStack;
use medici_core::function::{ServiceCompliance, State, StateContainer};
use medici_core::marker;
use medici_core::service::storage::{EntityStorage, StackStorage};
use medici_core::service::trigger::TriggerService;

use state_machine::state::prelude::*;
use state_machine::transaction::TransactionItem;

use implementation::entity::Entity;

/// The state machine.
///
/// The developer is encouraged to design this structure in any desired
/// way by storing services into it's members.
/// Each state machine MUST have a `state` and `transaction` field AT
/// MINIMUM.
#[derive(Debug, Clone)]
pub struct Machine<X, CTS>
where
    X: marker::TopLevel + State,
    CTS: CTStack,
{
    /* Absolute minimum variables */
    /// Field to encode the current state of the machine.
    ///
    /// This field is present to utilize the type system to statically verify
    /// legal transitions of the machine. This field has no (/zero) size
    /// at runtime.
    pub(crate) state: PhantomData<X>,
    /// Field to encode the transition history of the machine.
    pub(crate) history: PhantomData<CTS>,
    /// Field to store the provided Transaction object as rquired by the
    /// current state.
    pub transaction: X::Transaction,

    /* Optionals */
    /// Stack storage service to allow PushDown and Pullup behaviour to be
    /// implemented.
    pub transactions: StackStorage<TransactionItem>,
    /// Entities handler.
    pub entities: EntityStorage<Entity>,
    /// Trigger handler.
    pub triggers: TriggerService<TimingItem, TriggerItem>,
}

impl<X, CTS> StateContainer for Machine<X, CTS>
where
    X: marker::TopLevel + State,
    CTS: CTStack,
{
    type State = X;
    type TransitionRecord = CTS;
    type TimingEnum = TimingItem;
    type TriggerEnum = TriggerItem;
}

impl<X, CTS> ServiceCompliance<StackStorage<TransactionItem>> for Machine<X, CTS>
where
    X: marker::TopLevel + State,
    CTS: CTStack,
{
    fn get(&self) -> &StackStorage<TransactionItem> {
        &self.transactions
    }

    fn get_mut(&mut self) -> &mut StackStorage<TransactionItem> {
        &mut self.transactions
    }
}

impl<X, CTS> ServiceCompliance<TriggerService<TimingItem, TriggerItem>> for Machine<X, CTS>
where
    X: marker::TopLevel + State,
    CTS: CTStack,
{
    fn get(&self) -> &TriggerService<TimingItem, TriggerItem> {
        &self.triggers
    }

    fn get_mut(&mut self) -> &mut TriggerService<TimingItem, TriggerItem> {
        &mut self.triggers
    }
}

impl<X, CTS> ServiceCompliance<EntityStorage<Entity>> for Machine<X, CTS>
where
    X: marker::TopLevel + State,
    CTS: CTStack,
{
    fn get(&self) -> &EntityStorage<Entity> {
        &self.entities
    }

    fn get_mut(&mut self) -> &mut EntityStorage<Entity> {
        &mut self.entities
    }
}
