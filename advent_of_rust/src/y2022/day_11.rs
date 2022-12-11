#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<isize>,
    operation: (Option<isize>, char, Option<isize>),
    test: isize,
    if_true: usize,
    if_false: usize,
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

fn parse(input: &str) -> (Vec<Monkey>, isize) {
    let mut ms = vec![];
    let mut divisor = 1;
    for m in input.trim().split("\n\n") {
        let xs = m
            .trim()
            .split('\n')
            .map(|r| r.split(' ').collect())
            .collect::<Vec<Vec<_>>>();

        let ol = xs[2][xs[2].len() - 3].parse();
        let or = xs[2][xs[2].len() - 1].parse();

        ms.push(Monkey {
            items: xs[1]
                .iter()
                .skip(4)
                .map(|x| {
                    if x.ends_with(",") {
                        x[0..x.len() - 1].parse().unwrap()
                    } else {
                        x.parse().unwrap()
                    }
                })
                .collect(),
            operation: (
                if ol.is_err() { None } else { Some(ol.unwrap()) },
                xs[2][xs[2].len() - 2].chars().next().unwrap(),
                if or.is_err() { None } else { Some(or.unwrap()) },
            ),
            test: xs[3].last().unwrap().parse().unwrap(),
            if_true: xs[4].last().unwrap().parse().unwrap(),
            if_false: xs[5].last().unwrap().parse().unwrap(),
        });
        divisor *= ms.last().unwrap().test;
    }

    (ms, divisor)
}

fn part_1(input: String) -> usize {
    let (ms, d) = parse(&input);
    run(ms, 20, 3, d)
}

fn part_2(input: String) -> usize {
    let (ms, d) = parse(&input);
    run(ms, 10000, 1, d)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::{get_input_contents, get_year_day};

    #[test]
    fn test_part_1() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_1(input), 100345);
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_2(input), 28537348205);
    }
}
