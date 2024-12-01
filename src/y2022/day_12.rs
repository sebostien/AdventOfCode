use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<u16, u16> {
    Solution {
        part_1,
        part_2,
        answer: (425, 418),
    }
}

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Clone)]
struct Map {
    m: HashMap<(i8, i8), u16>,
    start: (i8, i8),
    end: (i8, i8),
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let mut map = HashMap::new();

        for (i, line) in s.trim().lines().enumerate() {
            for (j, c) in line.trim().chars().enumerate() {
                let pos = (j as i8, i as i8);
                if let Some(h) = ALPHA.find(c) {
                    map.insert(pos, h as u16);
                } else {
                    let h = match c {
                        'S' => {
                            start = Some(pos);
                            ALPHA.find('a').unwrap()
                        }
                        'E' => {
                            end = Some(pos);
                            ALPHA.find('z').unwrap()
                        }
                        _ => Err(anyhow!("Unkown char in input: {c}"))?,
                    };
                    map.insert(pos, h as u16);
                }
            }
        }

        if start.is_none() {
            Err(anyhow!("Could not find start!"))?;
        }

        if end.is_none() {
            Err(anyhow!("Could not find end!"))?;
        }

        Ok(Self {
            m: map,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

fn square_height(steps: u16, square: u16, reset: bool) -> u16 {
    if reset && square == 0 {
        0
    } else {
        steps
    }
}

fn flood_fill(start: (i8, i8), map: &Map, reset: bool) -> anyhow::Result<u16> {
    let mut to_visit = HashMap::new();
    to_visit.insert(start, 0);
    let mut visited: HashMap<(i8, i8), u16> = HashMap::new();

    while let Some((&pos, &steps)) = to_visit.iter().next() {
        to_visit.remove(&pos);
        if let Some(&prev) = visited.get(&pos) {
            if prev <= steps {
                continue;
            }
        }
        visited.insert(pos, steps);

        if pos == map.end {
            continue;
        }

        let ch = map.m.get(&pos).unwrap();
        for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if let Some(&height) = map.m.get(&new_pos) {
                let new_steps = square_height(steps + 1, height, reset);
                if ch + 1 >= height {
                    let &prev_v = visited.get(&new_pos).unwrap_or(&u16::MAX);
                    let &prev_t = to_visit.get(&new_pos).unwrap_or(&u16::MAX);
                    if new_steps < prev_v && new_steps < prev_t {
                        to_visit.insert(new_pos, new_steps);
                    }
                }
            }
        }
    }

    visited
        .get(&map.end)
        .copied()
        .ok_or(anyhow!("Could not reach end!"))
}

fn part_1(input: &str) -> anyhow::Result<u16> {
    let map: Map = input.parse()?;
    flood_fill(map.start, &map, false)
}

fn part_2(input: &str) -> anyhow::Result<u16> {
    let map: Map = input.parse()?;
    flood_fill(map.start, &map, true)
}
