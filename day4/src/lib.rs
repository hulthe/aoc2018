#![feature(test)]

use aoc_base::AoC;
use chrono::{prelude::*, Duration};
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;

pub struct Day4;

#[derive(Debug, PartialEq)]
enum GuardState {
    Awake,
    Asleep,
}

impl Day4 {
    fn parse_inputs(inputs: &str) -> Vec<(NaiveDateTime, usize, GuardState)> {
        let mut vec: Vec<String> = inputs.lines().map(|s| s.to_owned()).collect();
        vec.sort();

        let mut last_id: usize = 0;
        vec.iter()
            .map(|l| {
                (
                    Utc.datetime_from_str(&l[1..17], "%Y-%m-%d %H:%M")
                        .unwrap()
                        .naive_utc(),
                    &l[19..],
                )
            })
            .map(|(t, r)| {
                let state = match r.trim() {
                    "wakes up" => GuardState::Awake,
                    "falls asleep" => GuardState::Asleep,
                    new_shift => {
                        last_id = new_shift[7..]
                            .trim_end_matches(" begins shift")
                            .parse::<usize>()
                            .unwrap();
                        GuardState::Awake
                    }
                };
                (t, last_id, state)
            })
            .collect()
    }

    fn calculate_schedule(inputs: &str) -> HashMap<usize, ([usize; 60], usize)> {
        let mut sleep_schedule: HashMap<usize, ([usize; 60], usize)> = HashMap::new();

        let inputs = Self::parse_inputs(inputs);

        let minute = Duration::minutes(1);
        let mut iter = inputs.iter();
        let mut last: &(NaiveDateTime, usize, GuardState) = iter.next().unwrap().clone();
        for this in iter {
            let (time, id, _) = this;
            let (last_time, last_id, last_state) = last;
            if *last_state == GuardState::Asleep {
                if id != last_id {
                    let mut tick = last_time.time();
                    if tick.hour() != 0 {
                        panic!("Guard slept through his entire shift");
                    }
                    while tick < NaiveTime::from_hms(0, 59, 0) {
                        let (schedule, sum) =
                            sleep_schedule.entry(*last_id).or_insert(([0; 60], 0));
                        schedule[tick.minute() as usize] += 1;
                        *sum += 1;
                        tick += minute;
                    }
                } else {
                    let mut tick = last_time.clone();
                    if tick.hour() != 0 {
                        panic!("Guard is asleep before he even started...");
                    }
                    while tick < *time {
                        let (schedule, sum) = sleep_schedule.entry(*id).or_insert(([0; 60], 0));
                        schedule[tick.minute() as usize] += 1;
                        *sum += 1;
                        tick += minute;
                    }
                }
            }
            last = this;
        }

        sleep_schedule
    }
}

impl AoC<usize, usize> for Day4 {
    fn task_a(inputs: &str) -> Result<usize, Box<Error>> {
        let sleep_schedule = Day4::calculate_schedule(inputs);

        let (sleepiest_guard, _) = sleep_schedule
            .par_iter()
            .map(|(guard, (_, sum))| (guard, sum))
            .reduce(
                || (&0, &0),
                |guard1, guard2| match guard1.1 > guard2.1 {
                    true => guard1,
                    false => guard2,
                },
            );

        //let mut sleepiest_guard: usize = 0;
        //let mut sleepiest_sum: usize = 0;
        //for (guard, (_, sum)) in sleep_schedule.iter() {
        //    if *sum > sleepiest_sum {
        //        sleepiest_guard = *guard;
        //        sleepiest_sum = *sum;
        //    }
        //}

        let mut sleepiest_minute: usize = 0;
        let mut sleepiest_minute_i: usize = 0;
        let (schedule, _) = sleep_schedule.get(&sleepiest_guard).unwrap();
        for i in 0..60 {
            if schedule[i] > sleepiest_minute {
                sleepiest_minute = schedule[i];
                sleepiest_minute_i = i;
            }
        }

        Ok(sleepiest_guard * sleepiest_minute_i)
    }

    fn task_b(inputs: &str) -> Result<usize, Box<Error>> {
        let sleep_schedule = Day4::calculate_schedule(inputs);

        let (guard, minute_index, _) = sleep_schedule
            .iter()
            .map(|(guard, (schedule, _))| (guard, schedule))
            .map(|(guard, schedule)| {
                let (minute_index, minute_count) = (0..60)
                    .map(|i| (i, schedule[i]))
                    .fold((0, 0), |m1, m2| if m1.1 > m2.1 { m1 } else { m2 });
                (guard, minute_index, minute_count)
            })
            .fold((&0, 0, 0), |g1, g2| if g1.2 > g2.2 { g1 } else { g2 });

        Ok(guard * minute_index)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::Day4;
    use aoc_base::AoC;

    const TEST_DATA: &str = "[1518-11-01 00:55] wakes up              \n\
                             [1518-11-04 00:02] Guard #99 begins shift\n\
                             [1518-11-01 23:58] Guard #99 begins shift\n\
                             [1518-11-05 00:45] falls asleep          \n\
                             [1518-11-05 00:03] Guard #99 begins shift\n\
                             [1518-11-03 00:24] falls asleep          \n\
                             [1518-11-03 00:05] Guard #10 begins shift\n\
                             [1518-11-05 00:55] wakes up              \n\
                             [1518-11-03 00:29] wakes up              \n\
                             [1518-11-01 00:00] Guard #10 begins shift\n\
                             [1518-11-02 00:40] falls asleep          \n\
                             [1518-11-01 00:05] falls asleep          \n\
                             [1518-11-02 00:50] wakes up              \n\
                             [1518-11-04 00:36] falls asleep          \n\
                             [1518-11-01 00:25] wakes up              \n\
                             [1518-11-01 00:30] falls asleep          \n\
                             [1518-11-04 00:46] wakes up              ";

    #[test]
    fn test_a() {
        assert_eq!(Day4::task_a(TEST_DATA).unwrap(), 240);
    }

    #[test]
    fn test_b() {
        assert_eq!(Day4::task_b(TEST_DATA).unwrap(), 4455);
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
