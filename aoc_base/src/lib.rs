use std::error::Error;
use std::fmt::Display;

pub trait AoC<RA, RB>
    where RA: Display,
          RB: Display,
{
    fn task_a(input: &str) -> Result<RA, Box<Error>>;
    fn task_b(input: &str) -> Result<RB, Box<Error>>;
}

