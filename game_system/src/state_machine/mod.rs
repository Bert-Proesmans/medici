//! State machine implementation for the game: Goose game.

pub mod config;
pub mod machine;
pub mod marker;
pub mod state;
pub mod transaction;
pub mod transitions;

pub mod prelude {
    //! This module contains all the types you need to get started
    //! with the state machine defined inside the parent module.

    /// Re-export of the game configuration.
    pub use super::config::SetupConfig;
    /// Re-export of the state machine.
    pub use super::machine::Machine;
}
