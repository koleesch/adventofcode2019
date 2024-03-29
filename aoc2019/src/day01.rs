use std::io;

use crate::error::Error;

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let mut total1 = 0;
    let mut total2 = 0;

    for res in input.lines() {
        let line = res?;
        let n = line.trim().parse::<usize>()?;
        total1 += part_one(n);
        total2 += part_two(n);
    }

    Ok((format!("{}", total1), format!("{}", total2)))
}

fn part_one(n: usize) -> usize {
    match (n / 3).checked_sub(2) {
        Some(m) => m,
        None => 0,
    }
}

fn part_two(mut n: usize) -> usize {
    let mut total = 0;
    loop {
        let m = match (n / 3).checked_sub(2) {
            Some(m) => m,
            None => break total,
        };
        total += m;
        n = m;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let test_cases = &[
            // input, expected1, expected2
            ("12", "2", "2"),
            ("14", "2", "2"),
            ("1969", "654", "966"),
            ("100756", "33583", "50346"),
        ];

        for (input, expected1, expected2) in test_cases {
            let reader = io::BufReader::new(input.as_bytes());
            let (actual1, actual2) = run(reader).unwrap();
            assert_eq!(*expected1, actual1);
            assert_eq!(*expected2, actual2);
        }
    }
}
