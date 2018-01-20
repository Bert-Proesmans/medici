use std::convert::From;

use containers::entities::{Entity, EntityPrototype};
use super::GameEntity;

impl<'a> EntityPrototype for GameEntity<'a> {}

impl<'a> GameEntity<'a> {
	fn new(x:&'a Entity) -> Self {
		GameEntity(x)
	}
}

impl<'a> From<&'a Entity> for GameEntity<'a> {
    fn from(x: &'a Entity) -> Self {
    	GameEntity::new(x)
    }
}


// impl Entity for GameEntity {
//     fn reference_card(&self) -> u32;

// 	fn state(&self) -> u32;

// 	fn raw_value(&self, tag: u32) -> Option<u32>;
// 	fn set_raw_value(&mut self, tag: u32, value: u32) -> Option<u32>;
// }


