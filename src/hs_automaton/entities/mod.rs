pub mod game;

use value_from_type_macros::value_from_type;

use containers::entities::Entity;

pub use self::custom::{EnumerationPrototype, GameEntity};

mod custom {
    #![value_from_type(EnumerationPrototype)]

    #[derive(Debug)]
	pub struct GameEntity<'a>(pub(crate)&'a Entity);
}
