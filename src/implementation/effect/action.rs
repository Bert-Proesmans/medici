//! Module containing the code for all direct reactions on user input.
//!
//! Functionality defined in this module is explicit mapping on a choice
//! made by the current player (during his turn). Eg: Ending turn action
//! maps to [`end_turn`].

use failure::Error;

use medici_core::ctstack::EmptyStack;
use medici_core::stm::checked::*;

use state_machine::prelude::*;
use state_machine::state::prelude::*;
use state_machine::transaction::*;

use implementation::runtime::exec_triggers;

/// Method invoked to start the game.
pub fn start_game(
    machine: Machine<Wait<Start>, EmptyStack>,
) -> Result<Machine<Wait<Input>, EmptyStack>, Error> {
    // Transition into the desired state for trigger execution.
    let mut action: Machine<Action<Start>, _> = machine.transition(Epsilon);
    // Execute all listeners for this action.
    let mut effect: Machine<Effect<Start>, _> = action.pushdown(Epsilon);
    effect = exec_triggers(effect, Epsilon)?;
    // Pull back up into the Action state..
    action = effect.pullup()?;
    // .. and transition the state machine back into a waiting for input state.
    Ok(action.transition(Epsilon))
}

/// Method invoked by user action: EndTurn
pub fn end_turn(
    machine: Machine<Wait<Input>, EmptyStack>,
) -> Result<Machine<Wait<Input>, EmptyStack>, Error> {
    // Transition into the desired state.
    let mut action: Machine<Action<EndTurn>, _> = machine.transition(Epsilon);
    // Execute all listeners for this action.
    let mut effect: Machine<Effect<EndTurn>, _> = action.pushdown(Epsilon);
    effect = exec_triggers(effect, Epsilon)?;
    // Pull back up into the Action state..
    action = effect.pullup()?;
    // .. and transition the state machine back into a waiting for input state.
    Ok(action.transition(Epsilon))
}
