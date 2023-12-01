use anyhow::anyhow;

use crate::Solution;
use std::collections::HashSet;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        date: (2022, 9),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (6332, 2511),
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl std::str::FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(anyhow!("Unkown direction: '{s}'")),
        }
    }
}

struct Move {
    n: usize,
    dir: Dir,
}

fn tail_from_moves(moves: &[Move]) -> Vec<(isize, isize)> {
    let mut head: (isize, isize) = (0, 0);
    let mut tail = (0, 0);
    let mut tail_visit = vec![tail];

    for m in moves {
        for _ in 0..m.n {
            match m.dir {
                Dir::Up => head.1 += 1,
                Dir::Down => head.1 -= 1,
                Dir::Left => head.0 -= 1,
                Dir::Right => head.0 += 1,
            };

            if (tail.0 - head.0).abs() > 1 || (tail.1 - head.1).abs() > 1 {
                tail.0 += (head.0 - tail.0).signum();
                tail.1 += (head.1 - tail.1).signum();
                tail_visit.push(tail);
            }
        }
    }
    tail_visit
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Move>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut x = line.split(' ');
            Ok(Move {
                dir: x.next().unwrap().parse()?,
                n: x.next().unwrap().parse()?,
            })
        })
        .collect()
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let moves = parse_input(input)?;
    let all = tail_from_moves(&moves);
    let a: HashSet<(isize, isize)> = HashSet::from_iter(all);
    Ok(a.len())
}

fn tail_from_tail(moves: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let mut tail = (0, 0);
    let mut new_moves = Vec::new();

    for head in moves {
        if (tail.0 - head.0).abs() > 1 || (tail.1 - head.1).abs() > 1 {
            tail.0 += (head.0 - tail.0).signum();
            tail.1 += (head.1 - tail.1).signum();
        }
        new_moves.push(tail);
    }

    new_moves
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let moves = parse_input(input)?;
    let mut curr = tail_from_moves(&moves);
    for _ in 0..8 {
        curr = tail_from_tail(&curr);
    }

    let a: HashSet<(isize, isize)> = HashSet::from_iter(curr);
    Ok(a.len())
}
