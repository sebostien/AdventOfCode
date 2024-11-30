use advent_of_rust::test_year_day;

fn help() {
    println!(
        "{}",
        r#"
Advent of Rust

USAGE:
    advent_of_rust YEAR DAY

To test all solutions:
    run `cargo test` instead

"#
        .trim()
    );
}

fn run_test(year: u32, day: u32) {
    let years = advent_of_rust::get_all_years();

    let mut found = false;
    for (y, sols) in years {
        if y == year {
            for sol in sols {
                let (y, d) = sol.get_date();
                if y == year && d == day {
                    let (correct, e) = test_year_day(y, d, sol.as_ref());
                    if correct {
                        println!("All done!");
                    } else {
                        println!("{e}")
                    }
                    found = true;
                    break;
                }
            }
        }
    }

    if !found {
        println!("Could not find solution for {}/{}", year, day);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.len() {
        2 => {
            let year = if let Ok(y) = args[0].parse() {
                y
            } else {
                println!("{} is not a valid year", args[0]);
                std::process::exit(1);
            };

            let day = if let Ok(d) = args[1].parse() {
                d
            } else {
                println!("{} is not a valid day", args[1]);
                std::process::exit(1);
            };

            run_test(year, day);
        }
        _ => help(),
    }
}
