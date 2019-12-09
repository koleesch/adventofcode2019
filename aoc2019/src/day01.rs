use std::io;

use crate::error::Error;

pub fn run<R>(mut input: R) -> Result<(), Error>
where
    R: io::BufRead,
{
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;

    // part 1
    let mut reader = io::BufReader::new(&content[..]);
    run_part(&mut reader, part_one)?;

    // part 2
    let mut reader = io::BufReader::new(&content[..]);
    run_part(&mut reader, part_two)?;

    Ok(())
}

pub fn run_part<F, R>(input: &mut R, func: F) -> Result<(), Error>
where
    R: io::BufRead,
    F: Fn(usize) -> usize,
{
    let mut buffer = String::new();
    let mut total = 0;

    loop {
        if input.read_line(&mut buffer)? == 0 {
            break;
        }
        let n = buffer.trim().parse::<usize>()?;
        let fuel = func(n);
        total += fuel;

        buffer.clear();
    }

    println!("{}", total);

    Ok(())
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
/*
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
}*/
