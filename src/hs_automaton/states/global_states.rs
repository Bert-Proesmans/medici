#[derive(Debug)]
pub struct Wait<W> {
    pub activity: W,
}

#[derive(Debug)]
pub struct Action<T, A> {
    pub timing: T,
    pub activity: A,
}

#[derive(Debug)]
pub struct Finished();

#[derive(Debug)]
pub struct Pre();
#[derive(Debug)]
pub struct Peri();
#[derive(Debug)]
pub struct Post();
