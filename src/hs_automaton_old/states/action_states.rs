use value_from_type_macros::value_from_type;
use medici_macros::ActionState;

pub use self::custom::{EndTurn, EnumerationTrigger};

mod custom {
    #![value_from_type(EnumerationTrigger)]

    #[derive(Debug, ActionState)]
    pub struct EndTurn();

    /*
    #[derive(Debug, Clone, PartialEq, FromGeneric)]
    pub enum EnumerationTrigger {
        // StartGame,
        // EndGame,
        // EndTurn,
        // StartTurn,
        // Resource,
        // Draw,
        // ZoneChange,
        // Death,
        // Summon,
        // Battlecry,
        // TagChange,
    }
    */
}
