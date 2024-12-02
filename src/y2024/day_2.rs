use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (202, 271),
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let reports = input
        .trim()
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse::<usize>().unwrap()));

    let mut s = 0;

    'outer: for mut report in reports {
        let mut inc = true;
        let mut dec = true;

        let mut prev = report.next().unwrap();
        for c in report {
            if c > prev {
                dec = false;
            }

            if c < prev {
                inc = false;
            }

            if !(1..=3).contains(&prev.abs_diff(c)) {
                continue 'outer;
            }

            prev = c;
        }

        s += usize::from(inc || dec);
    }

    Ok(s)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let reports = input.trim().lines().map(|line| {
        line.split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut s = 0;
    for report in reports {
        'outer: for i in 0..report.len() {
            let mut inc = true;
            let mut dec = true;
            let mut prev = if i == 0 { report[1] } else { report[0] };

            for (j, &c) in report.iter().enumerate().skip(1) {
                if i == 0 && j < 2 || i == j {
                    continue;
                }

                dec &= c > prev;
                inc &= c < prev;

                if !(1..=3).contains(&prev.abs_diff(c)) {
                    continue 'outer;
                }

                prev = c;
            }

            if inc || dec {
                s += 1;
                break;
            }
        }
    }

    Ok(s)
}
