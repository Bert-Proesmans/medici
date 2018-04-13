#![value_from_type(ProtoItem)]
//! Module containing standard prototypes.
//!
//! The defined prototypes can be implemented in derived crates.

use function::Entity;
use marker::{ProtoEnumerator, PrototypeMarker};

use prefab::entity::Entity as EntityPrefab;

/// Prototype for game behaviour specifically targetting the [`Entity`] defined
/// within medici_core::prefab.
pub type Game<'a> = GameProto<'a, EntityPrefab>;
/// Prototype for player behaviour specifically targetting the [`Entity`] defined
/// within medici_core::prefab.
pub type Player<'a> = PlayerProto<'a, EntityPrefab>;

//
impl<'a> From<&'a EntityPrefab> for GameProto<'a, EntityPrefab> {
    fn from(x: &'a EntityPrefab) -> Self {
        GameProto(Either::Imut(x))
    }
}

impl<'a> From<&'a mut EntityPrefab> for GameProto<'a, EntityPrefab> {
    fn from(x: &'a mut EntityPrefab) -> Self {
        GameProto(Either::Mut(x))
    }
}

impl<'a> From<&'a EntityPrefab> for PlayerProto<'a, EntityPrefab> {
    fn from(x: &'a EntityPrefab) -> Self {
        PlayerProto(Either::Imut(x))
    }
}

impl<'a> From<&'a mut EntityPrefab> for PlayerProto<'a, EntityPrefab> {
    fn from(x: &'a mut EntityPrefab) -> Self {
        PlayerProto(Either::Mut(x))
    }
}

//

#[derive(Debug)]
/// Enum which abtracts immutable and mutable access to entity objects.
pub enum Either<'a, E: Entity + 'a> {
    /// Stores immutable references to entities.
    Imut(&'a E),
    /// Store mutable references to entities.
    Mut(&'a mut E),
}

#[derive(Debug)]
/// Prototype for game related behaviour.
pub struct GameProto<'a, E: Entity + 'a>(pub Either<'a, E>);
impl<'a, E: Entity + 'a> PrototypeMarker for GameProto<'a, E> {}

#[derive(Debug)]
/// Prototype for player related behaviour.
pub struct PlayerProto<'a, E: Entity + 'a>(pub Either<'a, E>);
impl<'a, E: Entity + 'a> PrototypeMarker for PlayerProto<'a, E> {}

// value_from_type cannot automatically implement [`ProtoEnumerator`]
// for the generated enum.
impl ProtoEnumerator for ProtoItem {}
