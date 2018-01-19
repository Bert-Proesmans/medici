use std::convert::TryInto;

use medici_traits::pushdown_automaton::{PushdownInto, PullupInto};
use medici_traits::action_traits::{Actionable, Triggerable};
use medici_traits::timing_traits::Timing;
use value_from_type_traits::FromType;

use containers::games::Game;
use containers::listeners::TriggerWrapper;
use hs_automaton::states::*;

pub fn exec_triggers(x: Game<Effect<Pre, EndTurn>>) -> Result<Game<Effect<Pre, EndTurn>>, Game<Finished>> {
    let pre_trigger: Game<Trigger<Pre, EndTurn>> = x.pushdown();
    let peri_trigger: Game<Trigger<Peri, EndTurn>> = pre_trigger.pushdown();
    // Execute all listeners for the specified state.
    let peri_trigger = exec_trigger_step(peri_trigger)?;

    let post_trigger: Game<Trigger<Post, EndTurn>> = peri_trigger.pushdown();

    // TODO; Run operations for each state we enter!

    let pulling_up: Game<Trigger<Peri, EndTurn>> = post_trigger.pullup();
    let pulling_up: Game<Trigger<Pre, EndTurn>> = pulling_up.pullup();
    let pulling_up: Game<Effect<Pre, EndTurn>> = pulling_up.pullup();
    Ok(pulling_up)
}

pub fn exec_trigger_step<T, U>(state: Game<Trigger<T, U>>) -> Result<Game<Trigger<T,U>>, Game<Finished>> 
where
    T: Timing,
    U: Triggerable,
    EnumerationTiming: FromType<T>,
    EnumerationTrigger: FromType<U>,
{
    let mut s = state;

    let listeners: Vec<_> = s.listeners
        .retrieve_pure_triggers::<T, U>()
        .map(|l| l.clone())
        // Collect must be done to drop the immutable reference on x.
        .collect();

    // Cast and run each listener
    for l in listeners.into_iter() {
        // The failure case is unreachable if no rogue entity inserted their custom
        // entry.
        let wrapper: TriggerWrapper<T, U> = match l.try_into() {
            Ok(item) => item,
            // TODO: Notify user?
            Err(_) => panic!("Shit's on fire, Yo!"),
        };
        s = (wrapper.get_handler())(s)?;
    }

    Ok(s)
}

pub fn exec_death_phase<T, U>(x: Game<Death<T, U>>) -> Result<Game<Death<T, U>>, Game<Finished>>
where
    T: Timing,
    U: Actionable,
    EnumerationTiming: FromType<T>,
    EnumerationTrigger: FromType<U>,
{
    Ok(x)
}
