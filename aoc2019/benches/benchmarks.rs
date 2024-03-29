use std::fs;
use std::io;
use std::time::Duration;

use aoc2019;
use criterion::{criterion_group, criterion_main, Criterion};

fn target_01(c: &mut Criterion) {
    let day_01 = fs::read_to_string("input/day01.txt").unwrap();
    c.bench_function("day_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day_01.as_bytes());
            aoc2019::day01::run(reader).unwrap();
        })
    });
}

fn target_02(c: &mut Criterion) {
    let day_02 = fs::read_to_string("input/day02.txt").unwrap();
    c.bench_function("day_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day_02.as_bytes());
            aoc2019::day02::run(reader).unwrap();
        })
    });
}
fn target_03(c: &mut Criterion) {
    let day_03 = fs::read_to_string("input/day03.txt").unwrap();
    c.bench_function("day_03", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day_03.as_bytes());
            aoc2019::day02::run(reader).unwrap();
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().warm_up_time(Duration::from_secs(5));
    targets = target_01, target_02, target_03
}
criterion_main!(benches);
