use std::convert::TryInto;
use std::iter::IntoIterator;

// use medici_traits::automata::pushdown_automaton::{PullupInto, PushdownInto};
use medici_traits::prelude::*;

use automaton::prelude::*;
use automaton::states::timing::EnumerationTiming;
use automaton::states::triggerable::EnumerationTrigger;
use containers::listeners::{ListenerEntry, TriggerWrapper};

fn fetch_listeners<T, U>(x: &Game<Trigger<T, U>>) -> Vec<ListenerEntry>
where
    T: Timing + IntoEnum<EnumerationTiming>,
    U: Triggerable + IntoEnum<EnumerationTrigger>,
{
    let collector: Vec<_> = x.listeners.retrieve_triggers::<T, U>().cloned().collect();
    collector
}

pub fn exec_action_listeners<U>(x: Game<Effect<U>>) -> Result<Game<Effect<U>>, Game<Finished>>
where
    U: Actionable + IntoEnum<EnumerationTrigger>,

    Game<Effect<U>>: Into<Game<Trigger<Pre, U>>>,
    Game<Trigger<Pre, U>>: Into<Game<Death<Pre, U>>>,
    Game<Trigger<Peri, U>>: Into<Game<Death<Peri, U>>>,
    Game<Trigger<Post, U>>: Into<Game<Death<Post, U>>>,

    Game<Death<Pre, U>>: Into<Game<Trigger<Peri, U>>>,
    Game<Death<Peri, U>>: Into<Game<Trigger<Post, U>>>,
    Game<Death<Post, U>>: Into<Game<Effect<U>>>,
{
    // PRE
    let mut pre_trigger: Game<Trigger<Pre, U>> = x.into();
    let listeners = fetch_listeners(&pre_trigger);
    pre_trigger = exec_trigger_step(pre_trigger, listeners)?;
    let x: Game<Death<Pre, U>> = pre_trigger.into();

    // PERI
    let mut peri_trigger: Game<Trigger<Peri, U>> = x.into();
    let listeners = fetch_listeners(&peri_trigger);
    peri_trigger = exec_trigger_step(peri_trigger, listeners)?;
    let x: Game<Death<Peri, U>> = peri_trigger.into();

    // POST
    let mut post_trigger: Game<Trigger<Post, U>> = x.into();
    let listeners = fetch_listeners(&post_trigger);
    post_trigger = exec_trigger_step(post_trigger, listeners)?;
    let mut x: Game<Death<Post, U>> = post_trigger.into();

    // Note: Death phase is only executed at the end of the action effects!
    x = exec_death_phase(x)?;
    Ok(x.into())
}

pub fn exec_trigger_step<T, U, I>(
    mut state: Game<Trigger<T, U>>,
    listeners: I,
) -> Result<Game<Trigger<T, U>>, Game<Finished>>
where
    T: Timing + IntoEnum<EnumerationTiming>,
    U: Triggerable + IntoEnum<EnumerationTrigger>,
    I: IntoIterator<Item = ListenerEntry>,
{
    // Cast and run each listener
    for l in listeners.into_iter() {
        // The failure case is unreachable if no rogue entity inserted their custom
        // entry.
        let wrapper: TriggerWrapper<T, U> = match l.try_into() {
            Ok(item) => item,
            // TODO: Notify user?
            Err(_) => panic!("Shit's on fire, Yo!"),
        };
        state = (wrapper.get_handler())(state)?;
    }

    Ok(state)
}

pub fn exec_death_phase<T, U>(x: Game<Death<T, U>>) -> Result<Game<Death<T, U>>, Game<Finished>>
where
    T: Timing + IntoEnum<EnumerationTiming>,
    U: Triggerable + IntoEnum<EnumerationTrigger>,
{
    Ok(x)
}
