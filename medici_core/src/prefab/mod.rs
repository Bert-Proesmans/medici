//! Types with ubiquitous usage in general state machine building or specific to
//! the opiniated nature of this framework.
//!
//! Framework users are encouraged to use these types whenever possible. The services
//! are built to make use of these types, but freedom was preserved wherever possible.

use value_from_type_macros::value_from_type;

/// Module containing ready-to-use types which can be used to construct
/// trigger constraints.
pub mod timing;

/// Module containing ready-to-use types which can be used to construct
/// trigger constraints.
pub mod trigger;

/// Module containing standard prototypes.
///
/// The defined prototypes can be implemented in derived crates.
pub mod prototype;

pub mod card;
pub mod entity;
