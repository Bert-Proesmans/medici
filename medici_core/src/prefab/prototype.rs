#![value_from_type(ProtoItem)]

use function::Entity;
use marker::{ProtoEnumerator, Prototype, PrototypeMut};

use prefab::entity::Entity as EntityPrefab;

/// Prototype for game behaviour specifically targetting the [`Entity`] defined
/// within medici_core::prefab.
pub type Game<'a> = GameProto<'a, EntityPrefab>;
/// Prototype for game behaviour specifically targetting the [`Entity`] defined
/// within medici_core::prefab.
pub type GameMut<'a> = GameProtoMut<'a, EntityPrefab>;
/// Prototype for player behaviour specifically targetting the [`Entity`] defined
/// within medici_core::prefab.
pub type Player<'a> = PlayerProto<'a, EntityPrefab>;
/// Prototype for game behaviour specifically targetting the [`Entity`] defined
/// within medici_core::prefab.
pub type PlayerMut<'a> = PlayerProtoMut<'a, EntityPrefab>;

#[derive(Debug)]
/// Prototype for game related behaviour.
pub struct GameProto<'a, E: Entity + 'a>(pub &'a E);
impl<'a, E: Entity + 'a> Prototype for GameProto<'a, E> {}

#[derive(Debug)]
/// Prototype for game related behaviour.
pub struct GameProtoMut<'a, E: Entity + 'a>(pub &'a mut E);
impl<'a, E: Entity + 'a> Prototype for GameProtoMut<'a, E> {}
impl<'a, E: Entity + 'a> PrototypeMut for GameProtoMut<'a, E> {}

#[derive(Debug)]
/// Prototype for player related behaviour.
pub struct PlayerProto<'a, E: Entity + 'a>(pub &'a E);
impl<'a, E: Entity + 'a> Prototype for PlayerProto<'a, E> {}

#[derive(Debug)]
/// Prototype for player related behaviour.
pub struct PlayerProtoMut<'a, E: Entity + 'a>(pub &'a mut E);
impl<'a, E: Entity + 'a> Prototype for PlayerProtoMut<'a, E> {}
impl<'a, E: Entity + 'a> PrototypeMut for PlayerProtoMut<'a, E> {}

// value_from_type cannot automatically implement [`ProtoEnumerator`]
// for the generated enum.
impl ProtoEnumerator for ProtoItem {}
