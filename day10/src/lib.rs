#![feature(test)]

use aoc_base::AoC;
use std::error::Error;
use std::collections::HashSet;

pub struct Day10;

type Vec2 = (i32, i32);
type Points = Vec<(Vec2, Vec2)>;

fn parse_input(input: &str) -> Points {
    input
        .lines()
        .map(|l| {
            let mut vals = l.trim_start_matches("position=<")
            .trim_end_matches(">")
            .split("> velocity=<")
            .map(|vec| {
                 let mut ns = vec.split(",")
                    .map(|n| n.trim().parse::<i32>().unwrap());
                 (ns.next().unwrap(), ns.next().unwrap())
            });
            (vals.next().unwrap(), vals.next().unwrap())
        })
        .collect()
}

fn calc_bounds(points: &Points) -> (i32, i32, i32, i32) {
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut max_y = std::i32::MIN;

    for (p, _) in points.iter() {
        if p.0 < min_x { min_x = p.0; }
        if p.1 < min_y { min_y = p.1; }
        if p.0 > max_x { max_x = p.0; }
        if p.1 > max_y { max_y = p.1; }
    }
    (min_x, min_y, max_x, max_y)
}

fn draw_points(points: &Points) -> String {
    let (min_x, min_y, max_x, max_y) = calc_bounds(points);
    let map: HashSet<Vec2> = points.iter().map(|(p, _)| *p).collect();
    let mut drawn: Vec<char> = vec![];
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.get(&(x, y)).is_some() {
                drawn.push('#');
            } else {
                drawn.push('.');
            }
        }
        drawn.push('\n');
    }

    drawn.into_iter().collect()
}

fn tick_points(points: &mut Points) {
    *points = points.iter()
        .map(|(p, v)| {
            ((
                p.0 + v.0,
                p.1 + v.1,
            ), (v.0, v.1))
        })
        .collect();
}

fn test_confidence(points: &Points) -> f32 {
    let map: HashSet<Vec2> = points.iter().map(|(p, _)| *p).collect();
    let len = map.len();

    let adjacent = |p: &Vec2| -> Vec<Vec2> {
        vec![
            (p.0 + 1, p.1),
            (p.0 - 1, p.1),
            (p.0, p.1 + 1),
            (p.0, p.1 - 1),
            (p.0 + 1, p.1 + 1),
            (p.0 + 1, p.1 - 1),
            (p.0 - 1, p.1 + 1),
            (p.0 - 1, p.1 - 1),
        ]
    };

    let mut vec = Vec::with_capacity(len);
    for p in map.iter() {
        if adjacent(p).into_iter().filter(|a| map.get(a).is_some()).count() == 2 {
            vec.push(p);
        }
    }

    vec.len() as f32 / len as f32
}

impl AoC<String, usize> for Day10 {
    fn task_a(input: &str) -> Result<String, Box<Error>> {
        let mut points = parse_input(input);

        let mut possible_states: Vec<(f32, usize, Points)> = Vec::new();

        let init_bounds = calc_bounds(&points);
        let mut iteration = 0;
        loop {
            tick_points(&mut points);
            iteration += 1;

            let new_bounds = calc_bounds(&points);
            if new_bounds.0 < init_bounds.0 ||
                new_bounds.1 < init_bounds.1 ||
                new_bounds.2 > init_bounds.2 ||
                new_bounds.3 > init_bounds.3 {
                break;
            }

            let confidence = test_confidence(&points);
            if confidence > 0.0 {
                possible_states.push((confidence, iteration, points.clone()))
            };
        }

        let state = possible_states.into_iter().fold((0.0, 0, vec![]), |a, b| {
            if a.0 > b.0 {a} else {b}
        });

        let rendered = draw_points(&state.2);

        //println!("Total {} iterations.", iteration);
        //println!("Result after {} iterations.", state.1);
        //println!("Confidence: {}", state.0);
        Ok(rendered)
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
    use super::Day10;

    const TEST_DATA: &str = "position=< 9,  1> velocity=< 0,  2>\n\
                             position=< 7,  0> velocity=<-1,  0>\n\
                             position=< 3, -2> velocity=<-1,  1>\n\
                             position=< 6, 10> velocity=<-2, -1>\n\
                             position=< 2, -4> velocity=< 2,  2>\n\
                             position=<-6, 10> velocity=< 2, -2>\n\
                             position=< 1,  8> velocity=< 1, -1>\n\
                             position=< 1,  7> velocity=< 1,  0>\n\
                             position=<-3, 11> velocity=< 1, -2>\n\
                             position=< 7,  6> velocity=<-1, -1>\n\
                             position=<-2,  3> velocity=< 1,  0>\n\
                             position=<-4,  3> velocity=< 2,  0>\n\
                             position=<10, -3> velocity=<-1,  1>\n\
                             position=< 5, 11> velocity=< 1, -2>\n\
                             position=< 4,  7> velocity=< 0, -1>\n\
                             position=< 8, -2> velocity=< 0,  1>\n\
                             position=<15,  0> velocity=<-2,  0>\n\
                             position=< 1,  6> velocity=< 1,  0>\n\
                             position=< 8,  9> velocity=< 0, -1>\n\
                             position=< 3,  3> velocity=<-1,  1>\n\
                             position=< 0,  5> velocity=< 0, -1>\n\
                             position=<-2,  2> velocity=< 2,  0>\n\
                             position=< 5, -2> velocity=< 1,  2>\n\
                             position=< 1,  4> velocity=< 2,  1>\n\
                             position=<-2,  7> velocity=< 2, -2>\n\
                             position=< 3,  6> velocity=<-1, -1>\n\
                             position=< 5,  0> velocity=< 1,  0>\n\
                             position=<-6,  0> velocity=< 2,  0>\n\
                             position=< 5,  9> velocity=< 1, -2>\n\
                             position=<14,  7> velocity=<-2,  0>\n\
                             position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn test_a() {
        assert_eq!(Day10::task_a(TEST_DATA).unwrap(), "");
    }

    #[test]
    fn test_b() {
        //assert_eq!(Day10::task_b(TEST_DATA).unwrap(), FIXME);
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
