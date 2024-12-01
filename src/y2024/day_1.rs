use std::collections::HashMap;

use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (2_970_687, 23_963_899),
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let (mut ls, mut rs): (Vec<_>, Vec<_>) = input
        .trim()
        .lines()
        .map(|line| {
            let mut chars = line.trim().split("   ");
            let a = chars.next().unwrap().parse::<usize>().unwrap();
            let b = chars.next().unwrap().parse::<usize>().unwrap();
            (a, b)
        })
        .unzip();

    ls.sort_unstable();
    rs.sort_unstable();

    Ok(ls.into_iter().zip(rs).map(|(l, r)| l.abs_diff(r)).sum())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let (ls, rs): (Vec<_>, Vec<_>) = input
        .trim()
        .lines()
        .map(|line| {
            let mut chars = line.trim().split("   ");
            let a = chars.next().unwrap().parse::<usize>().unwrap();
            let b = chars.next().unwrap().parse::<usize>().unwrap();
            (a, b)
        })
        .unzip();

    let mut count = HashMap::with_capacity(rs.len());
    for k in rs {
        *count.entry(k).or_insert(0) += 1;
    }

    Ok(ls
        .into_iter()
        .map(|i| count.get(&i).unwrap_or(&0) * i)
        .sum())
}
