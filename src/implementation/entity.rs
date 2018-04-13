//! Specialized implementation of [`EntityStruct`] for our state machine.

use medici_core::prefab::entity::EntityStruct;

use implementation::prototype::ProtoItem;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
/// Enumeration of all entity property keys.
pub enum EntityTags {
    // TODO
}

/// The specialized entity structure for our state machine.
pub type Entity = EntityStruct<EntityTags, ProtoItem>;
