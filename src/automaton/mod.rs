mod implementations;
mod config;

/* Broker overlay between configured automaton and medici implementations. */

pub mod prelude {
    // Make sure to NOT export
    //      super::config::states::*
    // because all states are leaked into that module because of access limitations
    // when building transition methods!
    
    pub use super::config::{Game, Entity, GameTags};
	pub use super::config::states::global::{Wait, Action, Finished, Effect, Trigger, Death};
    pub use super::config::states::waitable::Input;
    pub use super::config::states::timing::{Pre, Peri, Post};
    pub use super::config::states::actionable::{EndTurn};
    // pub use super::config::states::triggerable::*;
    // Don't confuse this type with the Game structure!
    pub use super::config::prototypes::Game as GameProto;
}

pub mod states {
	pub use super::config::states::global;
    pub use super::config::states::waitable;
    pub use super::config::states::timing;
    pub use super::config::states::actionable;
    pub use super::config::states::triggerable;
}

pub mod prototypes {
	pub use super::config::prototypes::*;
}

/* ************************** */
