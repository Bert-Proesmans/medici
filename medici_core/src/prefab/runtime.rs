//! Module containing methods which make working with the state
//! machine a bit easier.

use failure::{format_err, Error};
use value_from_type_traits::IntoEnum;

use function::{ServiceCompliance, State, StateContainer, TriggerState};
use marker::{ActionableMarker, TimingEnumerator, TimingMarker, TransactionMarker,
             TriggerEnumerator, TriggerMarker};
use service::storage::UnsafeTrigger;
use service::trigger::TriggerService;
use stm::*;

use prefab::state::{Effect, Trigger};
use prefab::timing::*;

/// Extract all triggers from the provided machine for matching
/// conditions.
pub fn fetch_triggers<M, ETM, ETR>(machine: &M) -> Vec<UnsafeTrigger<ETM, ETR>>
where
    M: StateContainer + ServiceCompliance<TriggerService<ETM, ETR>>,
    M::State: TriggerState,
    <M::State as TriggerState>::Timing: TimingMarker + IntoEnum<ETM>,
    <M::State as TriggerState>::Trigger: TriggerMarker + IntoEnum<ETR>,
    ETM: TimingEnumerator + PartialEq + Copy,
    ETR: TriggerEnumerator + PartialEq + Copy,
{
    ServiceCompliance::<TriggerService<ETM, ETR>>::get(machine)
        .retrieve_triggers(machine)
        .cloned()
        .collect()
}

/// Macro used for building a function that automatically constructs a method called [`exec_triggers`].
///
/// The constructed method will automatically transition into the trigger substates and execute the
/// triggers which constraints match the system state.
#[macro_export]
macro_rules! build_exec_triggers {
    ($container_name:ident) => {
            use std::result::Result;
            use $crate::failure::{format_err, Error};
            use $crate::value_from_type_traits::IntoEnum;

            use $crate::function::{State, StateContainer};
            use $crate::marker::{ActionableMarker, TransactionMarker};
            use $crate::prefab::state::{Effect, Trigger};
            use $crate::prefab::timing::*;
            use $crate::stm::*;

            type M1<TR> = $container_name<Effect<TR>>;
            type M1_TT<TR> = <<M1<TR> as StateContainer>::State as State>::Transaction;
            type M1_ETR<TR> = <M1<TR> as StateContainer>::TriggerEnum;
            type M2<TR> = $container_name<Trigger<Pre, TR>>;
            type M3<TR> = $container_name<Trigger<Peri, TR>>;
            type M4<TR> = $container_name<Trigger<Post, TR>>;

            /// Takes the provided machine (in [`Effect`] state) and executes direct and indirect
            /// triggers.
            pub fn exec_triggers<TR>(
                machine: M1<TR>,
                transaction: M1_TT<TR>,
            ) -> Result<M1<TR>, Error>
            where
                // Note: These type constraints suppose the transaction of Effect<TR> is the same
                // for each TriggerState variant over Timing.
                // eg: Effect<TR>::Transaction == Trigger<Pre, TR>::Transaction ==
                // Trigger<Peri, TR>::Transaction == ..
                TR: ActionableMarker
                    + State<Transaction = M1_TT<TR>>
                    + IntoEnum<M1_ETR<TR>>
                    + 'static,
                //
                M1<TR>: StateContainer + TransitionInto<M2<TR>>,
                M2<TR>: TransitionInto<M3<TR>>,
                M3<TR>: TransitionInto<M4<TR>>,
                M4<TR>: TransitionInto<M1<TR>>,
            {
            	/*
                // Pre
                let pre: M2<TR> = machine.transition(transaction);

                // Peri
                let peri: M3<TR> = pre.transition(transaction);

                // Post
                let post: M4<TR> = peri.transition(transaction);

                Result::Ok(post.transition(transaction))
                */
               unimplemented!()
            }
    };
}
