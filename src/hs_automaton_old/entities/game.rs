use std::convert::From;

use containers::entities::{Entity, EntityPrototype};
use super::Game;

impl<'a> EntityPrototype for Game<'a> {}

impl<'a> Game<'a> {
    fn new(x: &'a Entity) -> Self {
        Game(x)
    }
}

impl<'a> From<&'a Entity> for Game<'a> {
    fn from(x: &'a Entity) -> Self {
        Game::new(x)
    }
}

// impl Entity for GameEntity {
//     fn reference_card(&self) -> u32;

// 	fn state(&self) -> u32;

// 	fn raw_value(&self, tag: u32) -> Option<u32>;
// 	fn set_raw_value(&mut self, tag: u32, value: u32) -> Option<u32>;
// }
