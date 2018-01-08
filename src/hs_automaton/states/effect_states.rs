#[derive(Debug)]
pub struct Effect<T, E>(pub T, pub E);
#[derive(Debug)]
pub struct Trigger<T, U>(pub T, pub U);
#[derive(Debug)]
pub struct Death<T, D>(pub T, pub D);
