#![feature(nll, proc_macro)]
#![deny(missing_docs)]

//! Crate implementing a basic card game.

extern crate failure;
pub extern crate game_system;

pub mod action;
pub mod card_set;
pub mod trigger;
