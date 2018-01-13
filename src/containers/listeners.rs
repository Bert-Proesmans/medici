use std::fmt::Debug;
use std::clone::Clone;
use std::marker::PhantomData;
use std::convert::TryFrom;

use timing_traits::Timing;
use from_generic_traits::FromGeneric;
use action_traits::Triggerable;

use containers::games::Game;
use hs_automaton::states::*;
use hs_automaton::states::global_states::timing::EnumerationTiming;
use hs_automaton::states::action_states::EnumerationTrigger;

type FNTrigger<T, U> = fn(Game<Trigger<T, U>>) -> Result<Game<Trigger<T, U>>, Game<Finished>>;

#[derive(Debug)]
pub struct TriggerWrapper<T, U>
where
    T: Timing + Debug,
    U: Triggerable + Debug,
{
    handler: FNTrigger<T, U>,
    phantom: PhantomData<(T, U)>,
}

impl<T, U> Clone for TriggerWrapper<T, U>
where
    T: Timing + Debug,
    U: Triggerable + Debug,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            phantom: self.phantom.clone(),
        }
    }
}

impl<T, U> TriggerWrapper<T, U>
where
    T: Timing + Debug,
    U: Triggerable + Debug,
    EnumerationTiming: FromGeneric<T>,
    EnumerationTrigger: FromGeneric<U>,
{
    fn new(handler: FNTrigger<T, U>) -> Self {
        Self {
            handler: handler,
            phantom: PhantomData,
        }
    }

    pub fn get_handler(self) -> FNTrigger<T, U> {
        self.handler
    }
}

#[derive(Debug, Clone)]
pub struct ListenerEntry(pub EnumerationTiming, pub EnumerationTrigger, pub *const ());

impl<T, U> From<TriggerWrapper<T, U>> for ListenerEntry
where
    T: Timing + Debug,
    U: Triggerable + Debug,
    EnumerationTiming: FromGeneric<T>,
    EnumerationTrigger: FromGeneric<U>,
{
    fn from(x: TriggerWrapper<T, U>) -> Self {
        let timing = <EnumerationTiming as FromGeneric<T>>::from_generic();
        let trigger = <EnumerationTrigger as FromGeneric<U>>::from_generic();
        let transmuted = x.handler as *const ();
        ListenerEntry(timing, trigger, transmuted)
    }
}

impl<T, U> TryFrom<ListenerEntry> for TriggerWrapper<T, U>
where
    T: Timing + Debug,
    U: Triggerable + Debug,
    EnumerationTiming: FromGeneric<T>,
    EnumerationTrigger: FromGeneric<U>,
{
    type Error = String;

    fn try_from(x: ListenerEntry) -> Result<Self, Self::Error> {
        let timing = <EnumerationTiming as FromGeneric<T>>::from_generic();
        let trigger = <EnumerationTrigger as FromGeneric<U>>::from_generic();

        if x.2.is_null() {
            return Err("Handler is NULL!".into());
        }

        if x.0 == timing && x.1 == trigger {
            unsafe {
                let transmuted: FNTrigger<T, U> = ::std::mem::transmute(x.2);
                Ok(TriggerWrapper {
                    handler: transmuted,
                    phantom: PhantomData,
                })
            }
        } else {
            Err("Incompatible layout".into())
        }
    }
}

#[derive(Debug)]
pub struct ListenerService {
    // Contains all objects which should be invoked when certain requirements are met.
    pub pre_action: Vec<ListenerEntry>,
    pub peri_action: Vec<ListenerEntry>,
    pub post_action: Vec<ListenerEntry>,
    pub excluded_action: Vec<ListenerEntry>, // Non action related trigger listeners?
}

impl ListenerService {
    pub fn add_peri_action<T, U>(&mut self, handler: FNTrigger<T, U>) -> Result<(), String>
    where
        T: Timing + Debug + 'static,
        U: Triggerable + Debug + 'static,
        EnumerationTiming: FromGeneric<T>,
        EnumerationTrigger: FromGeneric<U>,
    {
        let wrapper = TriggerWrapper::<T, U>::new(handler);
        self.peri_action.push(wrapper.into());
        Ok(())
    }

    pub fn retrieve_peri_action<T, U>(&self) -> impl Iterator<Item = &ListenerEntry>
    where
        T: Timing + Debug + 'static,
        U: Triggerable + Debug + 'static,
        EnumerationTiming: FromGeneric<T>,
        EnumerationTrigger: FromGeneric<U>,
    {
        self.peri_action
            .iter()
            .filter(|l| l.0 == <EnumerationTiming as FromGeneric<T>>::from_generic())
            .filter(|l| l.1 == <EnumerationTrigger as FromGeneric<U>>::from_generic())
    }
}
