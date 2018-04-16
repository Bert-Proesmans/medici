//! Module containing methods which make working with the state
//! machine a bit easier.

use failure::{format_err, Error};
use value_from_type_traits::IntoEnum;

use function::{ServiceCompliance, State, StateContainer, TriggerState};
use marker;
use service::trigger::{TriggerService, TriggerWrapper};
use stm::*;
use storage::UnsafeTrigger;

/// Extract all triggers from the provided machine for matching
/// conditions.
pub fn fetch_triggers<M, ETM, ETR>(machine: &M) -> Vec<UnsafeTrigger<ETM, ETR>>
where
    M: StateContainer + ServiceCompliance<TriggerService<ETM, ETR>>,
    <M as StateContainer>::State: TriggerState,
    <M::State as TriggerState>::Timing: marker::Timing + IntoEnum<ETM>,
    <M::State as TriggerState>::Trigger: marker::Triggerable + IntoEnum<ETR>,
    ETM: marker::TimingEnumerator + PartialEq + Copy,
    ETR: marker::TriggerEnumerator + PartialEq + Copy,
{
    ServiceCompliance::<TriggerService<ETM, ETR>>::get(machine)
        .retrieve_triggers(machine)
        .cloned()
        .collect()
}

/// Executes all passed down triggers for the provided machine.
///
/// # Unsafe
/// Verify that each provided [`UnsafeTrigger`] has been specifically
/// created for the provided machine!
pub unsafe fn exec_trigger_stepped<M, TM, TR, ETM, ETR, I>(
    mut machine: M,
    triggers: I,
) -> Result<M, Error>
where
    M: StateContainer,
    M::State: TriggerState<Timing = TM, Trigger = TR>,
    TM: marker::Timing + IntoEnum<ETM>,
    TR: marker::Triggerable + IntoEnum<ETR>,
    ETM: marker::TimingEnumerator + PartialEq + Copy,
    ETR: marker::TriggerEnumerator + PartialEq + Copy,
    I: IntoIterator<Item = UnsafeTrigger<ETM, ETR>>,
{
    for t in triggers.into_iter() {
        // Cast the unsafe trigger into a safe wrapper.
        // This wrapper holds the callback tailored to the provided machine.
        // # Unsafe
        // Because the StateContainer type is erased, execution of the wrapped
        // trigger could lead to UB when a wrong trigger has been inserted.
        unsafe {
            let wrapper = match TriggerWrapper::try_from_trigger_entry(t) {
                Ok(t) => t,
                // TODO; Proper error handling here
                Err(e) => return Err(e),
            };

            // Execute trigger on the machine.
            // The machine is consumed and a new one is returned. This sequence happens for each
            // trigger.
            machine = (wrapper.into_callback())(machine)?;
        }
    }

    Ok(machine)
}

/// Macro used for building a function that automatically constructs a method called [`exec_triggers`].
///
/// The constructed method will automatically transition into the trigger substates and execute the
/// triggers which constraints match the system state.
#[macro_export]
macro_rules! build_exec_triggers_checked {
    ($container_name:ident) => {
        // Since we're in macro space, we have no access to the std prelude!
        use std::result::Result;
        use $crate::failure::Error;
        use $crate::value_from_type_traits::IntoEnum;

        use self::_shorten_syntax::*;
        use $crate::ctstack::CTStack;
        use $crate::function::{ServiceCompliance, State, StateContainer, TriggerState};
        use $crate::marker;
        use $crate::prefab::runtime::{exec_trigger_stepped, fetch_triggers};
        use $crate::prefab::state::{Effect, Trigger};
        use $crate::service::trigger::TriggerService;

        #[doc(hidden)]
        #[allow(non_camel_case)]
        mod _shorten_syntax {
            use super::*;
            pub type M1<TR, CTS> = $container_name<Effect<TR>, CTS>;
            pub type M2<TR, CTS> = $container_name<Trigger<Pre, TR>, CTS>;
            pub type M3<TR, CTS> = $container_name<Trigger<Peri, TR>, CTS>;
            pub type M4<TR, CTS> = $container_name<Trigger<Post, TR>, CTS>;
        }

        /// Takes the provided machine (in [`Effect`] state) and executes direct and indirect
        /// triggers.
        pub fn exec_triggers<CTS, TR, TT, ETM, ETR>(
            machine: M1<TR, CTS>,
            transaction: TT,
        ) -> Result<M1<TR, CTS>, Error>
        where
            CTS: CTStack + 'static,
            // Note: These type constraints suppose the transaction of Effect<TR> is the same
            // for each TriggerState variant over Timing.
            // eg: Effect<TR>::Transaction == Trigger<Pre, TR>::Transaction ==
            // Trigger<Peri, TR>::Transaction == ..
            TR: marker::Actionable + State<Transaction = TT> + IntoEnum<ETR>,
            TT: marker::Transaction,
            ETM: marker::TimingEnumerator + PartialEq + Copy,
            ETR: marker::TriggerEnumerator + PartialEq + Copy,
            //
            M1<TR, CTS>: StateContainer<TimingEnum = ETM, TriggerEnum = ETR>
                + TransitionInto<M2<TR, CTS>, CTS>,
            <M1<TR, CTS> as StateContainer>::State: State<Transaction = TT>,

            M2<TR, CTS>: StateContainer<TimingEnum = ETM, TriggerEnum = ETR>
                + TransitionInto<M3<TR, CTS>, CTS>
                + ServiceCompliance<TriggerService<ETM, ETR>>,
            <M2<TR, CTS> as StateContainer>::State:
                State<Transaction = TT> + TriggerState<Trigger = TR>,
            <<M2<TR, CTS> as StateContainer>::State as TriggerState>::Timing: IntoEnum<ETM>,

            M3<TR, CTS>: StateContainer<TimingEnum = ETM, TriggerEnum = ETR>
                + TransitionInto<M4<TR, CTS>, CTS>
                + ServiceCompliance<TriggerService<ETM, ETR>>,
            <M3<TR, CTS> as StateContainer>::State:
                State<Transaction = TT> + TriggerState<Trigger = TR>,
            <<M3<TR, CTS> as StateContainer>::State as TriggerState>::Timing: IntoEnum<ETM>,

            M4<TR, CTS>: StateContainer<TimingEnum = ETM, TriggerEnum = ETR>
                + TransitionInto<M1<TR, CTS>, CTS>
                + ServiceCompliance<TriggerService<ETM, ETR>>,
            <M4<TR, CTS> as StateContainer>::State:
                State<Transaction = TT> + TriggerState<Trigger = TR>,
            <<M4<TR, CTS> as StateContainer>::State as TriggerState>::Timing: IntoEnum<ETM>,
        {
            // Pre
            let mut pre: M2<TR, CTS> = machine.transition(transaction);
            let listeners = fetch_triggers(&pre);
            // IMMUT REBIND
            let pre = unsafe { exec_trigger_stepped(pre, listeners)? };

            // Peri
            let peri: M3<TR, CTS> = pre.transition(transaction);
            let listeners = fetch_triggers(&peri);
            // IMMUT REBIND
            let peri = unsafe { exec_trigger_stepped(peri, listeners)? };

            // Post
            let post: M4<TR, CTS> = peri.transition(transaction);
            let listeners = fetch_triggers(&post);
            // IMMUT REBIND
            let post = unsafe { exec_trigger_stepped(post, listeners)? };
            // Explicit Ok invariant type because we have no access to
            // the std prelude.
            Result::Ok(post.transition(transaction))
        }
    };
}
