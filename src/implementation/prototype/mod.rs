//! Specialized prototypes for our state machine.

use value_from_type_macros::value_from_type;

mod definition;
pub mod game;

// Re-export all proto-definitions
pub use self::definition::{Game, Player, ProtoItem};
