#![feature(test)]

use aoc_base::AoC;
use binary_heap_plus::BinaryHeap;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub struct Day7;

fn get_nodes_with_dependencies(input: &str) -> (HashSet<char>, HashMap<char, HashSet<char>>) {
    let input: Vec<&str> = input.lines().collect();
    let fold_id = || (HashSet::new(), HashMap::new());
    input
        .into_par_iter()
        .map(|l| {
            let mut cs = l.chars();
            let (fst, snd) = (cs.nth(5).unwrap(), cs.nth(30).unwrap());
            (snd, fst)
        })
        .fold(fold_id, |(mut nodes, mut deps), (snd, fst)| {
            nodes.insert(fst);
            nodes.insert(snd);
            deps.entry(snd).or_insert(HashSet::new()).insert(fst);
            (nodes, deps)
        })
        .reduce(fold_id, |(mut nds1, mut dps1), (nds2, dps2)| {
            nds1.extend(nds2);
            for (k, v) in dps2.iter() {
                dps1.entry(*k).or_insert(HashSet::new()).extend(v);
            }
            (nds1, dps1)
        })
}

fn work_completion_time(
    input: &str,
    worker_count: usize,
    base_time: usize,
) -> Result<usize, Box<Error>> {
    let mut workers: HashMap<char, usize> = HashMap::new();

    let (all_nodes, mut dependencies) = get_nodes_with_dependencies(input);

    let mut pqueue = BinaryHeap::new_min();
    all_nodes
        .iter()
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
        if workers.len() == 0 {
            break;
        };

        let mut work = true;
        while work {
            seconds += 1;
            workers = workers
                .into_iter()
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

impl AoC<String, usize> for Day7 {
    fn task_a(input: &str) -> Result<String, Box<Error>> {
        let (all_nodes, mut dependencies) = get_nodes_with_dependencies(input);
        let mut pqueue = BinaryHeap::new_min();
        all_nodes
            .iter()
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

    fn task_b(inputs: &str) -> Result<usize, Box<Error>> {
        work_completion_time(inputs, 5, 60)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::*;
    use aoc_base::AoC;

    const TEST_DATA: &str = "Step C must be finished before step A can begin.\n\
                             Step C must be finished before step F can begin.\n\
                             Step A must be finished before step B can begin.\n\
                             Step A must be finished before step D can begin.\n\
                             Step B must be finished before step E can begin.\n\
                             Step D must be finished before step E can begin.\n\
                             Step F must be finished before step E can begin.";

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
