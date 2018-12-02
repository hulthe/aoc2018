use std::collections::HashMap;
use std::hash::Hash;

pub fn find_similar_id<S, I>(iterator: I) -> Option<String>
    where S: AsRef<str>,
          I: Iterator<Item=S>,
{
    let ids: Vec<S> = iterator.collect();

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
                return Some(id);
            }
        }
    }

    return None;
}

pub fn checksum<E, S, I>(iterator: I) -> i32
    where E: Eq + Clone + Hash,
          S: AsRef<[E]>,
          I: Iterator<Item=S>,
{

    let mut twos = 0;
    let mut threes = 0;
    for s in iterator {
        let mut has_two = false;
        let mut has_three = false;
        let mut map = HashMap::new();

        for c in s.as_ref() {
            if let Some(i) = map.insert(c.clone(), 1) {
                map.insert(c.clone(), i+1);
            }
        }

        for &c in map.values() {
            if c == 2 { has_two = true; }
            else if c == 3 { has_three = true; }
        }

        if has_two { twos += 1; }
        if has_three {threes += 1; }
    }

    return twos * threes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_similar_id() {
        let input = vec![
            "abcde",
            "fghij",
            "klmno",
            "pqrst",
            "fguij",
            "axcye",
            "wvxyz",
        ];
        assert_eq!(find_similar_id(input.iter()).unwrap(), "fgij");
    }

    #[test]
    fn test_checksum() {
        let input = vec![
            "abcdef",
            "bababc",
            "abbcde",
            "abcccd",
            "aabcdd",
            "abcdee",
            "ababab",
        ];
        assert_eq!(checksum(input.iter()), 12);
    }
}
