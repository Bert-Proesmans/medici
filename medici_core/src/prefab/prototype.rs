#![value_from_type(ProtoItem)]
//! Module containing standard prototypes.
//!
//! The defined prototypes can be implemented in derived crates.

use error::custom_type::InvalidEntityMutUnwrap;
use function::Entity;
use marker;

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

// Note; No lifetime bounds necessary for the containing methods.
// The listed generic constraints are enough information for the compiler
// to infer correct lifetimes.
impl<'a, E> Either<'a, E>
where
    E: Entity + 'a,
{
    /// Returns an immutable reference to the contained [`Entity`].
    pub fn unwrap(&self) -> &E {
        match self {
            Either::Imut(e) => e,
            Either::Mut(e) => e,
        }
    }

    /// Returns a mutable reference to the contained [`Entity`].
    pub fn unwrap_mut(&mut self) -> Result<&mut E, InvalidEntityMutUnwrap> {
        match self {
            Either::Mut(e) => Ok(e),
            _ => Err(InvalidEntityMutUnwrap),
        }
    }
}

#[derive(Debug)]
/// Prototype for game related behaviour.
pub struct GameProto<'a, E: Entity + 'a>(pub Either<'a, E>);
impl<'a, E: Entity + 'a> marker::Prototype for GameProto<'a, E> {}

#[derive(Debug)]
/// Prototype for player related behaviour.
pub struct PlayerProto<'a, E: Entity + 'a>(pub Either<'a, E>);
impl<'a, E: Entity + 'a> marker::Prototype for PlayerProto<'a, E> {}

// value_from_type cannot automatically implement [`ProtoEnumerator`]
// for the generated enum.
impl marker::ProtoEnumerator for ProtoItem {}
