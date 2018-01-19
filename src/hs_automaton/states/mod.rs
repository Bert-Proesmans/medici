pub mod action_states;
pub mod effect_states;
pub mod global_states;
pub mod wait_states;

pub use medici_traits::timing_traits::default::{EnumerationTiming, Peri, Post, Pre};
pub use self::action_states::*;
pub use self::effect_states::*;
pub use self::global_states::*;
pub use self::wait_states::*;

#[cfg(test)]
mod tests {
    use medici_traits::FromType;
    use super::*;

    #[test]
    fn value_from_type() {
        let variant: EnumerationTrigger = <EnumerationTrigger as FromType<EndTurn>>::from_type();
        assert_eq!(variant, EnumerationTrigger::EndTurn);

        let variant: EnumerationTiming = <EnumerationTiming as FromType<Pre>>::from_type();
        assert_eq!(variant, EnumerationTiming::Pre);
    }

    #[test]
    fn it_works() {
        // entry();
    }
}
