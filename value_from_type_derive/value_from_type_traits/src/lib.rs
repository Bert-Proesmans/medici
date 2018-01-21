pub trait FromType<T> {
    fn from_type() -> Self;
}

// Reflexive implementation on FromType
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
