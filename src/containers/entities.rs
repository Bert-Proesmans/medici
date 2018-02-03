use std::collections::HashMap;
use std::default::Default;

use medici_traits::entities::{EntityId, GAME_E_ID};
use automaton::prelude::*;

use containers::cards::CardContainer;

#[derive(Debug)]
pub struct EntityService {
    // This contains all entities instantiated within a certain game.
    // This could become a Vec (because we know EntityId is actually a monotone
    // integer)
    entities: HashMap<EntityId, Entity>,
    last_entity_id: EntityId,
    zones: u32, // TODO
}

// DBG
impl Default for EntityService {
    fn default() -> Self {
        EntityService {
            entities: hashmap!{},
            last_entity_id: 0,
            zones: 0,
        }
    }
}

impl EntityService {
    pub fn new() -> Self {
        // Build game entity
        let game_card = CardContainer::game_card();
        let mut game_entity = Entity::new(GAME_E_ID, game_card);
        game_entity.add_proto::<GameProto>().unwrap();

        Self {
            entities: hashmap!{
                GAME_E_ID => game_entity
            },
            last_entity_id: GAME_E_ID,
            zones: 0,
        }
    }

    pub fn new_entity(&mut self, card: &'static Card) -> &mut Entity {
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
        tag: GameTags,
        value: u32,
    ) -> Result<Option<u32>, ()> {
        let e_id = e_id.into();
        self.entities
            .get_mut(&e_id)
            .map(|e| e.set_value(tag, value))
            .ok_or(())
    }
}
