use std::fmt::Debug;
pub trait Timing: Debug {}

// Attribute macro must be imported through a use statement
use value_from_type_macros::value_from_type;

pub mod default {
	// This macro will build enum EnumerationTiming into the this module.
    #![value_from_type(EnumerationTiming)]

    // #[derive(Debug, Timing)]
    pub struct Pre();
    // #[derive(Debug, Timing)]
    pub struct Peri();
    // #[derive(Debug, Timing)]
    pub struct Post();

	// #[derive(Debug, Clone, PartialEq)]
	// pub enum EnumerationTiming {
	//     Pre,
	//     Peri,
	//     Post,
	// }
}


