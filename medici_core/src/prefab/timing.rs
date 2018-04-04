#![value_from_type(TimingItem)]

use marker::{Timing, TimingEnumerator};

/// Type representing a timing relationship. Pre X means before X is executed.
#[derive(Debug, Clone, Copy)]
pub struct Pre();
impl Timing for Pre {}

/// Type representing a timing relationship. Peri X means during/while X is executed.
#[derive(Debug, Clone, Copy)]
pub struct Peri();
impl Timing for Peri {}

/// Type representing a timing relationship. Pre X means after X is executed.
#[derive(Debug, Clone, Copy)]
pub struct Post();
impl Timing for Post {}

// value_from_type builds an enumeration of all structures defined within this module.
// The first parameter of the macro will be the identifier of the generated
// enumeration == [`TimingItem`].
// The enumeration itself will be defined INSIDE this module.
impl TimingEnumerator for TimingItem {}
