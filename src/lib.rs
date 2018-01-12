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

mod automata;
mod containers;
mod hs_automaton;

use timing_traits::Timing;

use containers::games::Game;
use containers::entities::EntityService;
use containers::tapes::TapeService;
use containers::listeners::ListenerService;
use automata::pushdown_automaton::{PullupInto, PushdownInto};
use hs_automaton::states::*;
use hs_automaton::states::global_states::timing;

fn run_triggers(
    x: Game<Effect<timing::Pre, EndTurn>>,
) -> Result<Game<Effect<timing::Pre, EndTurn>>, Game<Finished>> {
    let pre_trigger: Game<Trigger<timing::Pre, EndTurn>> = x.pushdown();
    let peri_trigger: Game<Trigger<timing::Peri, EndTurn>> = pre_trigger.pushdown();
    let post_trigger: Game<Trigger<timing::Post, EndTurn>> = peri_trigger.pushdown();

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

pub fn entry() {
    let new_game = Game {
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

    // Do stuff
    let first_turn = end_turn(new_game).expect("Game finished");
    let second_turn = end_turn(first_turn).expect("Game finished");

    // let item = Game { state: Pre(Action { activity: EndTurn() }) };

    // // let pushed: Game<Pre<Trigger<Pre<EndTurn>>>> = item.pushdown();
    // let pushed = item.pushdown();
    // // let item = pushed.pullup();

    println!("OK - Finished");
}
