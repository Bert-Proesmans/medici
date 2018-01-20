use std::fmt::Debug;

pub trait Triggerable: Debug {}
pub trait Actionable: Triggerable {}
