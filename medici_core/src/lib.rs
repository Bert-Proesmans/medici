// Linters.
#![allow(unknown_lints, dead_code, unused_mut, unused_variables, let_and_return, useless_format,
         unused_unsafe)]
#![deny(missing_docs)]
// Unstable features.
#![feature(associated_type_defaults, try_from, never_type, proc_macro)]
// Clippy linting when building debug versions.
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
// Linters for code residing in documentation.
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

//! Types for implementing a state machine for (board) games.
//! This crate provides an opinionated framework which the developers
//! can use for their own games.

// Note: All macros are NOT globally available anymore. They have to be imported
// just like any other defined type!
//
// eg:
// `use value_from_type_macros::value_from_type;`

extern crate value_from_type_macros;
extern crate value_from_type_traits;

extern crate failure;
#[macro_use] extern crate maplit;

pub mod error;
pub mod function;
pub mod marker;
pub mod prefab;
pub mod service;
pub mod stm;
pub mod transaction;
