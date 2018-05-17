//! Module which implements an enum to handle the difference between
//! utilizing a mutable and immutable reference to something.

use std::convert::{AsMut, AsRef};

/// Compile time boolean.
///
/// Types implementing this trait are used to enforce certain behaviour.
///
/// # See also
/// [`MutSwitch`]
pub trait CTBool {
    /// Runtime value for
    const VALUE: bool;
}

/// Type used to enforce a True value during compilation.
pub struct CTTrue;
impl CTBool for CTTrue {
    const VALUE: bool = true;
}

/// Type used to enforce a False value during compilation.
// Equivalent to never-type (!), but the [`Lock`] concept cannot be applied
// when ! is used.
pub type CTFalse = !;
impl CTBool for CTFalse {
    const VALUE: bool = false;
}

#[derive(Debug)]
/// Enumeration for simplifying access to a certain object.
///
/// This enumeration can hold a mutable or immutable reference to an object.
/// The intention is to have this enumeration abstract over mutable or immutable
/// access so code doesn't need to be duplicated to create a mutable container
/// for each immutable container.
pub enum MutSwitch<'a, T: 'a, AllowMut>
where
    AllowMut: CTBool,
{
    /// An immutable reference is stored.
    VarImut(&'a T),
    /// A mutable reference is stored.
    ///
    /// AllowMut is used to prevent this variant from being constructed when it's a
    /// "void type". This results in an sound logic where a mutable reference can never
    /// be created from an immutable reference + acts as compiler optimization hint.
    VarMut(&'a mut T, AllowMut),
}

// Implement immutable functionality.
impl<'a, T: 'a> MutSwitch<'a, T, CTFalse> {
    /// Constructs this enum with an immutable reference.
    pub fn from_ref(t: &'a T) -> Self {
        MutSwitch::VarImut(t)
    }

    /// Returns the stored reference.
    pub fn get(&self) -> &T {
        match *self {
            MutSwitch::VarImut(ref t) => t,
            _ => unreachable!(),
        }
    }
}

// Implement mutable functionality.
impl<'a, T: 'a> MutSwitch<'a, T, CTTrue> {
    /// Constructs this enum with a mutable reference.
    pub fn from_mut(t: &'a mut T) -> Self {
        MutSwitch::VarMut(t, CTTrue)
    }

    /// Returns the mutable reference stored within.
    pub fn get_mut(&mut self) -> &mut T {
        match *self {
            MutSwitch::VarMut(ref mut v, _) => v,
            _ => unreachable!(),
        }
    }
}

impl<'a, T: 'a> AsRef<T> for MutSwitch<'a, T, CTFalse> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<'a, T: 'a> AsMut<T> for MutSwitch<'a, T, CTTrue> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<'a, T: 'a> From<&'a T> for MutSwitch<'a, T, CTFalse> {
    fn from(t: &'a T) -> Self {
        MutSwitch::from_ref(t)
    }
}

impl<'a, T: 'a> From<&'a mut T> for MutSwitch<'a, T, CTTrue> {
    fn from(t: &'a mut T) -> Self {
        MutSwitch::from_mut(t)
    }
}
