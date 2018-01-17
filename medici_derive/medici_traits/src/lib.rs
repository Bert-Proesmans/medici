#![feature(proc_macro)]
#![feature(attr_literals)]

#[macro_use]
extern crate value_from_type_macros;
extern crate value_from_type_traits;

pub mod action_traits;
pub mod timing_traits;

pub use value_from_type_traits::FromType;
