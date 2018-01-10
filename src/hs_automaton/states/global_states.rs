use std::fmt::Debug;

pub trait GlobalState: Debug {}
pub trait Waitable: Debug {}
pub trait Actionable: Triggerable + Debug {}
pub trait Triggerable: Debug {}

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
    T: self::timing::Timing,
    A: Actionable,
{
    pub timing: T,
    pub activity: A,
}

#[derive(Debug, State)]
pub struct Finished();

pub mod timing {
    use std::fmt::Debug;
    pub trait Timing: Debug {}

    #[derive(Debug, Timing)]
    pub struct Pre();
    #[derive(Debug, Timing)]
    pub struct Peri();
    #[derive(Debug, Timing)]
    pub struct Post();

    // TODO Push this into a procedural macro
    #[derive(Debug)]
    pub enum EnumerationTiming {
        Pre,
        Peri,
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
