use std::collections::BTreeMap;

pub fn get_solution() -> crate::Solution<usize, usize> {
    crate::Solution {
        date: (2023, 4),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (22_897, 5_095_824),
    }
}

fn parse(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut o = vec![];

    for line in input.trim().lines() {
        let line = &line[line.find(':').unwrap() + 1..];
        let (left, right) = line.split_once('|').unwrap();

        let left = left.split(' ').filter_map(|x| x.parse().ok()).collect();
        let right = right.split(' ').filter_map(|x| x.parse().ok()).collect();
        o.push((left, right));
    }
    o
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let input = parse(input);
    let mut sum = 0;
    for (l, r) in input {
        let a = r.iter().filter(|x| l.contains(x)).count();

        if a > 0 {
            sum += 2usize.pow(a as u32 - 1);
        }
    }

    Ok(sum)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let input = parse(input);
    let mut instances = BTreeMap::new();
    for i in 0..input.len() {
        instances.insert(i, 1);
    }

    for (i, (l, r)) in input.iter().enumerate() {
        let n = *instances.get(&i).unwrap();
        let win = r.iter().filter(|x| l.contains(x)).count();

        if win > 0 {
            for j in i + 1..=i + win {
                if let Some(v) = instances.get_mut(&j) {
                    *v += n;
                }
            }
        }
    }

    Ok(instances.into_values().sum())
}
