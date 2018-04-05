//! Module containing standard entity structures.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use function::{self, EntityBuilder, EntityId};
use marker::ProtoEnumerator;

use prefab::prototype::ProtoItem;

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
        }
    }
}
