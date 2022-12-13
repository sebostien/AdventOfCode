use std::time::Duration;

use advent_of_rust::{get_all_years, util::get_input_contents};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    for (_, sols) in get_all_years() {
        for sol in sols {
            let (year, day) = sol.get_date();
            let input = get_input_contents(year, day).unwrap();

            c.bench_function(format!("{}/{} :: Part 1", year, day).as_str(), |b| {
                b.iter(|| black_box(sol.part_1(&input)));
            });

            c.bench_function(format!("{}/{} :: Part 2", year, day).as_str(), |b| {
                b.iter(|| black_box(sol.part_2(&input)))
            });
        }
    }
}


criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(12));
    targets = criterion_benchmark 
}
criterion_main!(benches);
