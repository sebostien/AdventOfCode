use std::str::FromStr;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<isize>,
    operation: (Option<isize>, char, Option<isize>),
    test: isize,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xs = s
            .trim()
            .split('\n')
            .skip(1)
            .map(|r| r.split(' ').collect())
            .collect::<Vec<Vec<_>>>();

        let ol = xs[1][xs[1].len() - 3].parse();
        let or = xs[1][xs[1].len() - 1].parse();

        let items = xs[0]
            .iter()
            .skip(4)
            .map(|x| {
                if x.ends_with(',') {
                    x[0..x.len() - 1].parse().unwrap()
                } else {
                    x.parse().unwrap()
                }
            })
            .collect();

        Ok(Monkey {
            items,
            operation: (
                if let Ok(x) = ol { Some(x) } else { None },
                xs[1][xs[1].len() - 2].chars().next().unwrap(),
                if let Ok(x) = or { Some(x) } else { None },
            ),
            test: xs[2].last().unwrap().parse().unwrap(),
            if_true: xs[3].last().unwrap().parse().unwrap(),
            if_false: xs[4].last().unwrap().parse().unwrap(),
        })
    }
}

fn run(mut ms: Vec<Monkey>, rounds: isize, div: isize, common_divisor: isize) -> usize {
    let mut inspections = vec![0; ms.len()];

    for _ in 0..rounds {
        for i in 0..ms.len() {
            for j in 0..ms[i].items.len() {
                let left = if let Some(n) = ms[i].operation.0 {
                    n
                } else {
                    ms[i].items[j]
                };
                let right = if let Some(n) = ms[i].operation.2 {
                    n
                } else {
                    ms[i].items[j]
                };

                inspections[i] += 1;
                let new_level = (match ms[i].operation.1 {
                    '+' => left + right,
                    '*' => left * right,
                    _ => unreachable!("{:?}", ms[i].operation),
                } / div)
                    % common_divisor;

                let t = ms[i].if_true;
                let f = ms[i].if_false;
                if new_level % ms[i].test == 0 {
                    ms[t].items.push(new_level);
                } else {
                    ms[f].items.push(new_level);
                }
            }
            ms[i].items.truncate(0);
        }
    }

    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

fn parse(input: &str) -> Result<(Vec<Monkey>, isize), String> {
    let mut ms: Vec<Monkey> = vec![];
    let mut divisor = 1;
    for row in input.trim().split("\n\n") {
        let m: Monkey = row.parse()?;
        divisor *= m.test;
        ms.push(m);
    }

    Ok((ms, divisor))
}

fn part_1(input: String) -> Result<usize, String> {
    let (ms, d) = parse(&input)?;
    Ok(run(ms, 20, 3, d))
}

fn part_2(input: String) -> Result<usize, String> {
    let (ms, d) = parse(&input)?;
    Ok(run(ms, 10000, 1, d))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::{get_input_contents, get_year_day};

    #[test]
    fn test_part_1() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        let p1 = part_1(input);
        if let Ok(res) = p1 {
            assert_eq!(res, 100345);
        } else {
            println!("Parse Error \n{:?}", p1.unwrap_err());
            assert!(false);
        }
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        let p2 = part_2(input);
        if let Ok(res) = p2 {
            assert_eq!(res, 28537348205);
        } else {
            println!("Parse Error \n{:?}", p2.unwrap_err());
            assert!(false);
        }
    }
}
