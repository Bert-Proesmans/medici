use medici_traits::pushdown_automaton::{PushdownInto, PullupInto};

use containers::games::Game;
use hs_automaton::states::*;

use hs_automaton::runtime::{exec_triggers, exec_death_phase};

pub fn end_turn(x: Game<Wait<Input>>) -> Result<Game<Wait<Input>>, Game<Finished>> {
    let pre_action: Game<Action<Pre, EndTurn>> = x.into();
    // Execute pre_action handlers
    let pre_effect: Game<Effect<Pre, EndTurn>> = pre_action.pushdown();
    let pre_effect = exec_triggers(pre_effect)?;
    // Execute death phase
    let pre_action: Game<Action<Pre, EndTurn>> = pre_effect.pullup();
    let pre_action_finished = exec_death_phase(pre_action.into())?;

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
