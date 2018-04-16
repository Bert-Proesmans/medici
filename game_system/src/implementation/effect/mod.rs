//! Module containing all code relating to powers in the game.

// Note: Regarding
// ```rust
// pub fn start_game_trigger<CTS>(
//    mut x: Machine<Trigger<Peri, Start>, CTS>,
// ) -> Result<Machine<Trigger<Peri, Start>, CTS>, Error>
// where
//    CTS: CTStack + 'static,
//	```
//
//	#![feature(default_type_parameter_fallback)] could solve the hassle of having
//	to prepare the method like so
//	```rust
//	machine
//        .triggers
//        .add_trigger(start_game_trigger::<AnyStack>)
//        .unwrap();
//	```
//
//	The first rust code could get the default for CTS, which is `AnyStack`
//	in the case of both examples.

pub mod action;
pub mod trigger;
