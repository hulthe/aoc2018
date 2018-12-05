use aoc_base::AoC;
use std::collections::HashMap;
use std::error::Error;

pub struct Day2;

impl<T> AoC<T, i32, String> for Day2
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    /// Compute a checksum for the ids
    fn task_a(inputs: T) -> Result<i32, Box<Error>> {
        let mut twos = 0;
        let mut threes = 0;
        for s in inputs {
            let mut has_two = false;
            let mut has_three = false;
            let mut map = HashMap::new();

            for c in s.as_ref().chars() {
                *map.entry(c).or_insert(0) += 1;
            }

            for &c in map.values() {
                if c == 2 {
                    has_two = true;
                } else if c == 3 {
                    has_three = true;
                }
            }

            if has_two {
                twos += 1;
            }
            if has_three {
                threes += 1;
            }
        }

        return Ok(twos * threes);
    }

    /// Find the one id which only differs by one character to another id
    fn task_b(inputs: T) -> Result<String, Box<Error>> {
        let ids: Vec<String> = inputs.into_iter().map(|s| s.as_ref().to_owned()).collect();

        // Return a vec of indices for the differing elements
        // Will return strange results if s1 and s2 are of different lengths
        let diffs = |s1: &str, s2: &str| -> Vec<usize> {
            let mut ds = Vec::with_capacity(s1.len());
            let mut i2 = s2.chars();
            for (i, c1) in s1.chars().enumerate() {
                if let Some(c2) = i2.next() {
                    if c1 != c2 {
                        ds.push(i);
                    }
                }
            }
            ds
        };

        for (i, s1) in ids.iter().enumerate() {
            for s2 in ids.iter().take(i) {
                let ds = diffs(s1.as_ref(), s2.as_ref());
                if ds.len() == 1 {
                    let mut id = String::from(s1.as_ref());
                    id.remove(ds[0]);
                    return Ok(id);
                }
            }
        }

        Ok("".into()) // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_base::AoC;

    #[test]
    fn test_find_similar_id() {
        let input = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        assert_eq!(Day2::task_b(input).unwrap(), "fgij");
    }

    #[test]
    fn test_checksum() {
        let input = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(Day2::task_a(input).unwrap(), 12);
    }
}
