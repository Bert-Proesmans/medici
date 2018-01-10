use std::fmt::Debug;
use std::marker::PhantomData;

use containers::games::Game;
use hs_automaton::states::*;
use hs_automaton::states::global_states::timing;

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
pub struct TriggerWrapper<T, U, H>
where
    T: timing::Timing + Into<timing::EnumerationTiming>,
    U: Into<EnumerationTrigger>,
    H: Fn(Game<Trigger<T, U>>) -> Result<Game<Trigger<T, U>>, Game<Finished>>,
{
    handler: H,
    phantom: PhantomData<(T, U)>,
}

impl<T, U, H> Listener for TriggerWrapper<T, U, H> 
where
    T: timing::Timing + Into<timing::EnumerationTiming>,
    U: Into<EnumerationTrigger> + Debug,
    H: Fn(Game<Trigger<T, U>>) -> Result<Game<Trigger<T, U>>, Game<Finished>> + Debug,
{
}

impl<T, U, H> TriggerWrapper<T, U, H>
where
    T: timing::Timing + Into<timing::EnumerationTiming>,
    U: Into<EnumerationTrigger>,
    H: Fn(Game<Trigger<T, U>>) -> Result<Game<Trigger<T, U>>, Game<Finished>> + Debug,
{
    fn build_entry(timing: T, trigger: U, handler: H) -> ListenerEntry {
        let timing = timing.into();
        let trigger = trigger.into();
        let wrapper = Self { handler: handler, phantom: PhantomData };
        //
        ListenerEntry(timing, trigger, Box::new(wrapper))
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
