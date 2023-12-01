use crate::Solution;

pub fn get_solution() -> Solution<u32, u32> {
    Solution {
        date: (2023, 1),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (54990, 54473),
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

const NUM_NAMES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn to_digit(s: &str) -> Option<(usize, u32)> {
    if let Some(d) = s.chars().next().and_then(|c| c.to_digit(10)) {
        return Some((1, d));
    }

    for (i, name) in NUM_NAMES.iter().enumerate() {
        if s.starts_with(name) {
            return Some((name.len() - 1, i as u32));
        }
    }
    None
}

fn part_2(input: &str) -> anyhow::Result<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let mut first = 10;
            let mut last = 10;

            let mut i = 0;
            while i < line.len() {
                if let Some((l, d)) = to_digit(&line[i..]) {
                    i += l;
                    if first > 9 {
                        first = d;
                    };
                    last = d;
                } else {
                    i += 1;
                }
            }

            first * 10 + last
        })
        .sum();

    Ok(sum)
}
