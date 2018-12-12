#![feature(test)]

use aoc_base::AoC;
use std::collections::HashSet;
use std::error::Error;

pub struct Day12;

impl Day12 {
    fn parse_input(input: &str) -> (HashSet<i64>, HashSet<String>) {
        let mut lines = input.lines();

        let state: HashSet<i64> = lines
            .next()
            .unwrap()
            .trim_start_matches("initial state: ")
            .trim()
            .chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '#' => Some(i as i64),
                _ => None,
            })
            .collect();

        let valid_combos: HashSet<String> = lines
            .skip(1)
            .map(|l| l.trim())
            .filter(|l| l.chars().last().unwrap() == '#')
            .map(|l| l.chars().take(5).collect())
            .collect();

        (state, valid_combos)
    }

    fn pattern_equals(a: &HashSet<i64>, b: &HashSet<i64>) -> Option<i64> {
        if a.len() != b.len() {
            None
        } else if a.len() == 0 {
            Some(0)
        } else {
            let amin = a.iter().min().unwrap();
            let bmin = b.iter().min().unwrap();
            let diff = bmin - amin;
            if a.iter().filter(|&i| b.contains(&(i + diff))).count() == a.len() {
                Some(diff)
            } else {
                None
            }
        }
    }

    fn get_plant_range(state: &HashSet<i64>) -> (i64, i64) {
        let (min, max) = state
            .iter()
            .fold((0, 0), |(min, max), i| (min.min(*i), max.max(*i)));
        (min - 2, max + 2)
    }

    fn solve(input: &str, iterations: i64) -> i64 {
        let (mut state, valid_combos) = Self::parse_input(input);
        for i in 1..=iterations {
            let mut new_state = HashSet::new();
            let (min, max) = Self::get_plant_range(&state);
            for j in min..=max {
                let neighbors: String = (j - 2..=j + 2)
                    .map(|jp| state.contains(&jp))
                    .map(|ps| match ps {
                        true => '#',
                        false => '.',
                    })
                    .collect();
                if valid_combos.contains(&neighbors) {
                    new_state.insert(j);
                }
            }

            if let Some(d) = Self::pattern_equals(&state, &new_state) {
                let sum: i64 = new_state.iter().sum();
                let iter_left = iterations - i;
                let step = d * state.len() as i64;
                return sum + step * iter_left;
            }

            std::mem::swap(&mut state, &mut new_state);
        }
        state.iter().sum()
    }
}

impl AoC<i64, i64> for Day12 {
    fn task_a(input: &str) -> Result<i64, Box<Error>> {
        Ok(Self::solve(input, 20))
    }

    fn task_b(input: &str) -> Result<i64, Box<Error>> {
        Ok(Self::solve(input, 50000000000))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::Day12;
    use aoc_base::AoC;

    const TEST_DATA: &str = "initial state: #..#.#..##......###...###\n\n\
                             ...## => #\n\
                             ..#.. => #\n\
                             .#... => #\n\
                             .#.#. => #\n\
                             .#.## => #\n\
                             .##.. => #\n\
                             .#### => #\n\
                             #.#.# => #\n\
                             #.### => #\n\
                             ##.#. => #\n\
                             ##.## => #\n\
                             ###.. => #\n\
                             ###.# => #\n\
                             ####. => #";

    #[test]
    fn test_compare_pattern() {
        let a = (1..5).collect();
        let b = (3..7).collect();
        assert_eq!(Day12::pattern_equals(&a, &b), Some(2));
    }

    #[test]
    fn test_a() {
        assert_eq!(Day12::task_a(TEST_DATA).unwrap(), 325);
    }

    #[bench]
    fn bench_a(b: &mut Bencher) {
        b.iter(test_a)
    }
}
