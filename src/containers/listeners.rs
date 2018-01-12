use std::fmt::Debug;
use std::marker::PhantomData;

use timing_traits::Timing;
use from_generic_traits::FromGeneric;

use containers::games::Game;
use hs_automaton::states::*;
use hs_automaton::states::global_states::timing::EnumerationTiming;

// TODO The following enums have to be constructed before compilation
// Maybe execute a pre-build event?
// Or fallback to macro magic..

#[derive(Debug)]
pub enum EnumerationTrigger {
    StartGame,
    EndGame,
    EndTurn,
    StartTurn,
    Resource,
    Draw,
    ZoneChange,
    Death,
    Summon,
    Battlecry,
    TagChange,
}

pub trait Listener: Debug {}

#[derive(Debug)]
pub struct TriggerWrapper<T, U>
where
    T: Timing + Debug,
    U: Debug,
{
    handler: fn(Game<Trigger<T, U>>) -> Result<Game<Trigger<T, U>>, Game<Finished>>,
    phantom: PhantomData<(T, U)>,
}

impl<T, U> Listener for TriggerWrapper<T, U>
where
    T: Timing + Debug,
    U: Debug,
{
}

impl<T, U> TriggerWrapper<T, U>
where
    T: Timing + Debug,
    U: Debug,
    EnumerationTiming: FromGeneric<T>,
    EnumerationTrigger: FromGeneric<U>,
{
    fn build_entry(
        handler: fn(Game<Trigger<T, U>>) -> Result<Game<Trigger<T, U>>, Game<Finished>>,
    ) -> ListenerEntry {
        let timing = <EnumerationTiming as FromGeneric<T>>::from_generic();
        // let trigger = EnumerationTrigger::from_generic::<U>();
        let wrapper = Self { handler: handler, phantom: PhantomData };
        //
        // let item = ListenerEntry(timing, trigger, Box::new(wrapper));

        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ListenerEntry(
    pub timing::EnumerationTiming,
    pub EnumerationTrigger,
    pub Box<Listener>,
);

#[derive(Debug)]
pub struct ListenerService {
    // Contains all objects which should be invoked when certain requirements are met.
    pub pre_action: Vec<ListenerEntry>,
    pub peri_action: Vec<ListenerEntry>,
    pub post_action: Vec<ListenerEntry>,
    pub excluded_action: Vec<ListenerEntry>, // Non action related trigger listeners?
}
