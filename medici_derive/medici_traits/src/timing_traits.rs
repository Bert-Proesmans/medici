use std::fmt::Debug;
pub trait Timing: Debug {}

// Attribute macro must be imported through a use statement
use value_from_type_macros::value_from_type;

// The module is made public by the proc macro
mod default {
    // This macro will build enum EnumerationTiming into the this module.
    #![value_from_type(EnumerationTiming)]

    #[derive(Debug)]
    pub struct Pre();
    impl Timing for Pre {}

    #[derive(Debug)]
    pub struct Peri();
    impl Timing for Peri {}

    #[derive(Debug)]
    pub struct Post();
    impl Timing for Post {}

    // #[derive(Debug, Clone, PartialEq)]
    // pub enum EnumerationTiming {
    //     Pre,
    //     Peri,
    //     Post,
    // }
}

// This module has issues.
// It looks like the import 'default' is NOT the default we want. It seems like the import
// is still bound to the module BEFORE the procedural macro had rewritten the module.
// We actually want to bind to the newly outputted default module (coming from the proc macro)!
#[cfg(test)]
mod tests {
    // We need an absolute import here because the entire default module is rewritten!
    use super::default;
    use value_from_type_traits::FromType;

    #[test]
    fn default_timing_into() {
        let pre_variant: default::EnumerationTiming =
            <default::EnumerationTiming as FromType<default::Pre>>::from_type();
        assert_eq!(pre_variant, default::EnumerationTiming::Pre);
        let var_clone = pre_variant.clone();
        assert_eq!(pre_variant == var_clone, true);
    }
}
