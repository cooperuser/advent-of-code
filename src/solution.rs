pub trait Solution<T: std::fmt::Display + Eq> {
    fn meta() -> Meta<T>
    where
        Self: Sized;
    fn new(raw: Vec<String>) -> Self
    where
        Self: Sized;
    fn part_a(&self) -> Option<T>;
    fn part_b(&self) -> Option<T>;
    fn run()
    where
        Self: Sized,
    {
        let meta = Self::meta();
        let sample_a = Self::new(crate::split(meta.sample_a));
        let sample_b = Self::new(crate::split(meta.sample_b));
        let start = std::time::Instant::now();
        let real = Self::new(crate::split(meta.input));
        let duration = start.elapsed();

        println!("parse :\t{:?}\n", duration);
        match sample_a.part_a() {
            Some(received) if received == meta.answer_a => {
                let start = std::time::Instant::now();
                let answer = real.part_a();
                let duration = start.elapsed();
                println!("part_a:\t{}", answer.unwrap());
                println!(" ** in:\t{:?}", duration);
            }
            Some(received) => {
                println!("part_a: failed!");
                println!("\texpected: {}", meta.answer_a);
                println!("\treceived: {}", received);
            }
            None => {
                println!("part_a: unsolved!");
            }
        }

        match sample_b.part_b() {
            Some(received) if received == meta.answer_b => {
                let start = std::time::Instant::now();
                let answer = real.part_b();
                let duration = start.elapsed();
                println!("part_b:\t{}", answer.unwrap());
                println!(" ** in:\t{:?}", duration);
            }
            Some(received) => {
                println!("part_b: failed!");
                println!("\texpected: {}", meta.answer_b);
                println!("\treceived: {}", received);
            }
            None => {
                println!("part_b: unsolved!");
            }
        }
    }
}

#[derive(Clone)]
pub struct Meta<T> {
    pub input: String,
    pub sample_a: String,
    pub sample_b: String,
    pub answer_a: T,
    pub answer_b: T,
}
