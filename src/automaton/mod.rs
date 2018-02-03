mod config;
mod setup_config;
pub mod implementations;
pub mod card_sets;
pub mod runtime;

/* Broker overlay between configured automaton and medici implementations. */

pub mod prelude {
    // Make sure to NOT export
    //      super::config::states::*
    // because all states are leaked into that module because of access limitations
    // when building transition methods!

    pub use super::config::{Card, Entity, Game, GameTags};
    pub use super::setup_config::SetupConfig;
    pub use super::config::states::global::{Action, Death, Effect, Finished, Trigger, Wait};
    pub use super::config::states::waitable::{Input, Start};
    pub use super::config::states::timing::{Peri, Post, Pre};
    pub use super::config::states::triggerable::EndTurn;
    // pub use super::config::states::triggerable::*;
    // Don't confuse this type with the Game structure!
    pub use super::config::prototypes::{Game as GameProto, GameMut as GameProtoMut};
}

pub mod states {
    pub use super::config::states::global;
    pub use super::config::states::waitable;
    pub use super::config::states::timing;
    pub use super::config::states::triggerable;
}

pub mod prototypes {
    pub use super::config::prototypes::*;
}

/* ************************** */
