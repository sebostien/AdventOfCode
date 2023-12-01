use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        date: (2022, 2),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (8392, 10116),
    }
}

fn to_score(c: &str) -> anyhow::Result<usize> {
    Ok(match c {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => Err(anyhow!("Unkown char {c}"))?,
    })
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut score = 0;

    for line in input.trim().lines() {
        let mut cs = line.trim().split(' ');

        let a = to_score(cs.next().unwrap())?;
        let x = to_score(cs.next().unwrap())?;
        score += x;
        if a == x {
            score += 3;
        } else if x == 3 && a == 2 || x == 2 && a == 1 || x == 1 && a == 3 {
            score += 6;
        }
    }

    Ok(score)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut score = 0;

    for line in input.trim().lines() {
        let mut cs = line.trim().split(' ');

        let a = to_score(cs.next().unwrap())?;
        let x = to_score(cs.next().unwrap())?;
        score += (x - 1) * 3;

        match x {
            1 => score += if a - 1 == 0 { 3 } else { a - 1 },
            2 => score += a,
            3 => score += if a + 1 == 4 { 1 } else { a + 1 },
            _ => Err(anyhow!("Unkown score: {}", x))?,
        }
    }

    Ok(score)
}
