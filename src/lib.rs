#![feature(attr_literals)] // Used for 'from_generic_derive' macro
#![feature(conservative_impl_trait)] // Used for 'fn() -> impl Iterator<Item=X>'
#![feature(try_from)]

#[macro_use]
extern crate action_macros;
extern crate action_traits;
#[macro_use]
extern crate from_generic_macros;
extern crate from_generic_traits;
#[macro_use]
extern crate timing_macros;
extern crate timing_traits;
#[macro_use]
extern crate wait_macros;
extern crate wait_traits;

use std::convert::TryInto;

mod automata;
mod containers;
mod hs_automaton;

use timing_traits::Timing;

use containers::games::Game;
use containers::entities::EntityService;
use containers::tapes::TapeService;
use containers::listeners::{ListenerService, TriggerWrapper};
use automata::pushdown_automaton::{PullupInto, PushdownInto};
use hs_automaton::states::*;
use hs_automaton::states::global_states::timing;
use hs_automaton::states::global_states::timing::EnumerationTiming;
use hs_automaton::states::action_states::EnumerationTrigger;

fn run_triggers(
    x: Game<Effect<timing::Pre, EndTurn>>,
) -> Result<Game<Effect<timing::Pre, EndTurn>>, Game<Finished>> {
    let pre_trigger: Game<Trigger<timing::Pre, EndTurn>> = x.pushdown();
    let mut peri_trigger: Game<Trigger<timing::Peri, EndTurn>> = pre_trigger.pushdown();

    let per_listeners: Vec<_> = peri_trigger
        .listeners
        .retrieve_peri_action::<timing::Peri, EndTurn>()
        .map(|l| l.clone())
        .collect();

    // Cast and run each listener
    for listener in per_listeners.into_iter() {
        let wrapper: TriggerWrapper<timing::Peri, EndTurn> = match listener.try_into() {
            Ok(item) => item,
            // TODO: Notify user?
            Err(_) => panic!(),
        };
        peri_trigger = (wrapper.get_handler())(peri_trigger)?;
    }

    let post_trigger: Game<Trigger<timing::Post, EndTurn>> = peri_trigger.pushdown();

    // TODO; Run operations for each state we enter!

    let pulling_up: Game<Trigger<timing::Peri, EndTurn>> = post_trigger.pullup();
    let pulling_up: Game<Trigger<timing::Pre, EndTurn>> = pulling_up.pullup();
    let pulling_up: Game<Effect<timing::Pre, EndTurn>> = pulling_up.pullup();
    Ok(pulling_up)
}

fn run_death_phase<T, U>(x: Game<Death<T, U>>) -> Result<Game<Death<T, U>>, Game<Finished>>
where
    T: Timing,
{
    Ok(x)
}

fn end_turn(x: Game<Wait<Input>>) -> Result<Game<Wait<Input>>, Game<Finished>> {
    let pre_action: Game<Action<timing::Pre, EndTurn>> = x.into();
    // Execute pre_action handlers
    let pre_effect: Game<Effect<timing::Pre, EndTurn>> = pre_action.pushdown();
    let pre_effect = run_triggers(pre_effect)?;
    // Execute death phase
    let pre_action: Game<Action<timing::Pre, EndTurn>> = pre_effect.pullup();
    let pre_action_finished = run_death_phase(pre_action.into())?;

    // // Run actual action phase
    // let action = pre_action_finished.into();
    // // Execute action handlers
    // let action = run_triggers(action.pushdown())?;
    // // Execute death phase
    // let action = action.pullup();
    // let action_finished = run_death_phase(action.into());

    // let peri_action = pre_action_finished.into();
    let post_action: Game<Action<timing::Post, EndTurn>> = pre_action_finished.into();
    let post_action_finished: Game<Death<timing::Post, EndTurn>> = post_action.into();

    // Set current state back to awaiting input
    Ok(post_action_finished.into())
}

fn turn_end_trigger(
    x: Game<Trigger<timing::Peri, EndTurn>>,
) -> Result<Game<Trigger<timing::Peri, EndTurn>>, Game<Finished>> {
    println!("PERI - END TURN");
    Ok(x)
}

pub fn entry() {
    let mut new_game = Game {
        state: Wait { activity: Input() },
        entities: EntityService {},
        storage: TapeService {},
        listeners: ListenerService {
            pre_action: Vec::new(),
            peri_action: Vec::new(),
            post_action: Vec::new(),
            excluded_action: Vec::new(),
        },
    };

    // Add trigger
    new_game
        .listeners
        .add_peri_action(turn_end_trigger)
        .unwrap();

    // Do stuff
    let first_turn = end_turn(new_game).expect("Game finished");
    let second_turn = end_turn(first_turn).expect("Game finished");

    // let item = Game { state: Pre(Action { activity: EndTurn() }) };

    // // let pushed: Game<Pre<Trigger<Pre<EndTurn>>>> = item.pushdown();
    // let pushed = item.pushdown();
    // // let item = pushed.pullup();

    println!("OK - Finished");
}

#[cfg(test)]
mod tests {
    use super::entry;

    #[test]
    fn it_works() {
        entry();
    }
}
