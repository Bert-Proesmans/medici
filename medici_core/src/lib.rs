// Linters.
#![allow(unused_unsafe)]
#![deny(missing_docs)]
// Unstable features.
#![feature(
    associated_type_defaults, try_from, never_type, proc_macro, proc_macro_mod,
    proc_macro_path_invoc, const_fn
)]
// Clippy linting when building debug versions.
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]
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

// Note: Crates loaded with a public modifier are re-exported so defined macros can
// access them!

extern crate value_from_type_macros;
pub extern crate value_from_type_traits;

pub extern crate failure;
extern crate failure_derive;
#[macro_use]
extern crate maplit;

mod workaround;

#[macro_use]
pub mod prefab;
#[macro_use]
pub mod ctstack;
#[macro_use]
pub mod macros;

pub mod error;
pub mod function;
pub mod marker;
pub mod service;
pub mod stm;
pub mod storage;
pub mod transaction;
