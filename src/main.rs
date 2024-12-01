use anyhow::anyhow;
use clap::Parser;
use criterion::Criterion;
use std::time::Duration;

#[derive(Parser)]
#[command(about)]
enum Cli {
    Test {
        #[command()]
        year: Option<usize>,
        #[command()]
        day: Option<usize>,
        #[arg(short, long, default_value = "false")]
        release: bool,
    },
    Bench {
        #[command()]
        year: usize,
        #[command()]
        day: usize,
        #[command()]
        part: usize,
    },
}

fn main() {
    let command = Cli::parse();

    match command {
        Cli::Test { year, day, release } => {
            if let Err(e) = run_test(year, day, release) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        Cli::Bench { year, day, part } => {
            if let Err(e) = run_bench(year, day, part) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }
}

fn run_test(year: Option<usize>, day: Option<usize>, release: bool) -> std::io::Result<()> {
    let mut command = std::process::Command::new("cargo");
    command.arg("test");

    if release {
        command.arg("--release");
    }

    match (year, day) {
        (Some(year), None) => {
            command.arg("--").arg(format!("y{year}"));
        }
        (Some(year), Some(day)) => {
            command.arg(format!("y{year}::d{day}"));
        }
        _ => {}
    }

    command.spawn()?.wait()?;
    Ok(())
}

fn run_bench(year: usize, day: usize, part: usize) -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        return Err(anyhow!("Please run the bencher in release mode"));
    }

    let input = advent_of_rust::download::get_input_contents(year, day)?;
    let solution = advent_of_rust::_include::get_solution(year, day);
    let mut bencher = Criterion::default().measurement_time(Duration::from_secs(12));

    match part {
        1 => {
            bencher.bench_function(&format!("y{year}::d{day}_part{part}"), |b| {
                b.iter(|| assert!(solution.part_1(&input).unwrap().0))
            });
        }
        2 => {
            bencher.bench_function(&format!("y{year}::d{day}_part{part}"), |b| {
                b.iter(|| assert!(solution.part_2(&input).unwrap().0))
            });
        }
        _ => panic!("No part {part}"),
    }

    Ok(())
}
