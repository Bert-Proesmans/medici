#![feature(nll, proc_macro)]
#![deny(missing_docs)]

//! # Example crate
//! This crate is used to implement the game rules of a board game.
//! It's idiomatic to build a seperate crate for the game rules implementation
//! because the state machine produced by [`game_system`] contains non-
//! accessible types to guarantee certain invariants are valid throughout the
//! runtime of the machine.
//!
//! # See also
//! [`game_system`] for an example on how to implement a state machine for
//! a board game.

extern crate failure;
pub extern crate game_system;

pub mod action;
pub mod card_set;
pub mod trigger;
