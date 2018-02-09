use medici_traits::automata::deterministic_automaton::TransitionInto;
use medici_traits::automata::pushdown_automaton::{PullupInto, PushdownInto};

use automaton::prelude::*;
use automaton::runtime::exec_action_listeners;

// Method invoked by user action: EndTurn
pub fn end_turn(x: Game<Wait<Input>>) -> Result<Game<Wait<Input>>, Game<Finished>> {
	// Transition into the desired state.
    let action: Game<Action<EndTurn>> = x.transition(Epsilon());
    // Execute all listeners for this action.
    let effect = exec_action_listeners(action.pushdown())?;

    // Circle back towards the type we received..
    let post_action: Game<Action<EndTurn>> = effect.pullup();
    // .. and return the requested state.
    Ok(post_action.transition(Epsilon()))
}
