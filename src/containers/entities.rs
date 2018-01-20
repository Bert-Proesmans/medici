use std::collections::HashMap;
use std::convert::From;

use medici_traits::prelude::FromType;

use hs_automaton::entities::EnumerationPrototype;

pub type EntityId = u32;
pub trait EntityPrototype {}

#[derive(Debug)]
pub struct EntityService {
	// This contains all entities instantiated within a certain game.
	entities: HashMap<EntityId, Entity>,
	zones: u32,
}

#[derive(Debug)]
pub struct EntityData {
	id: EntityId,
	state: HashMap<u32, u32>,
}

impl EntityData {
    pub fn new(entity_id: EntityId) -> Result<Self, ()> {
    	Ok(Self {
    		id: entity_id,
    		state: hashmap!{0 => entity_id}
    	})
    }
}

#[derive(Debug)]
pub struct Entity {
	card: u32,
	data: EntityData,
	prototypes: Vec<EnumerationPrototype>,
}

impl Entity {

	fn entity_data(&self) -> &EntityData {
		&self.data
	}

    fn as_proto<'a, P>(&'a self) -> Result<P, ()> 
    where
    	P: EntityPrototype + From<&'a Entity>,
    	EnumerationPrototype: FromType<P>,
    {
    	let proto_value = <EnumerationPrototype as FromType<P>>::from_type();
    	if self.prototypes.contains(&proto_value) {
    		Ok(P::from(self))
    	} else {
    		Err(())
    	}
    }
}
