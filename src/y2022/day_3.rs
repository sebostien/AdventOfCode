use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (7878, 2760),
    }
}

fn priority(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - 38
    } else {
        c as usize - 96
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut sum = 0;

    for line in input.trim().lines() {
        let chars = line.trim().chars();
        let middle = line.trim().len() / 2;
        let right: Vec<char> = chars.clone().skip(middle).collect();
        for c in chars.take(middle) {
            if right.contains(&c) {
                sum += priority(c);
                break;
            }
        }
    }

    Ok(sum)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut sum = 0;

    let sacks = input
        .trim()
        .lines()
        .map(|s| s.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (i, sack) in sacks.iter().enumerate().step_by(3) {
        for c in sack {
            if sacks[i + 1].contains(c) && sacks[i + 2].contains(c) {
                sum += priority(*c);
                break;
            }
        }
    }

    Ok(sum)
}
