#![feature(proc_macro)]
#![feature(attr_literals)]

extern crate value_from_type_macros;
extern crate value_from_type_traits;

// Immediate re-export for usability
// TODO: Move this into Prelude module
pub use value_from_type_traits::FromType;

pub mod action_traits;
pub mod timing_traits;
pub mod wait_traits;
