use std::collections::{HashSet, VecDeque};

use anyhow::anyhow;

use crate::util::Vector2;

pub fn get_solution() -> crate::Solution<usize, usize> {
    crate::Solution {
        part_1,
        part_2,
        answer: (6886, 0),
    }
}

type V2 = Vector2<usize>;

struct Maze {
    maze: Vec<Vec<u8>>,
}

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

        Self { maze }
    }
}

impl std::ops::Index<V2> for Maze {
    type Output = u8;

    fn index(&self, v: V2) -> &Self::Output {
        &self.maze[v.y][v.x]
    }
}

impl Maze {
    fn neighbours(&self, v: V2) -> Vec<V2> {
        let V2 { x, y } = v;

        match self[v] {
            b'|' => vec![V2::new(x, y - 1), V2::new(x, y + 1)],
            b'-' => vec![V2::new(x - 1, y), V2::new(x + 1, y)],
            b'J' => vec![V2::new(x, y - 1), V2::new(x - 1, y)],
            b'L' => vec![V2::new(x, y - 1), V2::new(x + 1, y)],
            b'7' => vec![V2::new(x, y + 1), V2::new(x - 1, y)],
            b'F' => vec![V2::new(x, y + 1), V2::new(x + 1, y)],
            b'S' => {
                let mut n = vec![];
                if y > 0 && matches!(self[V2::new(x, y - 1)], b'|' | b'F' | b'7') {
                    n.push(V2::new(x, y - 1));
                }
                if y < self.maze.len() - 1 && matches!(self[V2::new(x, y + 1)], b'|' | b'L' | b'J')
                {
                    n.push(V2::new(x, y + 1));
                }
                if x > 0 && matches!(self[V2::new(x - 1, y)], b'-' | b'L' | b'F') {
                    n.push(V2::new(x - 1, y));
                }
                if x < self.maze[y].len() - 1
                    && matches!(self[V2::new(x + 1, y)], b'-' | b'J' | b'7')
                {
                    n.push(V2::new(x + 1, y));
                }
                n
            }
            _ => vec![],
        }
    }

    fn start(&self) -> V2 {
        for (y, row) in self.maze.iter().enumerate() {
            for (x, pipe) in row.iter().enumerate() {
                if *pipe == b'S' {
                    return V2::new(x, y);
                }
            }
        }
        unreachable!()
    }

    fn longest_path(&mut self) -> usize {
        let mut to_visit: VecDeque<(usize, V2)> = VecDeque::from([(0, self.start())]);

        let mut max = 0;
        let mut visited: HashSet<V2> = HashSet::new();

        while let Some((p_len, pos)) = to_visit.pop_front() {
            for neigh in self.neighbours(pos) {
                max = max.max(p_len);

                if visited.insert(neigh) {
                    to_visit.push_back((p_len + 1, neigh));
                }
            }
        }
        max
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    Ok(Maze::from(input).longest_path())
}

impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.maze {
            for c in row {
                write!(f, "{}", *c as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    Err(anyhow!("Not Done!"))
}
