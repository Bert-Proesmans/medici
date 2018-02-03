use std::clone::Clone;
use std::convert::TryFrom;
use std::default::Default;

use medici_traits::prelude::*;
use automaton::prelude::*;
use automaton::states::timing::EnumerationTiming;
use automaton::states::triggerable::EnumerationTrigger;

type FNTrigger<T, U> = fn(Game<Trigger<T, U>>) -> Result<Game<Trigger<T, U>>, Game<Finished>>;

#[derive(Debug)]
pub struct TriggerWrapper<T, U>
where
    T: Timing,
    U: Triggerable,
{
    handler: FNTrigger<T, U>,
}

impl<T, U> Clone for TriggerWrapper<T, U>
where
    T: Timing,
    U: Triggerable,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
        }
    }
}

impl<T, U> TriggerWrapper<T, U>
where
    T: Timing,
    U: Triggerable,
{
    fn new(handler: FNTrigger<T, U>) -> Self {
        Self { handler: handler }
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
    T: Timing + IntoEnum<EnumerationTiming>,
    U: Triggerable + IntoEnum<EnumerationTrigger>,
{
    fn from(x: TriggerWrapper<T, U>) -> Self {
        let timing: EnumerationTiming = T::into_enum();
        let trigger: EnumerationTrigger = U::into_enum();
        let transmuted = x.handler as *const ();
        ListenerEntry::new(timing, trigger, transmuted)
    }
}

impl<T, U> TryFrom<ListenerEntry> for TriggerWrapper<T, U>
where
    T: Timing + IntoEnum<EnumerationTiming>,
    U: Triggerable + IntoEnum<EnumerationTrigger>,
{
    type Error = String;

    fn try_from(x: ListenerEntry) -> Result<Self, Self::Error> {
        let timing: EnumerationTiming = T::into_enum();
        let trigger: EnumerationTrigger = U::into_enum();

        if x.2.is_null() {
            return Err("Handler is NULL!".into());
        }

        if x.0 == timing && x.1 == trigger {
            unsafe {
                let transmuted: FNTrigger<T, U> = ::std::mem::transmute(x.2);
                Ok(TriggerWrapper {
                    handler: transmuted,
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
    pre_listener: Vec<ListenerEntry>,
    peri_listener: Vec<ListenerEntry>,
    post_listener: Vec<ListenerEntry>,
}

// DBG
impl Default for ListenerService {
    fn default() -> Self {
        ListenerService {
            pre_listener: vec![],
            peri_listener: vec![],
            post_listener: vec![],
        }
    }
}

impl ListenerService {
    pub fn new(c: &SetupConfig) -> Result<Self, ()> {
        Ok(Self {
            pre_listener: vec![],
            peri_listener: vec![],
            post_listener: vec![],
        })
    }
}

impl ListenerService {
    /* 
        Action dependant listeners.
        These triggers have the purpose to directly interact with 
        performed actions.
     */

    fn get_container<AT>(&self) -> &Vec<ListenerEntry>
    where
        AT: Timing + IntoEnum<EnumerationTiming>,
    {
        match AT::into_enum() {
            EnumerationTiming::Pre => &self.pre_listener,
            EnumerationTiming::Peri => &self.peri_listener,
            EnumerationTiming::Post => &self.post_listener,
        }
    }

    fn get_container_mut<AT>(&mut self) -> &mut Vec<ListenerEntry>
    where
        AT: Timing + IntoEnum<EnumerationTiming>,
    {
        match AT::into_enum() {
            EnumerationTiming::Pre => &mut self.pre_listener,
            EnumerationTiming::Peri => &mut self.peri_listener,
            EnumerationTiming::Post => &mut self.post_listener,
        }
    }

    pub fn add_trigger<T, U>(&mut self, handler: FNTrigger<T, U>) -> Result<(), String>
    where
        T: Timing + IntoEnum<EnumerationTiming>,
        U: Triggerable + IntoEnum<EnumerationTrigger>,
    {
        let wrapper = TriggerWrapper::<T, U>::new(handler);
        let container = self.get_container_mut::<T>();
        container.push(wrapper.into());
        Ok(())
    }

    pub fn retrieve_triggers<T, U>(&self) -> impl Iterator<Item = &ListenerEntry>
    where
        T: Timing + IntoEnum<EnumerationTiming>,
        U: Triggerable + IntoEnum<EnumerationTrigger>,
    {
        let container = self.get_container::<T>();
        container
            .iter()
            // .filter(|l| l.0 == T::into_enum()) // Unnecessary if triggers are stored seperately
            .filter(|l| l.1 == U::into_enum())
    }
}
