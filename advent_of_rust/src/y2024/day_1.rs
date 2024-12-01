use std::collections::HashMap;

use crate::Solution;

pub fn get_solution() -> Solution<u32, u32> {
    Solution {
        date: (2024, 1),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (2970687, 23963899),
    }
}

fn part_1(input: &str) -> anyhow::Result<u32> {
    let (mut ls, mut rs) = input
        .trim()
        .lines()
        .map(|line| {
            let mut chars = line.trim().split("   ");
            let a = chars.next().unwrap().parse::<u32>().unwrap();
            let b = chars.next().unwrap().parse::<u32>().unwrap();
            (a, b)
        })
        .fold((Vec::new(), Vec::new()), |(mut ls, mut rs), (l, r)| {
            ls.push(l);
            rs.push(r);
            (ls, rs)
        });

    ls.sort_unstable();
    rs.sort_unstable();

    Ok(ls.into_iter().zip(rs).map(|(l, r)| l.abs_diff(r)).sum())
}

fn part_2(input: &str) -> anyhow::Result<u32> {
    let (ls, rs) = input
        .trim()
        .lines()
        .map(|line| {
            let mut chars = line.trim().split("   ");
            let a = chars.next().unwrap().parse::<u32>().unwrap();
            let b = chars.next().unwrap().parse::<u32>().unwrap();
            (a, b)
        })
        .fold((Vec::new(), HashMap::new()), |(mut ls, mut rs), (l, r)| {
            ls.push(l);
            *rs.entry(r).or_insert(0) += 1;
            (ls, rs)
        });

    Ok(ls.into_iter().map(|i| rs.get(&i).unwrap_or(&0) * i).sum())
}
