//! Traits used in conjunction with the [`value_from_type_macros`] crate.

#![doc(html_root_url = "https://docs.rs/value_from_type_traits")]

/// Returns an enum variant for the generic argument.
/// 
/// This trait is similar to [From] without needing to provide an
/// actual (l-)value.   
/// As with the [From] trait, developers are encouraged to implement
/// this trait for their types or equivalently use the procedural macro attribute
/// [`value_from_type_macros::value_from_type`] found within the [value_from_type_macros] crate.
/// 
/// 
/// # Examples
/// 
/// ``` 
/// # use value_from_type_traits::{FromType, IntoEnum};
/// #[derive(Debug)]
/// struct X;
/// #[derive(Debug, PartialEq)]
/// enum StructVariants { X_V };
/// 
/// impl FromType<X> for StructVariants {
/// 	fn from_type() -> Self {
/// 		StructVariants::X_V
/// 	}
/// }
/// 
/// assert_eq!(StructVariants::X_V, <StructVariants as FromType<X>>::from_type());
/// assert_eq!(StructVariants::X_V, X::into_enum()); 
/// ```
/// 
/// 
/// [From]: std::convert::From
/// [value_from_type_macros]: value_from_type_macros
pub trait FromType<T> {
    fn from_type() -> Self;
}

/// Reflexive implementation on [`FromType`].
/// 
/// Developers are encouraged to implement [`FromType`], because [`IntoEnum`]
/// is automatically derived from that definition.
pub trait IntoEnum<E> {
    fn into_enum() -> E;
}

impl<T, E> IntoEnum<E> for T
where
    E: FromType<T>,
{
    fn into_enum() -> E {
        <E as FromType<T>>::from_type()
    }
}
