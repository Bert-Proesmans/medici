#![feature(proc_macro)]
#![feature(attr_literals)]

extern crate value_from_type_macros;
extern crate value_from_type_traits;

pub mod automata;

pub mod action;
pub mod timing;
pub mod waiting;

pub mod prelude {
    // Immediate re-export for usability
    pub use value_from_type_traits::IntoEnum;

    pub use action::{Actionable, Triggerable};
    pub use waiting::Waitable;
    pub use timing::Timing;
}
