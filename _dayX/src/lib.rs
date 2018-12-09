#![feature(test)]

use aoc_base::AoC;
use std::error::Error;

pub struct DayX;

impl DayX {
}

impl AoC<usize, usize> for DayX {
    fn task_a(input: &str) -> Result<usize, Box<Error>> {
        unimplemented!();
    }

    fn task_b(input: &str) -> Result<usize, Box<Error>> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use aoc_base::AoC;
    use super::DayX;

    const TEST_DATA: &str = "";

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
