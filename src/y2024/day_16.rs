use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::ops::Index;

use crate::{util::Vector2, Solution};

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (65_436, 489),
    }
}

type V2 = Vector2<usize>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Dir {
    North,
    East,
    West,
    South,
}

impl Dir {
    fn turn(self) -> [Dir; 2] {
        match self {
            Dir::North => [Dir::East, Dir::West],
            Dir::South => [Dir::East, Dir::West],
            Dir::East => [Dir::North, Dir::South],
            Dir::West => [Dir::North, Dir::South],
        }
    }

    pub fn step(self, v: V2) -> V2 {
        match self {
            Dir::North => V2::new(v.x, v.y - 1),
            Dir::South => V2::new(v.x, v.y + 1),
            Dir::West => V2::new(v.x - 1, v.y),
            Dir::East => V2::new(v.x + 1, v.y),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    weight: usize,
    pos: V2,
    dir: Dir,
    dw: usize,
}

impl Move {
    fn new(weight: usize, pos: V2, dir: Dir, end: V2) -> Self {
        let dx = pos.x.abs_diff(end.x);
        let dy = pos.y.abs_diff(end.y);
        let mut dt = 0;
        match dir {
            Dir::North => {
                if pos.y < end.y {
                    dt += 2000;
                }
                if pos.x != end.x {
                    dt += 1000;
                }
            }
            Dir::South => {
                if pos.y > end.y {
                    dt += 2000;
                }
                if pos.x != end.x {
                    dt += 1000;
                }
            }
            Dir::East => {
                if pos.x > end.x {
                    dt += 2000;
                }
                if pos.y != end.y {
                    dt += 1000;
                }
            }
            Dir::West => {
                if pos.x < end.x {
                    dt += 2000;
                }
                if pos.y != end.y {
                    dt += 1000;
                }
            }
        }

        Self {
            weight,
            pos,
            dir,
            dw: weight + dx + dy + dt,
        }
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.dw.cmp(&other.dw) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.weight.cmp(&other.weight) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.pos.x.cmp(&other.pos.x) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.pos.y.cmp(&other.pos.y) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.dir.cmp(&other.dir)
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Move2 {
    weight: usize,
    pos: V2,
    dir: Dir,
    dw: usize,
    tiles: Vec<V2>,
}

impl Move2 {
    fn new(weight: usize, pos: V2, dir: Dir, end: V2, mut tiles: Vec<V2>) -> Self {
        let dw = pos.x.abs_diff(end.x)
            + pos.y.abs_diff(end.y)
            + (pos.y != end.y).then_some(1000).unwrap_or(0)
            + (pos.x != end.x).then_some(1000).unwrap_or(0);

        tiles.push(pos);
        Self {
            weight,
            pos,
            dir,
            dw,
            tiles,
        }
    }
}

impl Ord for Move2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.dw.cmp(&other.dw) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.weight.cmp(&other.weight) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.pos.x.cmp(&other.pos.x) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.pos.y.cmp(&other.pos.y) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.dir.cmp(&other.dir)
    }
}

impl PartialOrd for Move2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<bool>>,
    start: V2,
    end: V2,
}

impl Index<V2> for Map {
    type Output = bool;

    fn index(&self, index: V2) -> &Self::Output {
        &self.map[index.y][index.x]
    }
}

impl Map {
    fn new(input: &str) -> Self {
        let mut start = None;
        let mut end = None;

        let mut map = Vec::new();
        for (y, s_line) in input.trim().lines().enumerate() {
            let mut line = Vec::new();
            for (x, c) in s_line.trim().chars().enumerate() {
                line.push(c != '#');
                if c == 'S' {
                    start = Some(V2::new(x, y));
                } else if c == 'E' {
                    end = Some(V2::new(x, y));
                }
            }
            map.push(line);
        }

        Self {
            map,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn find(&self) -> usize {
        let mut to_visit = BTreeSet::new();
        to_visit.insert(Move::new(0, self.start, Dir::East, self.end));
        let mut visited = HashMap::new();

        let mut min = usize::MAX;

        while let Some(Move {
            weight,
            pos,
            dir,
            dw: _,
        }) = to_visit.pop_first()
        {
            if weight > min {
                break;
            }

            if pos == self.end {
                min = min.min(weight);
            }

            let new_pos = dir.step(pos);
            if self[new_pos] {
                let new_w = weight + 1;
                if new_w < min {
                    if let Some(&pw) = visited.get(&(new_pos, dir)) {
                        if new_w >= pw {
                            continue;
                        }
                    }

                    visited.insert((new_pos, dir), new_w);
                    to_visit.insert(Move::new(new_w, new_pos, dir, self.end));
                }
            }

            for new_dir in dir.turn() {
                let new_w = weight + 1000;
                if new_w < min {
                    if let Some(&pw) = visited.get(&(pos, new_dir)) {
                        if new_w >= pw {
                            continue;
                        }
                    }

                    visited.insert((pos, new_dir), new_w);
                    to_visit.insert(Move::new(new_w, pos, new_dir, self.end));
                }
            }
        }

        min
    }

    fn find2(&self) -> usize {
        let mut to_visit = BTreeSet::new();
        to_visit.insert(Move2::new(0, self.start, Dir::East, self.end, Vec::new()));
        let mut visited = HashMap::new();

        let mut best = HashSet::new();
        let mut min = self.find();

        while let Some(Move2 {
            weight,
            pos,
            dir,
            dw: _,
            tiles,
        }) = to_visit.pop_first()
        {
            if weight > min {
                break;
            }
            if pos == self.end {
                if weight <= min {
                    for t in &tiles {
                        best.insert(*t);
                    }
                }
                min = min.min(weight);
            }

            let new_pos = dir.step(pos);
            if self[new_pos] {
                let new_w = weight + 1;
                if new_w <= min {
                    if let Some(&pw) = visited.get(&(new_pos, dir)) {
                        if new_w > pw {
                            continue;
                        }
                    }
                    visited.insert((new_pos, dir), new_w);

                    to_visit.insert(Move2::new(new_w, new_pos, dir, self.end, tiles.clone()));
                }
            }

            for new_dir in dir.turn() {
                let new_w = weight + 1000;
                if new_w <= min {
                    if let Some(&pw) = visited.get(&(pos, new_dir)) {
                        if new_w > pw {
                            continue;
                        }
                    }

                    visited.insert((pos, new_dir), new_w);
                    to_visit.insert(Move2::new(new_w, pos, new_dir, self.end, tiles.clone()));
                }
            }
        }

        best.len()
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let map = Map::new(input.trim());
    Ok(map.find())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let map = Map::new(input.trim());
    Ok(map.find2())
}

#[test]
fn bbbb() {
    let input = r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
    "#;
    assert_eq!(7036, part_1(input).unwrap());
    assert_eq!(45, part_2(input).unwrap());
}

#[test]
fn bbb() {
    let input = r#"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
    "#;
    assert_eq!(11048, part_1(input).unwrap());
    assert_eq!(64, part_2(input).unwrap());
}

#[test]
fn bbbbb() {
    let input = r#"
#######
#.....#
#S...E#
#######
    "#;
    assert_eq!(4, part_1(input).unwrap());
    assert_eq!(5, part_2(input).unwrap());
}
