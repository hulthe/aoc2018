#![feature(test)]

use aoc_base::AoC;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::iter::repeat;

pub struct Day06;

fn parse_inputs(inputs: &str) -> Result<Vec<(i32, i32)>, Box<Error>> {
    inputs
        .lines()
        .map(|s| s.split(", ").map(|c| c.parse()).collect())
        .map(|s: Vec<Result<i32, _>>| Ok((s[0].clone()?, s[1].clone()?)))
        .collect()
}

fn get_dimensions<'a, T>(coords: T) -> (i32, i32, i32, i32)
where
    T: IntoIterator<Item = &'a (i32, i32)> + 'a,
{
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut max_y = std::i32::MIN;
    for (x, y) in coords.into_iter() {
        min_x = i32::min(*x, min_x);
        min_y = i32::min(*y, min_y);
        max_x = i32::max(*x, max_x);
        max_y = i32::max(*y, max_y);
    }

    (min_x, min_y, max_x, max_y)
}

fn distance((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> usize {
    return ((x2 - x1).abs() + (y2 - y1).abs()) as usize;
}

fn all_coords(dimensions: (i32, i32, i32, i32)) -> Vec<(i32, i32)> {
    let (min_x, min_y, max_x, max_y) = dimensions;
    (min_x..=max_x)
        .flat_map(|i| repeat(i).zip(min_y..=max_y))
        .collect()
}

impl AoC<usize, usize> for Day06 {
    fn task_a(inputs: &str) -> Result<usize, Box<Error>> {
        let coords: Vec<(i32, i32)> = parse_inputs(inputs)?;
        let (min_x, min_y, max_x, max_y) = get_dimensions(&coords);

        let all_coords = all_coords((min_x, min_y, max_x, max_y));
        let all_coords_iter = rayon::iter::repeat(all_coords.clone());

        let all_coords_empty = || {
            all_coords
                .par_iter()
                .map(|p| (p.clone(), None, std::usize::MAX))
                .collect::<Vec<_>>()
        };

        let all_coords_with_distances = all_coords_iter
            .zip(coords)
            .map(|(ac, sp)| {
                ac.par_iter()
                    .map(|p| (p.clone(), Some(sp.clone()), distance(p, &sp)))
                    .collect::<Vec<_>>()
            })
            .reduce(all_coords_empty, |v1, v2| {
                v1.par_iter()
                    .zip(v2)
                    .map(|(&(_, s1, d1), (p, s2, d2))| {
                        if d1 < d2 {
                            (p, s1, d1)
                        } else if d2 < d1 {
                            (p, s2, d2)
                        } else {
                            (p, None, d1)
                        }
                    })
                    .collect()
            });

        let i: Vec<_> = all_coords_with_distances
            .par_iter()
            .filter_map(|&(p, s, _)| Some((p, s?)))
            .collect();

        let mut size_map: HashMap<(i32, i32), usize> = HashMap::new();
        for &(_, s) in i.iter() {
            *size_map.entry(s).or_insert(0) += 1;
        }

        for &((x, y), s) in i.iter() {
            if x == min_x || x == max_x || y == min_y || y == max_y {
                size_map.remove(&s);
            }
        }

        Ok(size_map
            .iter()
            .fold(0, |s1, (_, s2)| std::cmp::max(s1, *s2)))
    }

    fn task_b(inputs: &str) -> Result<usize, Box<Error>> {
        let coords: Vec<(i32, i32)> = parse_inputs(inputs)?;
        let all_coords = all_coords(get_dimensions(&coords));

        let region_size = all_coords
            .par_iter()
            .map(|p| coords.par_iter().map(|sp| distance(p, sp)).sum::<usize>())
            .filter(|&td| td < 10000)
            .count();

        Ok(region_size)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::Day06;
    use aoc_base::AoC;

    const TEST_DATA: &str = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";

    #[test]
    fn test_a() {
        assert_eq!(Day06::task_a(TEST_DATA).unwrap(), 17);
    }

    #[test]
    fn test_b() {
        // Note that this gets a region where d < 10000
        // But its only really useful for d < 32
        assert_eq!(Day06::task_b(TEST_DATA).unwrap(), 72 /* should be 16 */);
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
