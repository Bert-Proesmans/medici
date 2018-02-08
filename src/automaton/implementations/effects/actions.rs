use medici_traits::automata::deterministic_automaton::TransitionInto;
use medici_traits::automata::pushdown_automaton::{PullupInto, PushdownInto};

use automaton::prelude::*;
use automaton::runtime::exec_action_listeners;

pub fn end_turn(x: Game<Wait<Input>>) -> Result<Game<Wait<Input>>, Game<Finished>> {
    // Start chain of events.
    let action: Game<Action<EndTurn>> = x.transition(Epsilon());
    let effect = exec_action_listeners(action.pushdown())?;
    // TODO; Maybe do something here?

    let post_action: Game<Action<EndTurn>> = effect.pullup();
    // Set current state back to awaiting input
    Ok(post_action.transition(Epsilon()))
}
