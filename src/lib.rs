mod automata;
mod containers;
mod hs_automaton;

use std::convert::From;

use containers::games::Game;
use containers::entities::EntityService;
use containers::tapes::TapeService;
use containers::listeners::ListenerService;
use automata::pushdown_automaton::{Pushdown, Pullup};
use hs_automaton::states::*;
use hs_automaton::soft_transitions;

fn run_triggers(
    x: Game<Effect<Pre, EndTurn>>,
) -> Result<Game<Effect<Pre, EndTurn>>, Game<Finished>> {
    let pre_trigger: Game<Trigger<Pre, EndTurn>> = x.pushdown();
    let peri_trigger: Game<Trigger<Peri, EndTurn>> = pre_trigger.pushdown();
    let post_trigger: Game<Trigger<Post, EndTurn>> = peri_trigger.pushdown();

    let pulling_up = Game::<Trigger<Peri, EndTurn>>::pullup(post_trigger);
    let pulling_up = Game::<Trigger<Pre, EndTurn>>::pullup(pulling_up);
    let pulling_up = Game::<Effect<Pre, EndTurn>>::pullup(pulling_up);
    Ok(pulling_up)
}

fn run_death_phase<T, U>(x: Game<Death<T, U>>) -> Result<Game<Death<T, U>>, Game<Finished>> {
    Ok(x)
}

fn end_turn(x: Game<Wait<Input>>) -> Result<Game<Wait<Input>>, Game<Finished>> {
    let pre_action: Game<Action<Pre, EndTurn>> = x.into();
    // Execute pre_action handlers
    let pre_effect: Game<Effect<Pre, EndTurn>> = pre_action.pushdown();
    let pre_effect = run_triggers(pre_effect)?;
    // Execute death phase
    let pre_action = Game::<Action<Pre, EndTurn>>::pullup(pre_effect);
    let pre_action_finished = run_death_phase(pre_action.into())?;

    // // Run actual action phase
    // let action = pre_action_finished.into();
    // // Execute action handlers
    // let action = run_triggers(action.pushdown())?;
    // // Execute death phase
    // let action = action.pullup();
    // let action_finished = run_death_phase(action.into());

    // let peri_action = pre_action_finished.into();
    let post_action: Game<Action<Post, EndTurn>> = pre_action_finished.into();
    let post_action_finished: Game<Death<Post, EndTurn>> = post_action.into();

    // Set current state back to awaiting input
    Ok(post_action_finished.into())
}


pub fn entry() {
    let new_game = Game {
        state: Wait { activity: Input() },
        entities: EntityService {},
        storage: TapeService {},
        listeners: ListenerService {},
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
