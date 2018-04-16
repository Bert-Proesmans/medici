#![value_from_type(ProtoItem)]
//! Module containing specialized prototypes for our machine.
//!
//! The prototypes MUST be defined within this file because we're using
//! the value_from_type macro to generate ProtoItem.

use medici_core::function::Entity;
use medici_core::marker;
use medici_core::prefab::prototype::Either;

use implementation::entity::Entity as MachineEntity;

/// Prototype for game behaviour specifically targetting the [`Entity`] defined
/// within medici_core::prefab.
pub type Game<'a> = GameProto<'a, MachineEntity>;
/// Prototype for player behaviour specifically targetting the [`Entity`] defined
/// within medici_core::prefab.
pub type Player<'a> = PlayerProto<'a, MachineEntity>;

// TODO; Extend to new proto's (see also below)

#[derive(Debug)]
/// Prototype for game related behaviour.
pub struct GameProto<'a, E: Entity + 'a>(pub Either<'a, E>);
impl<'a, E: Entity + 'a> marker::Prototype for GameProto<'a, E> {}

impl<'a, E: Entity + 'a> From<&'a E> for GameProto<'a, E> {
    fn from(x: &'a E) -> Self {
        GameProto(Either::Imut(x))
    }
}

impl<'a, E: Entity + 'a> From<&'a mut E> for GameProto<'a, E> {
    fn from(x: &'a mut E) -> Self {
        GameProto(Either::Mut(x))
    }
}

#[derive(Debug)]
/// Prototype for player related behaviour.
pub struct PlayerProto<'a, E: Entity + 'a>(pub Either<'a, E>);
impl<'a, E: Entity + 'a> marker::Prototype for PlayerProto<'a, E> {}

impl<'a, E: Entity + 'a> From<&'a E> for PlayerProto<'a, E> {
    fn from(x: &'a E) -> Self {
        PlayerProto(Either::Imut(x))
    }
}

impl<'a, E: Entity + 'a> From<&'a mut E> for PlayerProto<'a, E> {
    fn from(x: &'a mut E) -> Self {
        PlayerProto(Either::Mut(x))
    }
}

// TODO; Extend to new proto's

// value_from_type cannot automatically implement [`ProtoEnumerator`]
// for the generated enum.
impl marker::ProtoEnumerator for ProtoItem {}
