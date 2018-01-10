use hs_automaton::states::global_states::{Actionable, Triggerable};

#[derive(Debug, ActionState, TriggerState)]
pub struct EndTurn();
