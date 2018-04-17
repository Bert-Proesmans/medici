//! Submodule defining all state types of the state machine.

pub mod leaf;
pub mod toplevel;

pub mod prelude {
    //! All state types handily packaged to get started with
    //! the defined state machine.

    /// Re-export of all triggerable leaf state types.
    pub use super::leaf::triggerable::*;
    /// Re-export of some leaf state types.
    pub use super::leaf::*;

    /// Re-export of all toplevel state types.
    pub use super::toplevel::*;
}
