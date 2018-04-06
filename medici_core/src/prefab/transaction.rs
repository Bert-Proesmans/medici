//! Types for provisioning the next state when performing a
//! transition within the state machine.

use marker::Transaction;

/// Empty Transaction object.
///
/// The name Epsilon is derived from NFA's where they indicate zero-step transitions
/// between states.
/// In this design it's intention is to convey that no Transition information is
/// necessary to transition into a next state.
#[derive(Debug, Clone, Copy)]
pub struct Epsilon;
impl Transaction for Epsilon {}
