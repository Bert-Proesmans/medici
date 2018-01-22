use medici_traits::prelude::*;

#[derive(Debug)]
pub struct Effect<T: Timing, E: Triggerable>(pub T, pub E);
#[derive(Debug)]
pub struct Trigger<T: Timing, U: Triggerable>(pub T, pub U);
#[derive(Debug)]
pub struct Death<T: Timing, D: Actionable>(pub T, pub D);
