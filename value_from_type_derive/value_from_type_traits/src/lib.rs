
/// Returns an enum variant for the generic argument.
/// 
/// This trait is similar to ['From`], without needing to provide an
/// actual (l-)value.
/// 
/// [`From`]: std::convert::From
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
