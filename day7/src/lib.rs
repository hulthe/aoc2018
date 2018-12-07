#![feature(test)]

use aoc_base::AoC;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use binary_heap_plus::BinaryHeap;

pub struct Day7;

fn work_completion_time<T>(inputs: T, worker_count: usize, base_time: usize) -> Result<usize, Box<Error>>
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    let mut workers: HashMap<char, usize> = HashMap::new();
    let mut all_nodes: HashSet<char> = HashSet::new();
    let mut dependencies: HashMap<char, HashSet<char>> = HashMap::new();
    inputs.into_iter()
        .map(|l| {
            let mut cs = l.as_ref().chars();
            let (fst, snd) = (cs.nth(5).unwrap(), cs.nth(30).unwrap());
            all_nodes.insert(fst);
            all_nodes.insert(snd);
            (snd, fst)
        })
        .for_each(|(snd, fst)| {
            dependencies.entry(snd).or_insert(HashSet::new()).insert(fst);
        });

    let mut pqueue = BinaryHeap::new_min();
    all_nodes.iter()
        .filter(|&n| !dependencies.contains_key(n))
        .for_each(|&n| pqueue.push(n));

    let mut seconds = 0;
    loop {
        while workers.len() < worker_count {
            if let Some(n) = pqueue.pop() {
                let time = base_time + (n as usize) - 65;
                workers.insert(n, time);
            } else {
                break;
            }
        }
        if workers.len() == 0 {break};

        let mut work = true;
        while work {
            seconds += 1;
            workers = workers.into_iter()
                .filter_map(|(node, time_left)| {
                    if time_left > 0 {
                        Some((node, time_left - 1))
                    } else {
                        for (snd, fst) in &mut dependencies {
                            if fst.remove(&node) && fst.is_empty() {
                                pqueue.push(*snd);
                            }
                        }
                        work = false;
                        None
                    }
                })
                .collect();
        }
    }
    Ok(seconds)
}

impl<T> AoC<T, String, usize> for Day7
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    fn task_a(inputs: T) -> Result<String, Box<Error>> {
        let mut all_nodes: HashSet<char> = HashSet::new();
        let mut dependencies: HashMap<char, HashSet<char>> = HashMap::new();
        inputs.into_iter()
            .map(|l| {
                let mut cs = l.as_ref().chars();
                let (fst, snd) = (cs.nth(5).unwrap(), cs.nth(30).unwrap());
                all_nodes.insert(fst);
                all_nodes.insert(snd);
                (snd, fst)
            })
            .for_each(|(snd, fst)| {
                dependencies.entry(snd).or_insert(HashSet::new()).insert(fst);
            });

        let mut pqueue = BinaryHeap::new_min();
        all_nodes.iter()
            .filter(|&n| !dependencies.contains_key(n))
            .for_each(|&n| pqueue.push(n));

        let mut result: Vec<char> = vec![];
        while let Some(n) = pqueue.pop() {
            for (snd, fst) in &mut dependencies {
                if fst.remove(&n) && fst.is_empty() {
                    pqueue.push(*snd);
                }
            }
            result.push(n);
        }

        Ok(result.iter().collect())
    }

    fn task_b(inputs: T) -> Result<usize, Box<Error>> {
        work_completion_time(inputs, 5, 60)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use aoc_base::AoC;
    use super::*;

    const TEST_DATA: &[&str] = &[
        "Step C must be finished before step A can begin.",
        "Step C must be finished before step F can begin.",
        "Step A must be finished before step B can begin.",
        "Step A must be finished before step D can begin.",
        "Step B must be finished before step E can begin.",
        "Step D must be finished before step E can begin.",
        "Step F must be finished before step E can begin.",
    ];

    #[test]
    fn test_a() {
        assert_eq!(Day7::task_a(TEST_DATA).unwrap(), "CABDFE");
    }

    #[test]
    fn test_b() {
        assert_eq!(work_completion_time(TEST_DATA, 2, 0).unwrap(), 15);
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
