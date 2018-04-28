//! Specialized implementation of [`EntityStruct`] for our state machine.

use medici_core::prefab::entity::EntityStruct;

use prototype::ProtoItem;
use tag::EntityTags;

/// [Re-export] Unique ID value for the Game entity.
pub use medici_core::prefab::entity::GAME_E_ID;
/// The specialized entity structure for our state machine.
pub type Entity = EntityStruct<EntityTags, ProtoItem>;
