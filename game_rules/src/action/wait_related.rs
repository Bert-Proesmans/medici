//! Module containing the code for all direct reactions on user input.
//!
//! Functionality defined in this module is explicit mapping on a choice
//! made by the current player (during his turn). Eg: Ending turn action
//! maps to [`end_turn`].

use game_system::prelude::*;
use game_system::runtime::exec_triggers;

/// Method invoked to start the game.
pub fn start_game(
    machine: Machine<Wait<Start>, EmptyStack>,
) -> Result<Machine<Wait<Input>, EmptyStack>, MachineError> {
    // Transition into the desired state for trigger execution.
    let mut action: Machine<Action<Start>, _> = machine.transition(transaction::Epsilon);
    // Execute all listeners for this action.
    let mut effect: Machine<Effect<Start>, _> = action.pushdown(transaction::Epsilon);
    effect = exec_triggers(effect, transaction::Epsilon)?;
    // Pull back up into the Action state..
    action = effect.pullup()?;
    // .. and transition the state machine back into a waiting for input state.
    Ok(action.transition(transaction::Epsilon))
}

/// Method invoked by user action: EndTurn
pub fn end_turn(
    machine: Machine<Wait<Input>, EmptyStack>,
) -> Result<Machine<Wait<Input>, EmptyStack>, MachineError> {
    // Transition into the desired state.
    let mut action: Machine<Action<EndTurn>, _> = machine.transition(transaction::Epsilon);
    // Execute all listeners for this action.
    let mut effect: Machine<Effect<EndTurn>, _> = action.pushdown(transaction::Epsilon);
    effect = exec_triggers(effect, transaction::Epsilon)?;
    // Pull back up into the Action state..
    action = effect.pullup()?;
    // .. and transition the state machine back into a waiting for input state.
    Ok(action.transition(transaction::Epsilon))
}
