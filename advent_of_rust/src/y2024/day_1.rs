use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<u32, u32> {
    Solution {
        date: (2024, 1),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (0, 0),
    }
}

fn part_1(input: &str) -> anyhow::Result<u32> {
    Ok(input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter_map(|c| c.to_digit(10));
            let a = chars.next().unwrap();
            let b = chars.last().unwrap_or(a);
            a * 10 + b
        })
        .sum())
}

fn part_2(input: &str) -> anyhow::Result<u32> {
    Err(anyhow!("Not Done!"))
}
