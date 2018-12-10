#![feature(test)]
#![feature(try_trait)]
#![feature(uniform_paths)]
mod tree;

use aoc_base::AoC;
use std::error::Error;
use tree::TreeNode;

pub struct Day8;

impl AoC<usize, usize> for Day8 {
    fn task_a(input: &str) -> Result<usize, Box<Error>> {
        let tree: TreeNode = input.parse()?;
        Ok(tree.iter()
            .map(|m| m.iter().sum::<usize>())
            .sum())
    }

    fn task_b(input: &str) -> Result<usize, Box<Error>> {
        fn rec_sum(node: &TreeNode) -> usize {
            if node.children.len() == 0 {
                node.metadata.iter().sum()
            } else {
                node.metadata.iter()
                    .filter(|&&m| m != 0)
                    .map(|m| m - 1)
                    .filter_map(|m| node.children.get(m))
                    .map(|c| rec_sum(c))
                    .sum()
            }
        }
        Ok(rec_sum(&input.parse()?))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::Day8;
    use aoc_base::AoC;

    const TEST_DATA: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_a() {
        assert_eq!(Day8::task_a(TEST_DATA).unwrap(), 138);
    }

    #[test]
    fn test_b() {
        assert_eq!(Day8::task_b(TEST_DATA).unwrap(), 66);
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
