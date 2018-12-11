#![feature(test)]

use aoc_base::AoC;
use std::collections::VecDeque;
use std::error::Error;

pub struct Day09;

fn rotate(queue: &mut VecDeque<usize>, n: i32) {
    if queue.len() == 0 {
        return;
    }
    if n > 0 {
        for _ in 0..n {
            let f = queue.pop_front().unwrap();
            queue.push_back(f);
        }
    } else {
        for _ in 0..(-n) {
            let b = queue.pop_back().unwrap();
            queue.push_front(b);
        }
    }
}

fn parse_input(input: &str) -> Result<(usize, usize), Box<Error>> {
    let mut input_iter = input.split(" ");
    let player_count = input_iter.next().unwrap().parse::<usize>()?;
    let marble_count = input_iter.nth(5).unwrap().parse::<usize>()?;
    Ok((player_count, marble_count))
}

fn marble_game(player_count: usize, marble_count: usize) -> usize {
    let mut player_points: Vec<usize> = vec![0; player_count];

    let mut circle: VecDeque<usize> = VecDeque::with_capacity(marble_count);
    circle.push_back(0);

    let mut current_player = 0;

    for m in 1..=marble_count {
        if m % 23 == 0 {
            rotate(&mut circle, -7);
            let removed = circle.pop_back().unwrap();
            player_points[current_player] += removed + m;
            rotate(&mut circle, 1);
        } else {
            rotate(&mut circle, 1);
            circle.push_back(m);
        }
        current_player += 1;
        current_player %= player_count;
    }

    *player_points.iter().max().unwrap()
}

impl AoC<usize, usize> for Day09 {
    fn task_a(input: &str) -> Result<usize, Box<Error>> {
        let (player_count, marble_count) = parse_input(input)?;
        Ok(marble_game(player_count, marble_count))
    }

    fn task_b(input: &str) -> Result<usize, Box<Error>> {
        let (player_count, marble_count) = parse_input(input)?;
        Ok(marble_game(player_count, marble_count * 100))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::Day09;
    use aoc_base::AoC;

    const TEST_DATA: &[(&str, usize, usize)] = &[
        ("9 players; last marble is worth 25 points", 32, 22563),
        (
            "10 players; last marble is worth 1618 points",
            8317,
            74765078,
        ),
        (
            "13 players; last marble is worth 7999 points",
            146373,
            1406506154,
        ),
        (
            "17 players; last marble is worth 1104 points",
            2764,
            20548882,
        ),
        (
            "21 players; last marble is worth 6111 points",
            54718,
            507583214,
        ),
        (
            "30 players; last marble is worth 5807 points",
            37305,
            320997431,
        ),
    ];

    #[test]
    fn test_a() {
        for (case, result_a, _) in TEST_DATA {
            assert_eq!(Day09::task_a(case).unwrap(), *result_a);
        }
    }

    #[test]
    fn test_b() {
        for (case, _, result_b) in TEST_DATA {
            assert_eq!(Day09::task_b(case).unwrap(), *result_b);
        }
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
