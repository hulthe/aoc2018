#![feature(test)]

mod parser;

use std::error::Error;
use std::collections::{HashMap, HashSet};
use std::iter::repeat;
use crate::parser::{RectParser, Rectangle, Rule};
use pest::Parser;
use from_pest::FromPest;
use aoc_base::AoC;

pub struct Day3;

impl Day3 {
    fn parse_squares<T>(inputs: T) ->
        Result<impl Iterator<Item=(i32, impl Iterator<Item=(i32, i32)>)>, Box<Error>>
        where T: IntoIterator,
              T::Item: AsRef<str>,
    {
        let iter = inputs.into_iter()
            .map(|s| s.as_ref().to_owned())
            .map(|s| {
                let mut p = RectParser::parse(Rule::rect, &s).unwrap();
                Rectangle::from_pest(&mut p).unwrap()
            })
            .map(|r| {
                let (x, y) = (r.coord.x.v, r.coord.y.v);
                let (w, h) = (r.size.w.v, r.size.h.v);
                let v: Vec<_> = (x..(x+w))
                    .flat_map(|i| repeat(i).zip(y..(y+h)))
                    .collect();

                (r.id.value.v, v.into_iter())
            });

        Ok(iter)
    }
}

impl<T> AoC<T, usize, i32> for Day3
    where T: IntoIterator,
          T::Item: AsRef<str>,
{
    /// Get number of overlapping cells
    fn task_a(inputs: T) -> Result<usize, Box<Error>> {
        let mut overlapping: usize = 0;
        let mut map: HashMap<(i32, i32), usize> = HashMap::with_capacity(30*30);
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
    fn task_b(inputs: T) -> Result<i32, Box<Error>> {
        let mut possible_claims: HashSet<i32> = HashSet::new();
        let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::with_capacity(30*30);
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

        Ok(possible_claims.iter()
           .next()
           .map(|id| id.clone())
           .ok_or("No safe claim found")?)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use aoc_base::AoC;
    use super::*;

    const TEST_DATA: &[&str]= &[
        "#1 @ 1,3: 4x4",
        "#2 @ 3,1: 4x4",
        "#3 @ 5,5: 2x2",
    ];


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
