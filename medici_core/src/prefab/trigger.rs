#![value_from_type(TriggerItem)]

//! Module containing ready-to-use types which can be used to construct
//! trigger constraints.

use marker::{TriggerEnumerator, TriggerMarker};

/// Type representing the event when a game starts.
#[derive(Debug, Clone, Copy)]
pub struct GameStart();
impl TriggerMarker for GameStart {}

/// Type representing the event when a game ends.
#[derive(Debug, Clone, Copy)]
pub struct GameEnd();
impl TriggerMarker for GameEnd {}

/// Type representing the event when a player's turn starts.
#[derive(Debug, Clone, Copy)]
pub struct TurnStart();
impl TriggerMarker for TurnStart {}

/// Type representing the event when a player's turn ends.
#[derive(Debug, Clone, Copy)]
pub struct TurnEnd();
impl TriggerMarker for TurnEnd {}

// value_from_type builds an enumeration of all structures defined within this module.
// The first parameter of the macro will be the identifier of the generated
// enumeration == [`TriggerItem`].
// The enumeration itself will be defined INSIDE this module.
impl TriggerEnumerator for TriggerItem {}
