#![feature(test)]
use std::collections::HashSet;
use std::error::Error;
use aoc_base::AoC;

pub struct Day1;

impl Day1 {
    fn parse_freqs<T>(reader: T) -> impl Iterator<Item = i32>
    where T: IntoIterator,
          T::Item: AsRef<str>,
    {
        let iter = reader
            .into_iter()
            .filter_map(|s| s.as_ref().parse::<i32>().ok());
        Box::new(iter)
    }
}

impl<T> AoC<T, i32, i32> for Day1
    where T: IntoIterator,
          T::Item: AsRef<str>,
{
    /// Sum the frequencies
    fn task_a(reader: T) -> Result<i32, Box<Error>> {
        let sum = Self::parse_freqs(reader).sum();

        Ok(sum)
    }

    /// Find the first duplicate frequency
    fn task_b(reader: T) -> Result<i32, Box<Error>> {
        let pattern: Vec<i32> = Self::parse_freqs(reader).collect();
        let mut history: HashSet<i32> = HashSet::with_capacity(pattern.len());

        let mut last = 0;
        loop {
            for num in pattern.iter() {
                if !history.insert(last) {
                    return Ok(last);
                }
                last += num;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate test;
    use std::io;
    use self::test::Bencher;
    use super::*;
    use std::io::BufRead;
    use aoc_base::AoC;

    #[test]
    fn test_a() {
        let cursor = io::Cursor::new(b"+4\n-6\n+33").lines().map(|l| l.unwrap());
        assert_eq!(Day1::task_a(cursor).unwrap(), 31);
    }

    const TEST_DATA_B: &[(&[&str], i32)] = &[
        (&["1", "-1"], 0),
        (&["+3", "+3", "+4", "-2", "-4"], 10),
        (&["-6", "+3", "+8", "+5", "-6"], 5),
        (&["+7", "+7", "-2", "-7", "-4"], 14),
    ];

    #[test]
    fn test_b() {
        for (i, r) in TEST_DATA_B {
            assert_eq!(Day1::task_b(*i).unwrap(), r.clone());
        }
    }

    #[bench]
    fn bench_sum_4(b: &mut Bencher) {
        b.iter(test_b)
    }

    fn bench_find_dup(b: &mut Bencher, steps: i32) {
        let data = format!("+{}\n+1\n-{}\n+1", steps, steps);

        b.iter(|| {
            let iter = io::Cursor::new(&data);
            assert_eq!(Day1::task_b(iter.lines().map(|l| l.unwrap())).unwrap(), steps);
        })

    }

    #[bench]
    fn bench_find_dup_100(b: &mut Bencher) {
        bench_find_dup(b, 100);
    }

    #[bench]
    fn bench_find_dup_1000(b: &mut Bencher) {
        bench_find_dup(b, 1000);
    }

    #[bench]
    fn bench_find_dup_10000(b: &mut Bencher) {
        bench_find_dup(b, 10000);
    }
}
