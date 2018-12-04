use std::error::Error;
use std::fmt::Display;

pub trait AoC<T, RA, RB>
    where T: IntoIterator,
          T::Item: AsRef<str>,
          RA: Display,
          RB: Display,
{
    fn task_a(inputs: T) -> Result<RA, Box<Error>>;
    fn task_b(inputs: T) -> Result<RB, Box<Error>>;
}

