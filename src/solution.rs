pub trait Output {}
impl Output for i64 {}
impl Output for String {}
impl Output for Box<dyn Output + '_> {}

pub trait Solution<T: Output> {
    fn meta() -> Meta<T>
    where
        Self: Sized;
    fn new(raw: Vec<String>) -> Self
    where
        Self: Sized;
    fn part_a(&self) -> Option<T>;
    fn part_b(&self) -> Option<T>;
}

#[derive(Clone)]
pub struct Meta<T> {
    pub input: String,
    pub sample_a: String,
    pub sample_b: String,
    pub answer_a: T,
    pub answer_b: T,
}
