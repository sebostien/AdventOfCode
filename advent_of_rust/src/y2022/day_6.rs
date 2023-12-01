use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        date: (2022, 6),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (1544, 2145),
    }
}
fn unique(s: &str) -> bool {
    for (i, c) in s.chars().enumerate().skip(1) {
        if s[0..i].contains(c) {
            return false;
        }
    }

    true
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    for i in 0..input.len() - 4 {
        if unique(&input[i..i + 4]) {
            return Ok(i + 4);
        }
    }

    Err(anyhow!("No solution found"))
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    for i in 0..input.len() - 14 {
        if unique(&input[i..i + 14]) {
            return Ok(i + 14);
        }
    }

    Err(anyhow!("No solution found"))
}
