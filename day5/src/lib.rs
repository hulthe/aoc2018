#![feature(test)]

use aoc_base::AoC;
use rayon::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::iter::repeat;

pub struct Day5;

impl Day5 {
    fn same_letter(a: &char, b: &char) -> bool {
        for al in a.to_lowercase() {
            if al == *b {
                return true;
            };
        }
        for au in a.to_uppercase() {
            if au == *b {
                return true;
            };
        }
        return false;
    }

    fn keep_char(a: &char, b: &char) -> bool {
        if a != b {
            return !Self::same_letter(a, b);
        }
        return true;
    }

    fn react_polymer<T>(input: T) -> usize
    where
        T: IntoIterator<Item = char>,
    {
        let mut collapsed: Vec<char> = Vec::new();
        for c in input {
            if let Some(cp) = collapsed.last() {
                if !Self::keep_char(cp, &c) {
                    collapsed.pop();
                    continue;
                }
            }
            collapsed.push(c);
        }

        collapsed.len()
    }
}

impl<T> AoC<T, usize, usize> for Day5
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    fn task_a(inputs: T) -> Result<usize, Box<Error>> {
        let input = inputs.into_iter().next().unwrap();
        Ok(Self::react_polymer(input.as_ref().chars()))
    }

    fn task_b(inputs: T) -> Result<usize, Box<Error>> {
        let polymer: Vec<char> = inputs
            .into_iter()
            .next()
            .unwrap()
            .as_ref()
            .chars()
            .collect();
        let polymer_iter = repeat(polymer.clone());
        let mut all_units: HashSet<char> = HashSet::with_capacity(25);
        for c in polymer {
            if let Some(cl) = c.to_lowercase().next() {
                all_units.insert(cl);
            }
        }

        let all: Vec<(char, Vec<char>)> = all_units.into_iter().zip(polymer_iter).collect();
        let shortest = all
            .par_iter()
            .map(|(u, p)| repeat(*u).zip(p))
            .map(|p| {
                p.filter(|(u, c)| !Self::same_letter(c, &u))
                    .map(|(_, c)| *c)
            })
            .map(|p| Self::react_polymer(p))
            .reduce(|| std::usize::MAX, std::cmp::min);
        //.fold(std::usize::MAX, std::cmp::min);
        Ok(shortest)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::Day5;
    use aoc_base::AoC;

    const TEST_DATA_A: &[(&[&str], usize)] = &[
        (&["aA"], 0),
        (&["abBA"], 0),
        (&["abAB"], 4),
        (&["aabAAB"], 6),
        (&["dabAcCaCBAcCcaDA"], 10),
    ];

    #[test]
    fn test_a() {
        for (input, result) in TEST_DATA_A {
            assert_eq!(Day5::task_a(*input).unwrap(), *result);
        }
    }

    const TEST_DATA_B: (&[&str], usize) = (&["dabAcCaCBAcCcaDA"], 4);

    #[test]
    fn test_b() {
        let (input, result) = TEST_DATA_B;
        assert_eq!(Day5::task_b(input).unwrap(), result);
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
