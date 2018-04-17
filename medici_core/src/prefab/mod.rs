//! Types with ubiquitous usage within state machines (containers) or specific to
//! the opiniated nature of this framework.
//! Framework users are encouraged to use these types whenever possible. The services
//! are built to make use of these types, but freedom was preserved wherever possible.
//!
//! # Note
//! The child (and grandchild) types defined within this module are built to be as generic
//! as possible, but some make use of other types within the [`prefab`] module.
//! Incompatibility may arise when the coupling with these other types is not considered.

use value_from_type_macros::value_from_type;

pub mod card;
pub mod entity;
pub mod prototype;
#[macro_use]
pub mod runtime;
pub mod state;
pub mod timing;
pub mod transaction;
pub mod trigger;
