#![feature(test)]

use aoc_base::AoC;
use std::error::Error;
use std::collections::HashSet;

pub struct Day10;

type Vec2 = (i32, i32);
type Bounds = (i32, i32, i32, i32);
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

fn calc_bounds(points: &Points) -> Bounds {
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut max_y = std::i32::MIN;

    for ((x, y), _) in points.iter() {
        if *x < min_x { min_x = *x; }
        if *y < min_y { min_y = *y; }
        if *x > max_x { max_x = *x; }
        if *y > max_y { max_y = *y; }
    }
    (min_x, min_y, max_x, max_y)
}

fn out_of_bounds(points: &Points, original: &Bounds) -> bool {
    let (minx, miny, maxx, maxy) = original;
    let (minxp, minyp, maxxp, maxyp) = calc_bounds(&points);
    minxp < *minx ||
    minyp < *miny ||
    maxxp > *maxx ||
    maxyp > *maxy
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
        if y != max_y { drawn.push('\n'); }
    }

    drawn.into_iter().collect()
}

fn tick_points(points: &Points, dt: i32) -> Points {
    points.iter()
        .map(|(p, v)| {
            ((
                p.0 + v.0 * dt,
                p.1 + v.1 * dt,
            ), (v.0, v.1))
        })
        .collect()
}

fn scatter((_, min_y, _, max_y): Bounds) -> i32 {
    (max_y - min_y).abs()
}

fn solve_constellation(input: &str, font_height: i32) -> (String, i32) {
    let points = parse_input(input);
    let bounds = calc_bounds(&points);
    let init_scatter = scatter(bounds);

    let mut step_size: i32 = 2;
    while step_size < init_scatter / 20 {
        step_size *= 2;
    }

    let mut seconds = 0;
    let mut state: Points = points;
    while !out_of_bounds(&state, &bounds) {
        let curr_scatter = scatter(calc_bounds(&state));
        if curr_scatter == font_height {
            break;
        }

        let next = tick_points(&state, step_size);
        let next_scatter = scatter(calc_bounds(&next));

        if next_scatter < curr_scatter {
            state = next;
            seconds += step_size;
        } else {
            let half_step = (step_size / 2).max(1);
            state = tick_points(&state, -half_step);
            step_size = half_step;
            seconds -= half_step;
        }
    }

    (draw_points(&state), seconds)
}

impl AoC<String, i32> for Day10 {
    fn task_a(input: &str) -> Result<String, Box<Error>> {
        let (rendered, _) = solve_constellation(input, 9);
        Ok(rendered)
    }

    fn task_b(input: &str) -> Result<i32, Box<Error>> {
        let (_, seconds) = solve_constellation(input, 9);
        Ok(seconds)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::*;

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
    fn test() {
        assert_eq!(
            solve_constellation(TEST_DATA, 7),
            (
                "#...#..###\n\
                #...#...#.\n\
                #...#...#.\n\
                #####...#.\n\
                #...#...#.\n\
                #...#...#.\n\
                #...#...#.\n\
                #...#..###".into(),
                3
            ));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(test)
    }
}
