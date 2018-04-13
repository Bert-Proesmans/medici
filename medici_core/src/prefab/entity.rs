//! Module containing standard entity structures.

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use value_from_type_traits::IntoEnum;

use function::{self, EntityBuilder, EntityId};
use marker::{ProtoEnumerator, PrototypeMarker};
use service::error::MissingProtoTypeError;

use prefab::prototype::ProtoItem;

/// The game entity should always have ID 0.
pub const GAME_E_ID: EntityId = 0;

/// Entity structure which makes use of structures defined by the
/// medici_core::prefab module.
pub type Entity = EntityStruct<i32, ProtoItem>;

#[derive(Debug, Clone)]
/// Type representing a stateful 'thing' within the state-machine.
///
/// Essentially everything can be classified as an entity because there are
/// always properties that can be attributed to these 'things'.
/// Next to properties, which can be freely manipulated, behaviours can also
/// be defined.
/// Behaviours are essentially recipes of property manipulations. These
/// behaviours are called [`Prototypes`].
pub struct EntityStruct<S, P>
where
    S: Clone + Eq + Hash,
    P: ProtoEnumerator + Clone + Eq + Hash,
{
    id: EntityId,
    /// Contains all properties attributed to this entity.
    pub state: HashMap<S, u32>,
    /// Contains a set of behaviours that are attributed to this entity.
    pub prototypes: HashSet<P>,
    /// Provides the option to assign a string to this entity.
    ///
    /// Example usages include;
    ///     Name of player entities,
    ///     Short identity for debugging purposes.
    pub human_readable: Option<String>,
}

impl<S, P> function::Entity for EntityStruct<S, P>
where
    S: Clone + Eq + Hash,
    P: ProtoEnumerator + Clone + Eq + Hash,
{
    type ID = EntityId;

    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<S, P> EntityBuilder<Self> for EntityStruct<S, P>
where
    S: Clone + Eq + Hash,
    P: ProtoEnumerator + Clone + Eq + Hash,
{
    fn new_with_id(id: EntityId) -> Self {
        Self {
            id,
            state: hashmap!{},
            prototypes: hashset!{},
            human_readable: None,
        }
    }
}

impl<S, P> EntityStruct<S, P>
where
    S: Clone + Eq + Hash,
    P: ProtoEnumerator + Debug + Clone + Eq + Hash,
{
    /// Retrieves the value of the requested property defined within this entity.
    /// 0 is returned as default value when the property key was not found!
    pub fn get_value_default(&self, key: &S) -> u32 {
        self.state.get(key).cloned().or(Some(0)).unwrap()
    }

    /// Retrieves the value of the requested property defined within this entity.
    pub fn get_value(&self, key: &S) -> Option<u32> {
        self.state.get(key).cloned()
    }

    /// Store the provided property key with corresponding value into this entity.
    ///
    /// The old value is returned if the key was already known within this entity.
    pub fn set_value(&mut self, key: S, value: u32) -> Option<u32> {
        self.state.insert(key, value)
    }

    /// Attach new behaviour to this specific entity.
    pub fn add_proto<PT>(&mut self)
    where
        PT: PrototypeMarker + IntoEnum<P>,
    {
        let proto_entry: P = PT::into_enum();
        self.prototypes.insert(proto_entry);
    }

    /// Removes behaviour from this specific entity.
    pub fn remove_proto<PT>(&mut self)
    where
        PT: PrototypeMarker + IntoEnum<P>,
    {
        let proto_entry: P = PT::into_enum();
        self.prototypes.remove(&proto_entry);
    }

    /// Return this entity as the requested prototype.
    pub fn as_proto<'a, PT>(&'a self) -> Result<PT, MissingProtoTypeError<EntityId, P>>
    where
        PT: PrototypeMarker + IntoEnum<P> + From<&'a Self>,
    {
        let proto_entry: P = PT::into_enum();
        if self.prototypes.contains(&proto_entry) {
            Ok(PT::from(self))
        } else {
            Err(MissingProtoTypeError(self.id, proto_entry))
        }
    }

    /// Return this entity as the requested prototype.
    pub fn as_proto_mut<'a, PT>(&'a mut self) -> Result<PT, MissingProtoTypeError<EntityId, P>>
    where
        PT: PrototypeMarker + IntoEnum<P> + From<&'a mut Self>,
    {
        let proto_entry: P = PT::into_enum();
        if self.prototypes.contains(&proto_entry) {
            Ok(PT::from(self))
        } else {
            Err(MissingProtoTypeError(self.id, proto_entry))
        }
    }
}
