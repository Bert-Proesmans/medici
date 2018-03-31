// Linters.
#![allow(dead_code, unused_mut, unused_variables, let_and_return, useless_format)]
#![deny(missing_docs)]
// Unstable features.
#![feature(associated_type_defaults, try_from, never_type)]
// Clippy linting when building debug versions.
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
// Linters for code residing in documentation.
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

//! Types for implementing a state machine for (board) games.
//! This crate provides an opinionated framework which the developers
//! can use for their own games.

#[macro_use]
extern crate failure;
extern crate value_from_type_traits;

pub mod error;
pub mod function;
pub mod marker;
pub mod service;
pub mod stm;
pub mod transaction;
