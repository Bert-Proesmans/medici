use std::convert::From;

use medici_traits::prelude::*;
use medici_traits::entities::EntityPrototype;

use automaton::prelude::{Entity, GameTags};
use automaton::prototypes::EnumerationPrototype;

// TODO; Move card structure INTO automaton!
use containers::cards::Card;

impl<'a> From<&'a Entity> for EntityId {
    fn from(e: &'a Entity) -> EntityId {
        e.id.clone()
    }
}

impl Entity {
    pub fn new(e_id: EntityId, card: Card) -> Self {
        Entity {
            id: e_id,
            state: hashmap!{GameTags::EntityId => e_id},
            prototypes: vec![],
            card,
        }
    }

    pub fn get_value(&self, key: &GameTags) -> Option<&u32> {
        self.state.get(key)
    }

    pub fn set_value(&mut self, key: GameTags, value: u32) -> Option<u32> {
        // TODO; Filter here keys which are not allowed to be set?
        self.state.insert(key, value)
    }

    pub fn add_proto<P>(&mut self) -> Result<(), ()>
    where
        P: EntityPrototype,
        P: IntoEnum<EnumerationPrototype>,
    {
        let proto_value: EnumerationPrototype = P::into_enum();
        if !self.prototypes.contains(&proto_value) {
            self.prototypes.push(proto_value);
        }

        Ok(())
    }

    pub fn remove_proto<P>(&mut self) -> Result<(), ()>
    where
        P: EntityPrototype,
        P: IntoEnum<EnumerationPrototype>,
    {
        let proto_value: EnumerationPrototype = P::into_enum();
        // Removes all occurrences of proto_value.
        // This can, of course, be optimized later..
        self.prototypes.retain(|v| v != &proto_value);

        Ok(())
    }

    pub fn as_proto<'a, P>(&'a self) -> Result<P, ()>
    where
        P: EntityPrototype + From<&'a Entity>,
        P: IntoEnum<EnumerationPrototype>,
    {
        let proto_value: EnumerationPrototype = P::into_enum();
        if self.prototypes.contains(&proto_value) {
            Ok(P::from(self))
        } else {
            Err(())
        }
    }
}
