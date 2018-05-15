//! Defines the state machine itself.

use std::marker::PhantomData;

use medici_core::ctstack::CTStack;
use medici_core::function::{ServiceCompliance, State, StateContainer};
use medici_core::marker;
use medici_core::service::{EntityService, TriggerService};
use medici_core::storage::TransactionStorage;

use state_machine::state::prelude::*;
use state_machine::transaction::TransactionItem;
use zone::ZoneItem;

use entity::Entity;

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
    X: marker::TopLevel + State + Send,
    CTS: CTStack + Send,
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
    /// Object for manipulating [`Trigger`]s.
    pub triggers: TriggerService<TimingItem, TriggerItem>,
    /// Object for manipulating [`Entity`]s.
    pub entities: EntityService<Entity>,
    /// Storage object allowing [`PushdownInto`] and [`PullupInto`] to store
    /// the [`Transaction`] objects for each state to be re-used.
    pub transactions: TransactionStorage<TransactionItem>,
    // Stub services, these are inaccessable because they only contain owned
    // data. No functionality is defined on them.
    // Build the service to manipulate its data.
    pub(crate) stub_zone: ZoneServiceStub<Entity, ZoneItem>,
}

impl<X, CTS> StateContainer for Machine<X, CTS>
where
    X: marker::TopLevel + State + Send,
    CTS: CTStack + Send,
{
    type State = X;
    type TransitionRecord = CTS;
    type TimingEnum = TimingItem;
    type TriggerEnum = TriggerItem;
}

impl<X, CTS> ServiceCompliance<TriggerService<TimingItem, TriggerItem>> for Machine<X, CTS>
where
    X: marker::TopLevel + State + Send,
    CTS: CTStack + Send,
{
    fn get(&self) -> &TriggerService<TimingItem, TriggerItem> {
        &self.triggers
    }

    fn get_mut(&mut self) -> &mut TriggerService<TimingItem, TriggerItem> {
        &mut self.triggers
    }
}

impl<X, CTS> ServiceCompliance<EntityService<Entity>> for Machine<X, CTS>
where
    X: marker::TopLevel + State + Send,
    CTS: CTStack + Send,
{
    fn get(&self) -> &EntityService<Entity> {
        &self.entities
    }

    fn get_mut(&mut self) -> &mut EntityService<Entity> {
        &mut self.entities
    }
}
