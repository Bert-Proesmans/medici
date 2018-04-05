#![value_from_type(TriggerItem)]

use marker::{Trigger, TriggerEnumerator};

/// Type representing the event when a game starts.
#[derive(Debug, Clone, Copy)]
pub struct GameStart();
impl Trigger for GameStart {}

/// Type representing the event when a game ends.
#[derive(Debug, Clone, Copy)]
pub struct GameEnd();
impl Trigger for GameEnd {}

/// Type representing the event when a player's turn starts.
#[derive(Debug, Clone, Copy)]
pub struct TurnStart();
impl Trigger for TurnStart {}

/// Type representing the event when a player's turn ends.
#[derive(Debug, Clone, Copy)]
pub struct TurnEnd();
impl Trigger for TurnEnd {}

// value_from_type builds an enumeration of all structures defined within this module.
// The first parameter of the macro will be the identifier of the generated
// enumeration == [`TriggerItem`].
// The enumeration itself will be defined INSIDE this module.
impl TriggerEnumerator for TriggerItem {}
