#![feature(test)]

use aoc_base::AoC;
use std::error::Error;

pub struct DayX;

impl DayX {
}

impl<T> AoC<T, usize, usize> for DayX
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    fn task_a(inputs: T) -> Result<usize, Box<Error>> {
        Ok(std::usize::MAX) // TODO
    }

    fn task_b(inputs: T) -> Result<usize, Box<Error>> {
        Ok(std::usize::MAX) // TODO
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::DayX;
    use aoc_base::AoC;

    const TEST_DATA: &[&str] = &[
    ];

    #[test]
    fn test_a() {
        //assert_eq!(DayX::task_a(TEST_DATA).unwrap(), FIXME);
    }

    #[test]
    fn test_b() {
        //assert_eq!(DayX::task_b(TEST_DATA).unwrap(), FIXME);
    }

    #[bench]
    fn bench_a(b: &mut Bencher) {
        b.iter(test_a)
    }

    #[bench]
    fn bench_b(b: &mut Bencher) {
        b.iter(test_b)
    }
}
