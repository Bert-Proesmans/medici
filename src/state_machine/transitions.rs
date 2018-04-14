//! Defines all transitions within our state machine.

use std::marker::PhantomData;

use medici_core::error::SnapshottedErrorExt;
use medici_core::function::State;
use medici_core::stm::{PullupFrom, PushdownFrom, TransitionFrom};

use state_machine::prelude::*;
use state_machine::state::prelude::*;

/// Macro to easily implement [`TransitionFrom`] for state machine transitions.
macro_rules! build_transition {
    ($from:ty => $into:ty) => {
        impl TransitionFrom<$from> for $into {
            fn transition_from(old: $from, t: <Self::State as State>::Transaction) -> Self {
                Machine {
                    state: PhantomData,
                    transaction: t,
                    // Following properties MUST stay in sync with `Machine` !
                    transactions: old.transactions,
                    entities: old.entities,
                    triggers: old.triggers,
                }
            }
        }
    };
}

/// Macro to easily implement [`PushdownFrom`] for state machine transitions.
macro_rules! build_pushdown {
    ($from:ty => $into:ty) => {
        build_pushdown!($from => $into; $crate::state_machine::transaction::TransactionItem);
    };
    ($from:ty => $into:ty; $t_type:ty) => {
        impl PushdownFrom<$from, $t_type> for $into {
            fn pushdown_from(mut old: $from, t: <Self::State as State>::Transaction) -> Self
            where
                $from: $crate::medici_core::function::StateContainer,
            {
                // Archive state of the old machine.
                let old_transaction: $t_type =
                    $crate::medici_core::function::helper::pack_transaction(old.transaction);
                $crate::medici_core::function::ServiceCompliance::<
                    $crate::medici_core::service::storage::StackStorage<$t_type>,
                >::get_mut(&mut old)
                    .push(old_transaction)
                    .expect("Never type triggered!");

                // Build new machine.
                Machine {
                    state: PhantomData,
                    transaction: t,
                    // Following properties MUST stay in sync with `Machine` !
                    transactions: old.transactions,
                    entities: old.entities,
                    triggers: old.triggers,
                }
            }
        }
    };
}

/// Macro to easily implement [`PullupFrom`] for state machine transitions.
macro_rules! build_pullup {
    ($from:ty => $into:ty) => {
        build_pullup!($from => $into; $crate::state_machine::transaction::TransactionItem);
    };
    ($from:ty => $into:ty; $t_type:ty) => {
        impl PullupFrom<$from, $t_type> for $into {
            fn pullup_from(mut old: $from) -> Result<Self, $crate::medici_core::error::MachineError>
            where
                $from: $crate::medici_core::function::StateContainer,
            {
                // Archive state of the old machine.
                let old_transaction = $crate::medici_core::function::ServiceCompliance::<
                    $crate::medici_core::service::storage::StackStorage<$t_type>,
                >::get_mut(&mut old)
                    .pop()
                    .context($crate::medici_core::error::ErrorKind::LogicError, &old)
                    .and_then(|item| {
                        $crate::medici_core::function::helper::unpack_transaction(item)
                            .context($crate::medici_core::error::ErrorKind::ConstraintError, &old)
                    })?;

                // Build new machine.
                Ok(Machine {
                    state: PhantomData,
                    transaction: old_transaction,
                    // Following properties MUST stay in sync with `Machine` !
                    transactions: old.transactions,
                    entities: old.entities,
                    triggers: old.triggers,
                })
            }
        }
    };
}

build_transition!(Machine<Wait<Start>> => Machine<Action<Start>>);
build_transition!(Machine<Wait<Input>> => Machine<Action<EndTurn>>);

build_transition!(Machine<Action<Start>> => Machine<Wait<Input>>);
build_transition!(Machine<Action<EndTurn>> => Machine<Wait<Input>>);

build_pushdown!(Machine<Action<Start>> => Machine<Effect<Start>>);
build_pushdown!(Machine<Action<EndTurn>> => Machine<Effect<EndTurn>>);

build_transition!(Machine<Effect<Start>> => Machine<Trigger<Pre, Start>>);
build_transition!(Machine<Trigger<Pre, Start>> => Machine<Trigger<Peri, Start>>);
build_transition!(Machine<Trigger<Peri, Start>> => Machine<Trigger<Post, Start>>);
build_transition!(Machine<Trigger<Post, Start>> => Machine<Effect<Start>>);

build_transition!(Machine<Effect<EndTurn>> => Machine<Trigger<Pre, EndTurn>>);
build_transition!(Machine<Trigger<Pre, EndTurn>> => Machine<Trigger<Peri, EndTurn>>);
build_transition!(Machine<Trigger<Peri, EndTurn>> => Machine<Trigger<Post, EndTurn>>);
build_transition!(Machine<Trigger<Post, EndTurn>> => Machine<Effect<EndTurn>>);

build_pullup!(Machine<Effect<Start>> => Machine<Action<Start>>);
build_pullup!(Machine<Effect<EndTurn>> => Machine<Action<EndTurn>>);
