use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (1553, 1597),
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let lines = input.lines().map(|row| row.parse::<u32>().unwrap());

    let mut prev = u32::MAX;
    let mut inc = 0;
    for curr in lines {
        if curr > prev {
            inc += 1;
        }
        prev = curr;
    }

    Ok(inc)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let lines = input
        .lines()
        .map(|row| row.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut prev: u32 = u32::MAX;
    let mut inc = 0;
    for i in 2..lines.len() {
        let curr = lines[i] + lines[i - 1] + lines[i - 2];
        if curr > prev {
            inc += 1;
        }
        prev = curr;
    }

    Ok(inc)
}
