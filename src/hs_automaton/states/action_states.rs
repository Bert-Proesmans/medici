#[derive(Debug, ActionState, TriggerState)]
pub struct EndTurn();

#[derive(Debug, Clone, PartialEq, FromGeneric)]
pub enum EnumerationTrigger {
    // StartGame,
    // EndGame,
    #[generic("EndTurn")] EndTurn,
    // StartTurn,
    // Resource,
    // Draw,
    // ZoneChange,
    // Death,
    // Summon,
    // Battlecry,
    // TagChange,
}
