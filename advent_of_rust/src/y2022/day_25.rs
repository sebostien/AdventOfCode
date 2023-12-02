use crate::Solution;

pub fn get_solution() -> Solution<String, usize> {
    Solution {
        date: (2022, 25),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: ("2=001=-2=--0212-22-2".to_string(), 0),
    }
}

struct Snafu(isize);

impl From<&str> for Snafu {
    fn from(value: &str) -> Self {
        let (_, sum) = value.trim().chars().rev().fold((1, 0), |(p, sum), c| {
            (
                p * 5,
                sum + match c {
                    '2' => 2 * p,
                    '1' => p,
                    '0' => 0,
                    '-' => -p,
                    '=' => -2 * p,
                    _ => unreachable!("Unknown SNAFU char {c}"),
                },
            )
        });
        Self(sum)
    }
}

impl std::fmt::Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n = self.0;
        let mut out = vec![0; 100];

        while n > 0 {
            let p = (n as f64).log(5.0).floor() as usize;
            let pow = 5_isize.pow(p as u32);
            let x = n / pow;

            match x {
                1 => {
                    out[p] += 1;
                    n -= pow;
                }
                2 => {
                    out[p] += 2;
                    n -= 2 * pow;
                }
                3 => {
                    out[p] -= 2;
                    out[p + 1] += 1;
                    n = n + 2 * pow - (pow * 5);
                }
                4 => {
                    out[p] -= 1;
                    out[p + 1] += 1;
                    n = n + pow - (pow * 5);
                }
                _ => unreachable!(),
            }
        }

        for i in 0..out.len() {
            let n = out[i];
            if n < -2 {
                out[i - 1] -= 5;
                out[i] += 1;
            } else if n > 2 {
                out[i] -= 5;
                out[i + 1] += 1;
            }
        }

        out.into_iter()
            .rev()
            .skip_while(|x| *x == 0)
            .map(|x| match x {
                0 => "0",
                1 => "1",
                2 => "2",
                -1 => "-",
                -2 => "=",
                _ => unreachable!(),
            })
            .collect::<String>()
            .fmt(f)
    }
}

fn part_1(input: &str) -> anyhow::Result<String> {
    let mut s = 0;
    for line in input.lines() {
        s += Snafu::from(line).0;
    }

    Ok(Snafu(s).to_string())
}

fn part_2(_: &str) -> anyhow::Result<usize> {
    // No Part Two for this day
    Ok(0)
}

