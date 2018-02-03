// FIXME; Issue with properly resolving to the default Entity!
// mod prototypes;

use std::fmt::Debug;

pub type EntityId = u32;
pub const GAME_E_ID: EntityId = 0;
pub const E_ID_KEY: isize = 0;

pub trait EntityPrototype: Debug {}

pub mod default {
	use std::collections::HashMap;
	use super::EntityId;

	#[derive(Debug)]
	pub struct Entity {
		// Entity Data
		id: EntityId,
		state: HashMap<u32, u32>,
	}
}


