//! Defines the state machine itself.

use std::marker::PhantomData;

use medici_core::function::{ServiceCompliance, State, StateContainer};
use medici_core::marker::TopLevelMarker;
use medici_core::service::storage::{EntityStorage, StackStorage};
use medici_core::service::trigger::TriggerService;

use state_machine::state::leaf::triggerable::TriggerItem;
use state_machine::state::leaf::TimingItem;
use state_machine::transaction::TransactionItem;

use implementation::entity::Entity;

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
    pub(crate) state: PhantomData<X>,
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

impl<X> StateContainer for Machine<X>
where
    X: TopLevelMarker + State,
{
    type State = X;
    type TimingEnum = TimingItem;
    type TriggerEnum = TriggerItem;
}

impl<X> ServiceCompliance<StackStorage<TransactionItem>> for Machine<X>
where
    X: TopLevelMarker + State,
{
    fn get(&self) -> &StackStorage<TransactionItem> {
        &self.transactions
    }

    fn get_mut(&mut self) -> &mut StackStorage<TransactionItem> {
        &mut self.transactions
    }
}

impl<X> ServiceCompliance<TriggerService<TimingItem, TriggerItem>> for Machine<X>
where
    X: TopLevelMarker + State,
{
    fn get(&self) -> &TriggerService<TimingItem, TriggerItem> {
        &self.triggers
    }

    fn get_mut(&mut self) -> &mut TriggerService<TimingItem, TriggerItem> {
        &mut self.triggers
    }
}

impl<X> ServiceCompliance<EntityStorage<Entity>> for Machine<X>
where
    X: TopLevelMarker + State,
{
    fn get(&self) -> &EntityStorage<Entity> {
        &self.entities
    }

    fn get_mut(&mut self) -> &mut EntityStorage<Entity> {
        &mut self.entities
    }
}
