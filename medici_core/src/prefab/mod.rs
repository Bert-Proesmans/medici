//! Types with ubiquitous usage in general state machine building or specific to
//! the opiniated nature of this framework.
//!
//! Framework users are encouraged to use these types whenever possible. The services
//! are built to make use of these types, but freedom was preserved wherever possible.

use value_from_type_macros::value_from_type;

/// Module containing ready-to-use types useful for encoding state machines.
pub mod timing;
