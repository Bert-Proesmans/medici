use std::fmt::Debug;
use std::marker::PhantomData;

use containers::games::Game;
use hs_automaton::states::*;

// TODO The following enums have to be constructed before compilation
// Maybe execute a pre-build event?
// Or fallback to macro magic..
#[derive(Debug)]
pub enum EnumerationTiming {
    Pre,
    Peri,
    Post,
}

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
    H: Fn(Game<Trigger<T, U>>) -> Result<Game<Trigger<T, U>>, Game<Finished>>,
{
    handler: H,
    phantom: PhantomData<(T, U)>,
}

#[derive(Debug)]
pub struct ListenerEntry(pub EnumerationTiming, pub EnumerationTrigger, pub Box<Listener>);

#[derive(Debug)]
pub struct ListenerService {
    // Contains all objects which should be invoked when certain requirements are met.
    // pub pre_action: u32,
    // pub peri_action: u32,
    // pub post_action: u32,
    // pub excluded_action: u32, // Non action related trigger listeners?
}
