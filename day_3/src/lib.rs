#![feature(try_trait)]
#![feature(custom_attribute)]

mod parser;

use std::error::Error;
use std::collections::{HashMap, HashSet};
use crate::parser::{RectParser, Rectangle, Rule};
use pest::Parser;
use from_pest::FromPest;


pub fn parse_squares<T>(inputs: T) ->
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
            let (w, h) = (r.size.w.v, &r.size.h.v);

            let mut v: Vec<(i32, i32)> = Vec::new();
            for i in x..(x+w) {
                for j in y..(y+h) {
                    v.push((i,j));
                }
            }
            (r.id.value.v, v.into_iter())
        });

    Ok(iter)
}

pub fn safe_claim<T>(inputs: T) -> Result<i32, Box<Error>>
    where T: IntoIterator,
          T::Item: AsRef<str>,
{
    let mut possible_claims: HashSet<i32> = HashSet::new();
    let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::with_capacity(30*30);
    for (id, cells) in parse_squares(inputs)? {
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

pub fn overlapping<T>(inputs: T) -> Result<usize, Box<Error>>
    where T: IntoIterator,
          T::Item: AsRef<str>,
{
    let mut overlapping: usize = 0;
    let mut map: HashMap<(i32, i32), usize> = HashMap::with_capacity(30*30);
    for (_, cells) in parse_squares(inputs)? {
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &[&str]= &[
        "#1 @ 1,3: 4x4",
        "#2 @ 3,1: 4x4",
        "#3 @ 5,5: 2x2",
    ];


    #[test]
    fn test_overlapping() {
        assert_eq!(overlapping(TEST_DATA).unwrap(), 4);
    }

    #[test]
    fn test_safe_claim() {
        assert_eq!(safe_claim(TEST_DATA).unwrap(), 3);
    }
}
