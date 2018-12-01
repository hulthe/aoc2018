use std::io::{BufRead};
use std::error::Error;

fn parse_freqs<'a, R: BufRead + 'a>(reader: R) -> Box<(dyn Iterator<Item=i32> + 'a)> {
    let iter = reader.lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| s.parse::<i32>().ok());
    Box::new(iter)
}

pub fn sum_freqs<R: BufRead>(reader: R) -> Result<i32, Box<Error>> {
    let sum = parse_freqs(reader)
        .sum();

    Ok(sum)
}

pub fn freqs_first_dup<R: BufRead>(reader: R) -> Result<i32, Box<Error>> {
    let pattern: Vec<i32> = parse_freqs(reader).collect();
    let mut history: Vec<i32> = vec![0];
    loop {
        for num in pattern.iter() {
            let new = history.last().unwrap() + num;
            if history.contains(&new) {
                return Ok(new);
            }
            history.push(new);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn sum_test() {
        let cursor = io::Cursor::new(b"+4\n-6\n+33");
        assert_eq!(sum_freqs(cursor).unwrap(), 31);
    }

    #[test]
    fn first_dup_test() {
        let mut cr = vec![
            ("1\n-1", 0),
            ("+3\n+3\n+4\n-2\n-4", 10),
            ("-6\n+3\n+8\n+5\n-6", 5),
            ("+7\n+7\n-2\n-7\n-4", 14),
        ];
        let cases = cr.iter_mut().map(|(i, r)| (io::Cursor::new(i), r));

        for (i, r) in cases {
            assert_eq!(freqs_first_dup(i).unwrap(), r.clone());
        }
    }
}
