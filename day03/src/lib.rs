#![feature(test)]

mod parser;

use crate::parser::{RectParser, Rectangle, Rule};
use aoc_base::AoC;
use from_pest::FromPest;
use pest::Parser;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::iter::repeat;

pub struct Day3;

impl Day3 {
    fn parse_squares<'a>(
        inputs: &'a str,
    ) -> Result<impl Iterator<Item = (i32, impl Iterator<Item = (i32, i32)>)> + 'a, Box<Error>>
    {
        let iter = inputs
            .lines()
            .map(|s| s.to_owned())
            .map(|s| {
                let mut p = RectParser::parse(Rule::rect, &s).unwrap();
                Rectangle::from_pest(&mut p).unwrap()
            })
            .map(|r| {
                let (x, y) = (r.coord.x.v, r.coord.y.v);
                let (w, h) = (r.size.w.v, r.size.h.v);
                let v: Vec<_> = (x..(x + w))
                    .flat_map(|i| repeat(i).zip(y..(y + h)))
                    .collect();

                (r.id.value.v, v.into_iter())
            });

        Ok(iter)
    }
}

impl AoC<usize, i32> for Day3 {
    /// Get number of overlapping cells
    fn task_a(inputs: &str) -> Result<usize, Box<Error>> {
        let mut overlapping: usize = 0;
        let mut map: HashMap<(i32, i32), usize> = HashMap::with_capacity(30 * 30);
        for (_, cells) in Self::parse_squares(inputs)? {
            for (i, j) in cells {
                let val = map.entry((i, j)).or_insert(0);
                *val += 1;
                if *val == 2 {
                    overlapping += 1;
                }
            }
        }

        Ok(overlapping)
    }

    /// Find the one box which doesn't overlap
    fn task_b(inputs: &str) -> Result<i32, Box<Error>> {
        let mut possible_claims: HashSet<i32> = HashSet::new();
        let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::with_capacity(30 * 30);
        for (id, cells) in Self::parse_squares(inputs)? {
            let mut possible = true;
            for (i, j) in cells {
                let existing_claims = map.entry((i, j)).or_insert(vec![]);
                existing_claims.push(id);
                if existing_claims.len() > 1 {
                    possible_claims.remove(&existing_claims[0]);
                    possible = false;
                }
            }

            if possible {
                possible_claims.insert(id);
            }
        }

        Ok(possible_claims
            .iter()
            .next()
            .map(|id| id.clone())
            .ok_or("No safe claim found")?)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::*;
    use aoc_base::AoC;

    const TEST_DATA: &str = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";

    #[test]
    fn test_a() {
        assert_eq!(Day3::task_a(TEST_DATA).unwrap(), 4);
    }

    #[test]
    fn test_b() {
        assert_eq!(Day3::task_b(TEST_DATA).unwrap(), 3);
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
