use medici_traits::action_traits::{Triggerable, Actionable};
use medici_traits::timing_traits::Timing;
use medici_traits::wait_traits::Waitable;

#[derive(Debug)]
pub struct Effect<T: Timing, E: Triggerable>(pub T, pub E);
#[derive(Debug)]
pub struct Trigger<T: Timing, U: Triggerable>(pub T, pub U);
#[derive(Debug)]
pub struct Death<T: Timing, D: Actionable>(pub T, pub D);
