use std::collections::HashSet;

use anyhow::anyhow;

pub fn get_solution() -> crate::Solution<usize, usize> {
    crate::Solution {
        date: (2023, 10),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (6886, 0),
    }
}

struct Maze(Vec<Vec<u8>>);

impl<S> From<S> for Maze
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let maze = s
            .as_ref()
            .trim()
            .lines()
            .map(|line| line.trim().chars().map(|c| c as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self(maze)
    }
}

impl std::ops::Index<(usize, usize)> for Maze {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[y][x]
    }
}

impl Maze {
    fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        match self[(x, y)] {
            b'|' => vec![(x, y - 1), (x, y + 1)],
            b'-' => vec![(x - 1, y), (x + 1, y)],
            b'J' => vec![(x, y - 1), (x - 1, y)],
            b'L' => vec![(x, y - 1), (x + 1, y)],
            b'7' => vec![(x, y + 1), (x - 1, y)],
            b'F' => vec![(x, y + 1), (x + 1, y)],
            b'S' => {
                let mut n = vec![];
                if y > 0 && matches!(self[(x, y - 1)], b'|' | b'F' | b'7') {
                    n.push((x, y - 1));
                }
                if y < self.0.len() - 1 && matches!(self[(x, y + 1)], b'|' | b'L' | b'J') {
                    n.push((x, y + 1));
                }
                if x > 0 && matches!(self[(x - 1, y)], b'-' | b'L' | b'F') {
                    n.push((x - 1, y));
                }
                if x < self.0[y].len() - 1 && matches!(self[(x + 1, y)], b'-' | b'J' | b'7') {
                    n.push((x + 1, y));
                }
                n
            }
            _ => vec![],
        }
    }

    fn starts(&self) -> Vec<(usize, usize)> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, pipe) in row.iter().enumerate() {
                if *pipe == b'S' {
                    return self.neighbours((x, y));
                }
            }
        }
        unreachable!()
    }

    fn longest_path(&self) -> usize {
        let mut to_visit = self
            .starts()
            .into_iter()
            .map(|pos| (1, pos))
            .take(1)
            .collect::<Vec<_>>();

        let mut max = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::from_iter([to_visit[0].1]);

        while let Some((p_len, pos)) = to_visit.pop() {
            for neigh in self.neighbours(pos) {
                if self[neigh] == b'S' {
                    max = max.max(p_len + 1);
                } else if visited.insert(neigh) {
                    to_visit.push((p_len + 1, neigh));
                }
            }
        }
        max / 2
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    Ok(Maze::from(input).longest_path())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    Err(anyhow!("Not Done!"))
}

#[test]
fn day11() {
    let input = r#"
S------7
|F----7|
||....||
||....||
|L-7F-J|
|..||..|
L--JL--J
    "#;
    assert_eq!(4, part_2(input).unwrap());
}
