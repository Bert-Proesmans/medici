use std::fmt::Debug;
use std::clone::Clone;
use std::marker::PhantomData;
use std::convert::TryFrom;

use medici_traits::FromType;
use medici_traits::timing_traits::Timing;
use medici_traits::timing_traits::default::EnumerationTiming;
use medici_traits::action_traits::Triggerable;

use containers::games::Game;
use hs_automaton::states::*;

type FNTrigger<T, U> = fn(Game<Trigger<T, U>>) -> Result<Game<Trigger<T, U>>, Game<Finished>>;

#[derive(Debug)]
pub struct TriggerWrapper<T, U>
where
    T: Timing,
    U: Triggerable,
{
    handler: FNTrigger<T, U>,
    phantom: PhantomData<(T, U)>,
}

impl<T, U> Clone for TriggerWrapper<T, U>
where
    T: Timing,
    U: Triggerable,
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
    T: Timing,
    U: Triggerable,
    EnumerationTiming: FromType<T>,
    EnumerationTrigger: FromType<U>,
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
pub struct ListenerEntry(EnumerationTiming, EnumerationTrigger, *const ());

impl ListenerEntry {
    fn new(timing: EnumerationTiming, trigger: EnumerationTrigger, ptr: *const ()) -> Self {
        ListenerEntry(timing, trigger, ptr)
    }
}

impl<T, U> From<TriggerWrapper<T, U>> for ListenerEntry
where
    T: Timing,
    U: Triggerable,
    EnumerationTiming: FromType<T>,
    EnumerationTrigger: FromType<U>,
{
    fn from(x: TriggerWrapper<T, U>) -> Self {
        let timing = <EnumerationTiming as FromType<T>>::from_type();
        let trigger = <EnumerationTrigger as FromType<U>>::from_type();
        let transmuted = x.handler as *const ();
        ListenerEntry::new(timing, trigger, transmuted)
    }
}

impl<T, U> TryFrom<ListenerEntry> for TriggerWrapper<T, U>
where
    T: Timing,
    U: Triggerable,
    EnumerationTiming: FromType<T>,
    EnumerationTrigger: FromType<U>,
{
    type Error = String;

    fn try_from(x: ListenerEntry) -> Result<Self, Self::Error> {
        let timing = <EnumerationTiming as FromType<T>>::from_type();
        let trigger = <EnumerationTrigger as FromType<U>>::from_type();

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
    pub pre_actions: Vec<ListenerEntry>,
    pub peri_actions: Vec<ListenerEntry>,
    pub post_actions: Vec<ListenerEntry>,
    pub pure_triggers: Vec<ListenerEntry>, // Non action related trigger listeners?
}

// TODO; Use Medici-Macros and move actual implementation BACK into 'impl ListenerService'!
// The intention is to use the blanket_impl!{} macro to easily copy ONE implementation
// multiple times with certain identifiers replaced.
macro_rules! add_entry {
    ($method_name:ident ; $container:ident) => {
        pub fn $method_name<T, U>(&mut self, handler: FNTrigger<T, U>) -> Result<(), String>
        where
            T: Timing,
            U: Triggerable,
            EnumerationTiming: FromType<T>,
            EnumerationTrigger: FromType<U>,
        {
            let wrapper = TriggerWrapper::<T, U>::new(handler);
            self.$container.push(wrapper.into());
            Ok(())
        }
    }
}

macro_rules! retrieve_entry {
    ($method_name:ident ; $container:ident) => {
        pub fn $method_name<T, U>(&self) -> impl Iterator<Item = &ListenerEntry>
        where
            T: Timing,
            U: Triggerable,
            EnumerationTiming: FromType<T>,
            EnumerationTrigger: FromType<U>,
        {
            self.$container
                .iter()
                .filter(|l| l.0 == <EnumerationTiming as FromType<T>>::from_type())
                .filter(|l| l.1 == <EnumerationTrigger as FromType<U>>::from_type())
        }
    }
}

impl ListenerService {
    add_entry!(add_peri_action; peri_actions);
    add_entry!(add_pure_trigger; pure_triggers);

    retrieve_entry!(retrieve_peri_actions; peri_actions);
    retrieve_entry!(retrieve_pure_triggers; pure_triggers);
}
