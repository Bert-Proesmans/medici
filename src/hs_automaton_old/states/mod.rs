pub mod action_states;
pub mod effect_states;
pub mod global_states;
pub mod wait_states;

pub use self::action_states::*;
pub use self::effect_states::*;
pub use self::global_states::*;
pub use self::wait_states::*;

// We do not declare the timing structs ourselves.
// Instead we import the defaults provided to us!
pub use medici_traits::timing::default::{EnumerationTiming, Peri, Post, Pre};

#[cfg(test)]
mod tests {
    use medici_traits::prelude::IntoEnum;
    use super::*;

    #[test]
    fn value_from_type() {
        let variant: EnumerationTrigger = EndTurn::into_enum();
        assert_eq!(variant, EnumerationTrigger::EndTurn);

        let variant: EnumerationTiming = Pre::into_enum();
        assert_eq!(variant, EnumerationTiming::Pre);
    }
}
