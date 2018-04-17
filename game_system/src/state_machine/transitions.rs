//! Defines all transitions within our state machine.

use std::marker::PhantomData;
// Trait must be in scope for the [`SnapshottedErrorExt::context`] method to work.
use medici_core::error::SnapshottedErrorExt;
// Import the prelude because the macro arguments must resolve ofcourse.
use prelude::*;

/// Macro to easily implement [`TransitionFrom`] for state machine transitions.
macro_rules! build_transition {
    (Machine < $from:ty > => Machine < $into:ty >; $($args:tt)*) => {
        impl<CTS, $($args)*> $crate::re_export::TransitionFrom<$crate::prelude::Machine<$from, CTS>, CTS>
            for $crate::prelude::Machine<$into, CTS>
        where
            CTS: $crate::prelude::CTStack + 'static,
        {
            fn transition_from(
                old: Machine<$from, CTS>,
                t: <Self::State as $crate::re_export::function::State>::Transaction,
            ) -> Self {
                $crate::prelude::Machine {
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
    (Machine < $from:ty > => Machine < $into:ty >) => {
        build_transition!(Machine< $from > => Machine< $into > ;);
    };
}

/// Macro to implement both [`PushdownFrom`] and [`PullupFrom`] for two given types.
macro_rules! push_it {
    (
        Machine < $from:ty > : = : Machine < $into:ty >; $($args:tt)*
    ) => {
        build_pushdown!(Machine< $from > => Machine< $into >; $($args)*);
        build_pullup!(Machine< $into > => Machine< $from >; $($args)*);
    };
    (
        Machine < $from:ty > : = : Machine < $into:ty >
    ) => {
        push_it!(Machine < $from > : = : Machine < $into >;);
    };
}

/// Macro to easily implement [`PushdownFrom`] for state machine transitions.
macro_rules! build_pushdown {
    (Machine< $from:ty > => Machine< $into:ty >; $t_type:ty; $($args:tt)*) => {
        #[allow(non_camel_case_types)]
        impl<CTS_OLD, $($args)*> $crate::re_export::PushdownFrom<$crate::prelude::Machine<$from, CTS_OLD>,
            $crate::re_export::ct!($into => CTS_OLD),
            $t_type
        > for $crate::prelude::Machine< $into, $crate::re_export::ct!($into => CTS_OLD) >
        where
            CTS_OLD: $crate::prelude::CTStack + 'static,
        {
            fn pushdown_from(
                mut old: Machine<$from, CTS_OLD>,
                t: <Self::State as $crate::re_export::function::State>::Transaction,
            ) -> Self
            {
                // Archive state of the old machine.
                let old_transaction: $t_type = $crate::prelude::pack_transaction(old.transaction);
                $crate::re_export::function::ServiceCompliance::<$crate::re_export::storage::StackStorage<$t_type>>::get_mut(&mut old)
                    .push(old_transaction)
                    .expect("Never type triggered!");

                // Build new machine.
                $crate::prelude::Machine {
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
    (Machine< $from:ty > => Machine< $into:ty >; $($args:tt)*) => {
        build_pushdown!(Machine< $from > => Machine< $into >; $crate::state_machine::transaction::TransactionItem; $($args)*);
    };
    (Machine< $from:ty > => Machine< $into:ty >) => {
        build_pushdown!(Machine< $from > => Machine< $into >; $crate::state_machine::transaction::TransactionItem;);
    };
}

/// Macro to easily implement [`PullupFrom`] for state machine transitions.
macro_rules! build_pullup {
    (Machine<$from:ty> => Machine<$into:ty>; $t_type:ty; $($args:tt)*) => {
        #[allow(non_camel_case_types)]
        impl<CTS, $($args)*> $crate::re_export::PullupFrom<$crate::prelude::Machine<$from, CTS>, CTS, $t_type>
        for $crate::prelude::Machine< $into, <CTS as $crate::prelude::CTStack>::Tail >
        where
            CTS: $crate::prelude::CTStack + $crate::std::fmt::Debug + Clone + Send + Sync + 'static,
        {
            fn pullup_from(mut old: $crate::prelude::Machine<$from, CTS>) -> Result<Self, $crate::prelude::MachineError>
            {
                // Archive state of the old machine.
                let old_transaction = $crate::re_export::function::ServiceCompliance::<$crate::re_export::storage::StackStorage<$t_type>>
                ::get_mut(&mut old)
                    .pop()
                    .context($crate::prelude::ErrorKind::LogicError, &old)
                    .and_then(|item| {
                        $crate::prelude::unpack_transaction(item).context($crate::prelude::ErrorKind::ConstraintError, &old)
                    })?;

                // Build new machine.
                Ok($crate::prelude::Machine {
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
    (Machine< $from:ty > => Machine< $into:ty >; $($args:tt)*) => {
        build_pullup!(Machine< $from > => Machine< $into >; $crate::state_machine::transaction::TransactionItem; $($args)*);
    };
    (Machine< $from:ty > => Machine< $into:ty >) => {
        build_pullup!(Machine< $from > => Machine< $into >; $crate::state_machine::transaction::TransactionItem;);
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

// Note: Covered by generic implementation!
// push_it!(Machine<Trigger<Pre, Start>> :=: Machine<RecurseEffect<Start>>);
// push_it!(Machine<Trigger<Peri, Start>> :=: Machine<RecurseEffect<Start>>);
// push_it!(Machine<Trigger<Post, Start>> :=: Machine<RecurseEffect<Start>>);

/* Trigger: EndTurn */
build_transition!(Machine<Effect<EndTurn>> => Machine<Trigger<Pre, EndTurn>>);
build_transition!(Machine<Trigger<Pre, EndTurn>> => Machine<Trigger<Peri, EndTurn>>);
build_transition!(Machine<Trigger<Peri, EndTurn>> => Machine<Trigger<Post, EndTurn>>);
build_transition!(Machine<Trigger<Post, EndTurn>> => Machine<Effect<EndTurn>>);

// Note: Covered by generic implementation!
// push_it!(Machine<Trigger<Pre, EndTurn>> :=: Machine<RecurseEffect<EndTurn>>);
// push_it!(Machine<Trigger<Peri, EndTurn>> :=: Machine<RecurseEffect<EndTurn>>);
// push_it!(Machine<Trigger<Post, EndTurn>> :=: Machine<RecurseEffect<EndTurn>>);

/* Recurse trigger transitions */
// Note: These macro invocations don't work because parsing generic arguments is tough.
// Ideally a procedural macro should be used to generate these implementations!
// See module [`gen_impl`] for manual implementations of these transitions.
/*
push_it!(Machine<RecurseEffect<X>> :=: Machine<Trigger<Pre, X>>; 
    X: State + marker::Triggerable + Debug + Clone + Send + Sync + 'static,
    X::Transaction: Into<TransactionItem>,);
push_it!(Machine<Trigger<Pre, X>> :=: Machine<Trigger<Peri, X>>; 
    X: State + marker::Triggerable + Debug +Clone + Send + Sync + 'static,
    X::Transaction: Into<TransactionItem>,);
push_it!(Machine<Trigger<Peri, X>> :=: Machine<Trigger<Post, X>>; 
    X: State + marker::Triggerable + Debug +Clone + Send + Sync + 'static,
    X::Transaction: Into<TransactionItem>,);
*/

// Module for generic implementations of recursive trigger states.
// TODO: Create a (proc?) macro which can automate these implementations, since the
// bodies are very similar which currently leads to a lot of code duplication.
mod gen_impl {
    use std::convert::TryFrom;
    use std::fmt::Debug;
    use std::marker::PhantomData;

    use failure::Fail;

    use prelude::transaction::TransactionItem;
    use prelude::*;
    use re_export::{ct, function, marker, storage, PullupFrom, PushdownFrom};

    /* RecurseEffect<_> -> Trigger<Pre, _> */
    #[allow(non_camel_case_types)]
    impl<CTS_OLD, TR>
        PushdownFrom<
            Machine<RecurseEffect<TR>, CTS_OLD>,
            ct!(Trigger<Pre, TR> => CTS_OLD),
            TransactionItem,
        > for Machine<Trigger<Pre, TR>, ct!(Trigger<Pre, TR> => CTS_OLD)>
    where
        CTS_OLD: CTStack + 'static,
        TR: function::State + marker::Triggerable + 'static,
        <TR as function::State>::Transaction: Into<TransactionItem>,
    {
        fn pushdown_from(
            mut old: Machine<RecurseEffect<TR>, CTS_OLD>,
            t: <Self::State as function::State>::Transaction,
        ) -> Self {
            // Archive state of the old machine.
            let old_transaction: TransactionItem = pack_transaction(old.transaction);
            function::ServiceCompliance::<storage::StackStorage<TransactionItem>>::get_mut(
                &mut old,
            ).push(old_transaction)
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

    /* RecurseEffect<_> <- Trigger<Pre, _> */
    #[allow(non_camel_case_types)]
    impl<CTS, TR> PullupFrom<Machine<Trigger<Pre, TR>, CTS>, CTS, TransactionItem>
        for Machine<RecurseEffect<TR>, <CTS as CTStack>::Tail>
    where
        CTS: CTStack + Debug + Clone + Send + Sync + 'static,
        TR: function::State + marker::Triggerable + Debug + Clone + Send + Sync + 'static,
        <TR as function::State>::Transaction:
            TryFrom<TransactionItem> + Debug + Send + Sync + 'static,
        <<TR as function::State>::Transaction as TryFrom<TransactionItem>>::Error: Fail,
    {
        fn pullup_from(mut old: Machine<Trigger<Pre, TR>, CTS>) -> Result<Self, MachineError> {
            // Archive state of the old machine.
            let old_transaction = function::ServiceCompliance::<
                storage::StackStorage<TransactionItem>,
            >::get_mut(&mut old)
                .pop()
                .context(ErrorKind::LogicError, &old)
                .and_then(|item| {
                    unpack_transaction(item).context(ErrorKind::ConstraintError, &old)
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

    /* Trigger<Pre, _> -> Trigger<Peri, _> */
    #[allow(non_camel_case_types)]
    impl<CTS_OLD, TR>
        PushdownFrom<
            Machine<Trigger<Pre, TR>, CTS_OLD>,
            ct!(Trigger<Peri, TR> => CTS_OLD),
            TransactionItem,
        > for Machine<Trigger<Peri, TR>, ct!(Trigger<Peri, TR> => CTS_OLD)>
    where
        CTS_OLD: CTStack + 'static,
        TR: function::State + marker::Triggerable + 'static,
        <TR as function::State>::Transaction: Into<TransactionItem>,
    {
        fn pushdown_from(
            mut old: Machine<Trigger<Pre, TR>, CTS_OLD>,
            t: <Self::State as function::State>::Transaction,
        ) -> Self {
            // Archive state of the old machine.
            let old_transaction: TransactionItem = pack_transaction(old.transaction);
            function::ServiceCompliance::<storage::StackStorage<TransactionItem>>::get_mut(
                &mut old,
            ).push(old_transaction)
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

    /* Trigger<Pre, _> <- Trigger<Peri, _>  */
    #[allow(non_camel_case_types)]
    impl<CTS, TR> PullupFrom<Machine<Trigger<Peri, TR>, CTS>, CTS, TransactionItem>
        for Machine<Trigger<Pre, TR>, <CTS as CTStack>::Tail>
    where
        CTS: CTStack + Debug + Clone + Send + Sync + 'static,
        TR: function::State + marker::Triggerable + Debug + Clone + Send + Sync + 'static,
        <TR as function::State>::Transaction:
            TryFrom<TransactionItem> + Debug + Send + Sync + 'static,
        <<TR as function::State>::Transaction as TryFrom<TransactionItem>>::Error: Fail,
    {
        fn pullup_from(mut old: Machine<Trigger<Peri, TR>, CTS>) -> Result<Self, MachineError> {
            // Archive state of the old machine.
            let old_transaction = function::ServiceCompliance::<
                storage::StackStorage<TransactionItem>,
            >::get_mut(&mut old)
                .pop()
                .context(ErrorKind::LogicError, &old)
                .and_then(|item| {
                    unpack_transaction(item).context(ErrorKind::ConstraintError, &old)
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

    /* Trigger<Peri, _> -> Trigger<Post, _> */
    #[allow(non_camel_case_types)]
    impl<CTS_OLD, TR>
        PushdownFrom<
            Machine<Trigger<Peri, TR>, CTS_OLD>,
            ct!(Trigger<Post, TR> => CTS_OLD),
            TransactionItem,
        > for Machine<Trigger<Post, TR>, ct!(Trigger<Post, TR> => CTS_OLD)>
    where
        CTS_OLD: CTStack + 'static,
        TR: function::State + marker::Triggerable + 'static,
        <TR as function::State>::Transaction: Into<TransactionItem>,
    {
        fn pushdown_from(
            mut old: Machine<Trigger<Peri, TR>, CTS_OLD>,
            t: <Self::State as function::State>::Transaction,
        ) -> Self {
            // Archive state of the old machine.
            let old_transaction: TransactionItem = pack_transaction(old.transaction);
            function::ServiceCompliance::<storage::StackStorage<TransactionItem>>::get_mut(
                &mut old,
            ).push(old_transaction)
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

    /* Trigger<Peri, _> <- Trigger<Post, _>  */
    #[allow(non_camel_case_types)]
    impl<CTS, TR> PullupFrom<Machine<Trigger<Post, TR>, CTS>, CTS, TransactionItem>
        for Machine<Trigger<Peri, TR>, <CTS as CTStack>::Tail>
    where
        CTS: CTStack + Debug + Clone + Send + Sync + 'static,
        TR: function::State + marker::Triggerable + Debug + Clone + Send + Sync + 'static,
        <TR as function::State>::Transaction:
            TryFrom<TransactionItem> + Debug + Send + Sync + 'static,
        <<TR as function::State>::Transaction as TryFrom<TransactionItem>>::Error: Fail,
    {
        fn pullup_from(mut old: Machine<Trigger<Post, TR>, CTS>) -> Result<Self, MachineError> {
            // Archive state of the old machine.
            let old_transaction = function::ServiceCompliance::<
                storage::StackStorage<TransactionItem>,
            >::get_mut(&mut old)
                .pop()
                .context(ErrorKind::LogicError, &old)
                .and_then(|item| {
                    unpack_transaction(item).context(ErrorKind::ConstraintError, &old)
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

    /* Trigger<_, _> -> RecurseEffect<_> */
    #[allow(non_camel_case_types)]
    impl<CTS_OLD, TR, TM>
        PushdownFrom<
            Machine<Trigger<TM, TR>, CTS_OLD>,
            ct!(RecurseEffect<TR> => CTS_OLD),
            TransactionItem,
        > for Machine<RecurseEffect<TR>, ct!(RecurseEffect<TR> => CTS_OLD)>
    where
        CTS_OLD: CTStack + 'static,
        TR: function::State + marker::Triggerable + 'static,
        <TR as function::State>::Transaction: Into<TransactionItem>,
        TM: function::State + marker::Timing + 'static,
    {
        fn pushdown_from(
            mut old: Machine<Trigger<TM, TR>, CTS_OLD>,
            t: <Self::State as function::State>::Transaction,
        ) -> Self {
            // Archive state of the old machine.
            let old_transaction: TransactionItem = pack_transaction(old.transaction);
            function::ServiceCompliance::<storage::StackStorage<TransactionItem>>::get_mut(
                &mut old,
            ).push(old_transaction)
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

    /* Trigger<_, _> <- RecurseEffect<_>  */
    #[allow(non_camel_case_types)]
    impl<CTS, TR, TM> PullupFrom<Machine<RecurseEffect<TR>, CTS>, CTS, TransactionItem>
        for Machine<Trigger<TM, TR>, <CTS as CTStack>::Tail>
    where
        CTS: CTStack + Debug + Clone + Send + Sync + 'static,
        TR: function::State + marker::Triggerable + Debug + Clone + Send + Sync + 'static,
        <TR as function::State>::Transaction:
            TryFrom<TransactionItem> + Debug + Send + Sync + 'static,
        <<TR as function::State>::Transaction as TryFrom<TransactionItem>>::Error: Fail,
        TM: function::State + marker::Timing + 'static,
    {
        fn pullup_from(mut old: Machine<RecurseEffect<TR>, CTS>) -> Result<Self, MachineError> {
            // Archive state of the old machine.
            let old_transaction = function::ServiceCompliance::<
                storage::StackStorage<TransactionItem>,
            >::get_mut(&mut old)
                .pop()
                .context(ErrorKind::LogicError, &old)
                .and_then(|item| {
                    unpack_transaction(item).context(ErrorKind::ConstraintError, &old)
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
}
