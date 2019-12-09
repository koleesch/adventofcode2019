use std::fs;
use std::io;
use std::time::Duration;

use aoc2019;
use criterion::{criterion_group, criterion_main, Criterion}

fn target_01(c: &mut Criterion){
    let day01 = fs::read_to_string("input/day01.txt").unwrap();
    c.bench_function("day_01", |b| {
        b.iter(|| {
            let reader = io.BufReader::new(day01.as_bytes());
            aoc2019::day01::run(reader).unwrap();
        })
    });
}
