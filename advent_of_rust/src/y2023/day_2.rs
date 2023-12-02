use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        date: (2023, 2),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (2061, 72596),
    }
}
const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn part_1(input: &str) -> anyhow::Result<usize> {
    let a = input
        .lines()
        .enumerate()
        .filter_map(|(id, line)| {
            for set in line[line.find(':').unwrap_or(0) + 1..].split(';') {
                for balls in set.split(',') {
                    let num: usize = balls
                        .chars()
                        .filter(char::is_ascii_digit)
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    let color = balls.chars().find(char::is_ascii_alphabetic);
                    match color {
                        Some('r') => {
                            if num > MAX_RED {
                                return None;
                            }
                        }
                        Some('g') => {
                            if num > MAX_GREEN {
                                return None;
                            }
                        }
                        Some('b') => {
                            if num > MAX_BLUE {
                                return None;
                            }
                        }
                        _ => unreachable!("Unexpected color: {color:?}"),
                    }
                }
            }

            Some(id + 1)
        })
        .sum();

    Ok(a)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let a = input
        .lines()
        .map(|line| {
            let (r, g, b) = line[line.find(':').unwrap() + 1..].split(';').fold(
                (0, 0, 0),
                |(mut r, mut g, mut b), set| {
                    for balls in set.split(',') {
                        let num: usize = balls
                            .chars()
                            .filter(char::is_ascii_digit)
                            .collect::<String>()
                            .parse()
                            .unwrap();

                        let color = balls.chars().find(char::is_ascii_alphabetic);
                        match color {
                            Some('r') => {
                                r = r.max(num);
                            }
                            Some('g') => {
                                g = g.max(num);
                            }
                            Some('b') => {
                                b = b.max(num);
                            }
                            _ => unreachable!("Unexpected color: {color:?}"),
                        }
                    }
                    (r, g, b)
                },
            );
            r * g * b
        })
        .sum();

    Ok(a)
}
