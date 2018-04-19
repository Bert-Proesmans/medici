//! Defines the state machine itself.

use std::marker::PhantomData;

use medici_core::ctstack::CTStack;
use medici_core::function::{ServiceCompliance, State, StateContainer};
use medici_core::marker;
use medici_core::service::trigger::TriggerService;
use medici_core::storage::{EntityStorage, StackStorage};

use state_machine::state::prelude::*;
use state_machine::transaction::TransactionItem;

use entity::{Entity, Zones};

/// The state machine.
///
/// The developer is encouraged to alter this contents of this structure to fit his
/// use case.
/// Each state machine MUST have a `state`, `history` and `transaction` field AT
/// MINIMUM.
///
/// # Safety
/// Fields `state` and `history` are not accessible because altering them would cause
/// the state transition and -history system to fall apart.
/// Any code that has valid reason to alter these fields should be defined within this
/// crate.
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
    pub entities: EntityStorage<Entity, Zones>,
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

impl<X, CTS> ServiceCompliance<EntityStorage<Entity, Zones>> for Machine<X, CTS>
where
    X: marker::TopLevel + State,
    CTS: CTStack,
{
    fn get(&self) -> &EntityStorage<Entity, Zones> {
        &self.entities
    }

    fn get_mut(&mut self) -> &mut EntityStorage<Entity, Zones> {
        &mut self.entities
    }
}
