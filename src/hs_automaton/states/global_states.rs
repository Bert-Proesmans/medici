use action_traits::{Actionable, Triggerable};
use timing_traits::Timing;
use wait_traits::Waitable;

pub trait GlobalState {}

#[derive(Debug)]
pub struct Wait<W>
where
    W: Waitable,
{
    pub activity: W,
}

#[derive(Debug)]
pub struct Action<T, A>
where
    T: Timing,
    A: Actionable,
{
    pub timing: T,
    pub activity: A,
}

#[derive(Debug)] // , State
pub struct Finished();

pub mod timing {
    #[derive(Debug, Timing)]
    pub struct Pre();
    #[derive(Debug, Timing)]
    pub struct Peri();
    #[derive(Debug, Timing)]
    pub struct Post();

    // TODO Push this into a procedural macro
    #[derive(Debug)] // , FromGeneric
    pub enum EnumerationTiming {
        // #[generic("::hs_automaton::states::global_states::timing::Pre")]
        Pre,
        // #[generic("::hs_automaton::states::global_states::timing::Peri")]
        Peri,
        // #[generic("::hs_automaton::states::global_states::timing::Post")]
        Post,
    }

    impl From<Pre> for EnumerationTiming {
        fn from(_x: Pre) -> Self {
            EnumerationTiming::Pre
        }
    }

    impl From<Peri> for EnumerationTiming {
        fn from(_x: Peri) -> Self {
            EnumerationTiming::Peri
        }
    }

    impl From<Post> for EnumerationTiming {
        fn from(_x: Post) -> Self {
            EnumerationTiming::Post
        }
    }

}
