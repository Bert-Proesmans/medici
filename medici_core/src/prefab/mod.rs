//! Types with ubiquitous usage in general state machine building or specific to
//! the opiniated nature of this framework.
//!
//! Framework users are encouraged to use these types whenever possible. The services
//! are built to make use of these types, but freedom was preserved wherever possible.

use value_from_type_macros::value_from_type;

pub mod card;
pub mod entity;
pub mod prototype;
pub mod state;
pub mod timing;
pub mod transaction;
pub mod trigger;
