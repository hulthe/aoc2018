#![feature(test)]

use aoc_base::AoC;
use std::error::Error;
use rayon::iter::repeat;
use std::collections::HashMap;
use rayon::prelude::*;
use std::fmt::{Display, self};
use std::ops::Range;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct Pos(i32, i32);

impl Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub struct Day11;

fn grid(w: i32, h: i32) -> impl ParallelIterator<Item=Pos> + 'static {
    repeat(h)
        .zip(1..(w+1))
        .flat_map(|(h, x)| repeat(x).zip(1..(h+1)))
        .map(|(x, y)| Pos(x, y))
}

impl Day11 {
    pub fn power_level(cell: Pos, serial_number: i32) -> i32 {
        let rack_id = cell.0 + 10;
        let mut power_level = rack_id * cell.1;
        power_level += serial_number;
        power_level *= rack_id;
        power_level /= 100;
        power_level %= 10;
        power_level - 5
    }

    fn gen_sum_grid(serial_number: i32) -> HashMap<Pos, i32> {
        let mut cells: HashMap<Pos, i32> = HashMap::with_capacity(300*300);
        for x in (1..=300).rev() {
            for y in (1..=300).rev() {
                *cells.entry(Pos(x,y)).or_insert(Self::power_level(Pos(x,y), serial_number)) +=
                    *cells.get(&Pos(x+1, y  )).unwrap_or(&0) +
                    *cells.get(&Pos(x  , y+1)).unwrap_or(&0) -
                    *cells.get(&Pos(x+1, y+1)).unwrap_or(&0);
            }
        }
        cells
    }

    fn get_squares<'a>(size: i32, cells: &'a HashMap<Pos, i32>) -> impl ParallelIterator<Item=(Pos, i32, i32)> + 'a
    {
        grid(300 - size, 300 - size)
            .map(move |Pos(x, y)| {
                (Pos(x, y), size,
                    cells.get(&Pos(x, y)).unwrap()
                        -cells.get(&Pos(x + size, y       )).unwrap()
                        -cells.get(&Pos(x       , y + size)).unwrap()
                        +cells.get(&Pos(x + size, y + size)).unwrap()
                )
            })
    }

    pub fn largest_power_level_3x3(serial_number: i32) -> (Pos, i32) {
        let cells = Self::gen_sum_grid(serial_number);

        Self::get_squares(3, &cells)
            .map(|(p, _, v)| (p, v))
            //.fold(((0, 0), std::i32::MIN), |(p1, v1), (p2, v2)| {
            .reduce(|| (Pos(0, 0), std::i32::MIN), |(p1, v1), (p2, v2)| {
                if v1 > v2 {(p1, v1)} else {(p2, v2)}
            })
    }

    pub fn largest_power_level(serial_number: i32, size_range: Range<i32>) -> (Pos, i32, i32) {
        let cells = Self::gen_sum_grid(serial_number);

        size_range.into_par_iter()
            .flat_map(|s| Self::get_squares(s, &cells))
            .reduce(|| (Pos(0, 0), 0, std::i32::MIN), |(p1, s1, v1), (p2, s2, v2)| {
                if v1 > v2 {(p1, s1, v1)} else {(p2, s2, v2)}
            })
    }
}

impl AoC<Pos, String> for Day11 {
    fn task_a(input: &str) -> Result<Pos, Box<Error>> {
        Ok(Self::largest_power_level(input.trim().parse()?, 3..4).0)
    }

    fn task_b(input: &str) -> Result<String, Box<Error>> {
        let (p, s, _) = Self::largest_power_level(input.trim().parse()?, 1..301);
        Ok(format!("{}@{}²", p, s))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::*;


    #[test]
    fn test_power_level() {
        let cases = vec![
            (Pos(  3,  5),  8,  4),
            (Pos(122, 79), 57, -5),
            (Pos(217,196), 39,  0),
            (Pos(101,153), 71,  4),
        ];

        for (pos, id, result) in cases {
            assert_eq!(Day11::power_level(pos, id), result);
        }
    }

    #[test]
    fn test_a() {
        assert_eq!(Day11::largest_power_level(18, 3..4), (Pos(33, 45), 3, 29));
        assert_eq!(Day11::largest_power_level(42, 3..4), (Pos(21, 61), 3, 30));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day11::largest_power_level(18, 1..301), (Pos(90, 269), 16, 113));
        assert_eq!(Day11::largest_power_level(42, 1..301), (Pos(232, 251), 12, 119));
    }

    #[bench]
    fn bench_a(b: &mut Bencher) {
        b.iter(test_a)
    }

    //#[bench]
    //fn bench_b(b: &mut Bencher) {
    //    b.iter(test_b)
    //}
}
