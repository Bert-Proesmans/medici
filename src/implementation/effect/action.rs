//! Module containing the code for all direct reactions on user input.
//!
//! Functionality defined in this module is explicit mapping on a choice
//! made by the current player (during his turn). Eg: Ending turn action
//! maps to [`end_turn`].

use failure::Error;

use medici_core::stm::*;

use state_machine::prelude::*;
use state_machine::state::prelude::*;
use state_machine::transaction::*;

use implementation::runtime::exec_triggers;

/// Method invoked by user action: EndTurn
pub fn end_turn(machine: Machine<Wait<Input>>) -> Result<Machine<Wait<Input>>, Error> {
    // Transition into the desired state.
    let mut action: Machine<Action<EndTurn>> = machine.transition(Epsilon);
    // Execute all listeners for this action.
    let mut effect: Machine<Effect<EndTurn>> = action.pushdown(Epsilon);
    effect = exec_triggers(effect, Epsilon)?;
    // Pull back up into the Action state..
    action = effect.pullup()?;
    // .. and transition the state machine back into a waiting for input state.
    Ok(action.transition(Epsilon))
}
