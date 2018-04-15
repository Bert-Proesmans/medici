#![value_from_type(TimingItem)]
//! Module containing ready-to-use types which can be used to construct
//! trigger constraints.

use function::State;
use marker;

use prefab::transaction::Epsilon;

/// Type representing a timing relationship. Pre X means before X is executed.
#[derive(Debug, Clone, Copy)]
pub struct Pre();
impl marker::Timing for Pre {}
impl State for Pre {
    type Transaction = Epsilon;
}

/// Type representing a timing relationship. Peri X means during/while X is executed.
#[derive(Debug, Clone, Copy)]
pub struct Peri();
impl marker::Timing for Peri {}
impl State for Peri {
    type Transaction = Epsilon;
}

/// Type representing a timing relationship. Pre X means after X is executed.
#[derive(Debug, Clone, Copy)]
pub struct Post();
impl marker::Timing for Post {}
impl State for Post {
    type Transaction = Epsilon;
}

// value_from_type builds an enumeration of all structures defined within this module.
// The first parameter of the macro will be the identifier of the generated
// enumeration == [`TimingItem`].
// The enumeration itself will be defined INSIDE this module.
impl marker::TimingEnumerator for TimingItem {}
