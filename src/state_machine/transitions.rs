//! Defines all transitions within our state machine.

use std::marker::PhantomData;

use medici_core::error::{ErrorKind, MachineError, SnapshottedErrorExt};
use medici_core::function::helper::{pack_transaction, unpack_transaction};
use medici_core::function::{ServiceCompliance, State};
use medici_core::service::storage::StackStorage;
use medici_core::stm::{PullupFrom, PushdownFrom, TransitionFrom};

use super::Machine;
use super::state::leaves::triggerable::*;
use super::state::*;
use super::transaction::TransactionItem;

/* Machine<Wait<Start>> -> Machine<Wait<Input>> */
impl TransitionFrom<Machine<Wait<Start>>> for Machine<Wait<Input>> {
    fn transition_from(old: Machine<Wait<Start>>, t: <Self::State as State>::Transaction) -> Self {
        Machine {
            state: PhantomData,
            transaction: t,
            // Following properties MUST stay in sync with `Machine` !
            transaction_storage: old.transaction_storage,
            triggers: old.triggers,
        }
    }
}

/* Machine<Wait<Input>> -> Machine<Finished> */
impl TransitionFrom<Machine<Wait<Input>>> for Machine<Finished> {
    fn transition_from(old: Machine<Wait<Input>>, t: <Self::State as State>::Transaction) -> Self {
        Machine {
            state: PhantomData,
            transaction: t,
            // Following properties MUST stay in sync with `Machine` !
            transaction_storage: old.transaction_storage,
            triggers: old.triggers,
        }
    }
}

/* Machine<Wait<Input>> <-> Machine<Action<Print>> */
impl PushdownFrom<Machine<Wait<Input>>, TransactionItem> for Machine<Action<Print>> {
    fn pushdown_from(
        mut old: Machine<Wait<Input>>,
        t: <Self::State as State>::Transaction,
    ) -> Self {
        // Archive state of the old machine.
        let old_transaction: TransactionItem = pack_transaction(old.transaction);
        ServiceCompliance::<StackStorage<TransactionItem>>::get_mut(&mut old)
            .push(old_transaction)
            .expect("Never type triggered!");

        // Build new machine.
        Machine {
            state: PhantomData,
            transaction: t,
            // Following properties MUST stay in sync with `Machine` !
            transaction_storage: old.transaction_storage,
            triggers: old.triggers,
        }
    }
}

/* Machine<Wait<Input>> <-> Machine<Action<Print>> */
impl PullupFrom<Machine<Action<Print>>, TransactionItem> for Machine<Wait<Input>> {
    fn pullup_from(mut old: Machine<Action<Print>>) -> Result<Self, MachineError> {
        // Restore previously stored state.
        let old_transaction = ServiceCompliance::<StackStorage<TransactionItem>>::get_mut(&mut old)
            .pop()
            .context(ErrorKind::LogicError, &old)
            .and_then(|item| unpack_transaction(item).context(ErrorKind::ConstraintError, &old))?;

        // DBG
        // let old_transaction = Epsilon;

        // Build new machine.
        Ok(Machine {
            state: PhantomData,
            transaction: old_transaction,
            // Following properties MUST stay in sync with `Machine` !
            transaction_storage: old.transaction_storage,
            triggers: old.triggers,
        })
    }
}

/* Machine<Action<Print>> <-> Machine<Action<Load>> */
impl PushdownFrom<Machine<Action<Print>>, TransactionItem> for Machine<Action<Load>> {
    fn pushdown_from(
        mut old: Machine<Action<Print>>,
        t: <Self::State as State>::Transaction,
    ) -> Self {
        // Archive state of the old machine.
        let old_transaction: TransactionItem = pack_transaction(old.transaction);
        ServiceCompliance::<StackStorage<TransactionItem>>::get_mut(&mut old)
            .push(old_transaction)
            .expect("Never type triggered!");

        // Build new machine.
        Machine {
            state: PhantomData,
            transaction: t,
            // Following properties MUST stay in sync with `Machine` !
            transaction_storage: old.transaction_storage,
            triggers: old.triggers,
        }
    }
}

/* Machine<Action<Print>> <-> Machine<Action<Load>> */
impl PullupFrom<Machine<Action<Load>>, TransactionItem> for Machine<Action<Print>> {
    fn pullup_from(mut old: Machine<Action<Load>>) -> Result<Self, MachineError> {
        // Restore previously stored state.
        let old_transaction = ServiceCompliance::<StackStorage<TransactionItem>>::get_mut(&mut old)
            .pop()
            .context(ErrorKind::LogicError, &old)
            .and_then(|item| unpack_transaction(item).context(ErrorKind::ConstraintError, &old))?;

        // DBG
        // let old_transaction = PrintTransaction("dbg");

        // Build new machine.
        Ok(Machine {
            state: PhantomData,
            transaction: old_transaction,
            // Following properties MUST stay in sync with `Machine` !
            transaction_storage: old.transaction_storage,
            triggers: old.triggers,
        })
    }
}
