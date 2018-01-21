use std::collections::HashMap;
use std::convert::From;

use medici_traits::prelude::IntoEnum;

use containers::cards::{Card, GAME_CARD};

use hs_automaton::entities::{EnumerationPrototype, Game as GameEntity};

pub type EntityId = u32;
pub const GAME_E_ID: EntityId = 0;

pub trait EntityPrototype {}

#[derive(Debug)]
pub struct EntityService {
    // This contains all entities instantiated within a certain game.
    // This could become a Vec (because we know EntityId is actually a monotone
    // integer)
    entities: HashMap<EntityId, Entity>,
    last_entity_id: EntityId,
    zones: u32,
}

impl EntityService {
    pub fn new() -> Self {
        // Build game entity
        let mut game_entity = Entity::new(GAME_E_ID, GAME_CARD);
        game_entity.add_proto::<GameEntity>();

        Self {
            entities: hashmap!{
                GAME_E_ID => game_entity
            },
            last_entity_id: GAME_E_ID,
            zones: 0,
        }
    }

    pub fn new_entity(&mut self, card: Card) -> &mut Entity {
        let next_e_id = self.last_entity_id + 1;
        let new_entity = Entity::new(next_e_id, card);
        self.entities.insert(next_e_id, new_entity);
        self.last_entity_id = next_e_id;
        // Return a reference to the newly created entity.
        self.entities.get_mut(&next_e_id).unwrap()
    }

    pub fn entity<E: Into<EntityId>>(&self, e: E) -> Option<&Entity> {
        self.entities.get(&e.into())
    }

    pub fn entity_mut<E: Into<EntityId>>(&mut self, e: E) -> Option<&mut Entity> {
        self.entities.get_mut(&e.into())
    }

    pub fn update_raw_value<E: Into<EntityId>>(
        &mut self,
        e_id: E,
        tag: u32,
        value: u32,
    ) -> Result<Option<u32>, ()> {
        let e_id = e_id.into();
        self.entities
            .get_mut(&e_id)
            .map(|e| e.data_mut().set_value(tag, value))
            .ok_or(())
    }
}

#[derive(Debug)]
pub struct EntityData {
    id: EntityId,
    state: HashMap<u32, u32>,
}

impl EntityData {
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            id: entity_id,
            state: hashmap!{0 => entity_id},
        }
    }

    pub fn get_value(&self, key: &u32) -> Option<&u32> {
        self.state.get(key)
    }

    pub fn set_value(&mut self, key: u32, value: u32) -> Option<u32> {
        // TODO; Filter here keys which are not allowed to be set?
        self.state.insert(key, value)
    }
}

#[derive(Debug)]
pub struct Entity {
    card: Card,
    data: EntityData,
    prototypes: Vec<EnumerationPrototype>,
}

impl<'a> From<&'a Entity> for EntityId {
    fn from(e: &'a Entity) -> EntityId {
        e.data.id.clone()
    }
}

impl Entity {
    pub fn new(e_id: EntityId, card: Card) -> Self {
        let data = EntityData::new(e_id);
        Self {
            prototypes: vec![],
            card,
            data,
        }
    }

    pub fn data(&self) -> &EntityData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut EntityData {
        &mut self.data
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
