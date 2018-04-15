//! Defines all transitions within our state machine.

use std::marker::PhantomData;

use medici_core::error::SnapshottedErrorExt;

use state_machine::prelude::*;
use state_machine::state::prelude::*;

/// Macro to easily implement [`TransitionFrom`] for state machine transitions.
macro_rules! build_transition {
    (Machine < $from:ty > => Machine < $into:ty >) => {
        impl<CTS> $crate::medici_core::stm::checked::TransitionFrom<Machine<$from, CTS>, CTS>
            for Machine<$into, CTS>
        where
            CTS: $crate::medici_core::ctstack::CTStack + 'static,
        {
            fn transition_from(
                old: Machine<$from, CTS>,
                t: <Self::State as $crate::medici_core::function::State>::Transaction,
            ) -> Self {
                Machine {
                    state: PhantomData,
                    history: PhantomData,
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

/// Macro to implement both [`PushdownFrom`] and [`PullupFrom`] for two given types.
macro_rules! push_it {
    (Machine < $from:ty > : = : Machine < $into:ty >) => {
        build_pushdown!(Machine< $from > => Machine< $into >);
        build_pullup!(Machine< $into > => Machine< $from >);
    };
}

/// Macro to easily implement [`PushdownFrom`] for state machine transitions.
macro_rules! build_pushdown {
    (Machine< $from:ty > => Machine< $into:ty >) => {
        build_pushdown!(Machine< $from > => Machine< $into >; $crate::state_machine::transaction::TransactionItem);
    };
    (Machine< $from:ty > => Machine< $into:ty >; $t_type:ty) => {
        #[allow(non_camel_case_types)]
        impl<CTS_OLD> $crate::medici_core::stm::checked::PushdownFrom<
            Machine<$from, CTS_OLD>,
            $crate::medici_core::ct!($into => CTS_OLD),
            $t_type
        > for Machine< $into, $crate::medici_core::ct!($into => CTS_OLD) >
        where
            CTS_OLD: $crate::medici_core::ctstack::CTStack + 'static,
        {
            fn pushdown_from(
                mut old: Machine<$from, CTS_OLD>,
                t: <Self::State as $crate::medici_core::function::State>::Transaction,
            ) -> Self
            {
                // Archive state of the old machine.
                let old_transaction: $t_type = $crate::medici_core::transaction::pack_transaction(old.transaction);
                $crate::medici_core::function::ServiceCompliance::<$crate::medici_core::service::storage::StackStorage<$t_type>>::get_mut(&mut old)
                    .push(old_transaction)
                    .expect("Never type triggered!");

                // Build new machine.
                Machine {
                    state: PhantomData,
                    history: PhantomData,
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
    (Machine< $from:ty > => Machine< $into:ty >) => {
        build_pullup!(Machine< $from > => Machine< $into >; $crate::state_machine::transaction::TransactionItem);
    };
    (Machine<$from:ty> => Machine<$into:ty>; $t_type:ty) => {
        #[allow(non_camel_case_types)]
        impl<CTS> $crate::medici_core::stm::checked::PullupFrom<Machine<$from, CTS>, CTS, $t_type>
        for Machine< $into, <CTS as $crate::medici_core::ctstack::CTStack>::Tail >
        where
            CTS: $crate::medici_core::ctstack::CTStack + $crate::std::fmt::Debug + Clone + Send + Sync + 'static,
        {
            fn pullup_from(mut old: Machine<$from, CTS>) -> Result<Self, $crate::medici_core::error::MachineError>
            {
                // Archive state of the old machine.
                let old_transaction = $crate::medici_core::function::ServiceCompliance::<
                    $crate::medici_core::service::storage::StackStorage<$t_type>,
                >::get_mut(&mut old)
                    .pop()
                    .context($crate::medici_core::error::ErrorKind::LogicError, &old)
                    .and_then(|item| {
                        $crate::medici_core::transaction::unpack_transaction(item)
                            .context($crate::medici_core::error::ErrorKind::ConstraintError, &old)
                    })?;

                // Build new machine.
                Ok(Machine {
                    state: PhantomData,
                    history: PhantomData,
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

/* Actions */
push_it!(Machine<Action<Start>> :=: Machine<Effect<Start>>);
push_it!(Machine<Action<EndTurn>> :=: Machine<Effect<EndTurn>>);

/* Trigger: Start */
build_transition!(Machine<Effect<Start>> => Machine<Trigger<Pre, Start>>);
build_transition!(Machine<Trigger<Pre, Start>> => Machine<Trigger<Peri, Start>>);
build_transition!(Machine<Trigger<Peri, Start>> => Machine<Trigger<Post, Start>>);
build_transition!(Machine<Trigger<Post, Start>> => Machine<Effect<Start>>);

push_it!(Machine<Trigger<Pre, Start>> :=: Machine<RecurseEffect<Pre, Start>>);
push_it!(Machine<Trigger<Peri, Start>> :=: Machine<RecurseEffect<Peri, Start>>);
push_it!(Machine<Trigger<Post, Start>> :=: Machine<RecurseEffect<Post, Start>>);

/* Trigger: EndTurn */
build_transition!(Machine<Effect<EndTurn>> => Machine<Trigger<Pre, EndTurn>>);
build_transition!(Machine<Trigger<Pre, EndTurn>> => Machine<Trigger<Peri, EndTurn>>);
build_transition!(Machine<Trigger<Peri, EndTurn>> => Machine<Trigger<Post, EndTurn>>);
build_transition!(Machine<Trigger<Post, EndTurn>> => Machine<Effect<EndTurn>>);

push_it!(Machine<Trigger<Pre, EndTurn>> :=: Machine<RecurseEffect<Pre, EndTurn>>);
push_it!(Machine<Trigger<Peri, EndTurn>> :=: Machine<RecurseEffect<Peri, EndTurn>>);
push_it!(Machine<Trigger<Post, EndTurn>> :=: Machine<RecurseEffect<Post, EndTurn>>);

#[cfg(test)]
mod tests {
    use std::default::Default;
    use std::marker::PhantomData;

    use medici_core::ctstack::EmptyStack;
    use medici_core::stm::checked::{PullupFrom, PushdownFrom};

    use state_machine::prelude::*;
    use state_machine::state::prelude::*;
    use state_machine::transaction::Epsilon;

    #[test]
    fn checked_transitions() {
        // Build a new machine to reuse internal parts to build a custom one.
        let machine = Machine::new(&Default::default()).expect("Error building machine");
        let machine: Machine<Action<Start>, EmptyStack> = Machine {
            state: PhantomData,
            history: PhantomData,
            transaction: Epsilon,
            //
            transactions: machine.transactions,
            entities: machine.entities,
            triggers: machine.triggers,
        };

        println!("START\n{:?}\n", machine);
        let push: Machine<Effect<Start>, _> = PushdownFrom::pushdown_from(machine, Epsilon);
        println!("PUSHED DOWN\n{:?}\n", push);
        let pull: Machine<Action<Start>, _> =
            PullupFrom::pullup_from(push).expect("Failed to pullup!");
        println!("PULLED UP\n{:?}\n", pull);
    }

    #[test]
    fn invalid_transition() {
        // Build a new machine to reuse internal parts to build a custom one.
        let machine = Machine::new(&Default::default()).expect("Error building machine");
        let machine: Machine<Effect<Start>, EmptyStack> = Machine {
            state: PhantomData,
            history: PhantomData,
            transaction: Epsilon,
            //
            transactions: machine.transactions,
            entities: machine.entities,
            triggers: machine.triggers,
        };
        // This is an invalid pullup because the transition history is empty.
        let pull: Result<Machine<Action<Start>, _>, _> = PullupFrom::pullup_from(machine);
        assert!(pull.is_err());
    }
}
