use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        date: (2022, 14),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (0, 0),
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let lines = input.lines();

    let mut max = 0;
    let mut current = 0;
    for line in lines {
        if line.is_empty() {
            max = max.max(current);
            current = 0;
        } else {
            let i = line.parse::<usize>().unwrap();
            current += i;
        }
    }

    Ok(max)
}

fn part_2(_input: &str) -> anyhow::Result<usize> {
    Err(anyhow!("Not Done!"))
}
