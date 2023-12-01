use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        date: (2022, 23),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (13, 37),
    }
}

#[derive(Debug)]
struct Ranges {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Ranges {
    fn new() -> Self {
        Self {
            x_min: isize::MAX,
            x_max: isize::MIN,
            y_min: isize::MAX,
            y_max: isize::MIN,
        }
    }

    fn update(&mut self, x: isize, y: isize) {
        self.x_min = self.x_min.min(x);
        self.x_max = self.x_max.max(x);
        self.y_min = self.y_min.min(y);
        self.y_max = self.y_max.max(y);
    }

    fn area(&self) -> usize {
        ((self.x_max - self.x_min + 1) * (self.y_max - self.y_min + 1)) as usize
    }
}

struct Elves {
    r: Ranges,
    elves: HashSet<(isize, isize)>,
}

// TODO: Remove
impl Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = vec![vec!['.'; 20]; 20];

        for &(x, y) in self.elves.iter() {
            rows[(y + 5) as usize][(x + 5) as usize] = '#';
        }

        for row in rows {
            writeln!(
                f,
                "{}",
                row.iter().map(|x| x.to_string()).collect::<String>()
            )?;
        }

        Ok(())
    }
}

impl FromStr for Elves {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elves = HashSet::new();
        let mut r = Ranges::new();

        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        let x = x as isize;
                        let y = y as isize;
                        r.update(x, y);
                        elves.insert((x, y));
                    }
                    '.' => {}
                    _ => Err(anyhow!("Unknown char {}", char))?,
                }
            }
        }

        Ok(Elves { elves, r })
    }
}

impl Elves {
    fn get_next_pos(&self, x: isize, y: isize) -> (isize, isize) {
        let adj = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .into_iter()
        .map(|(i, j)| {
            // TODO: Optimization. turn into 2**x so that we can match 1d
            self.elves.contains(&(x + i, y + j))
        })
        .collect::<Vec<_>>();

        //    (NW, N, NE, W, E, SW, S, SE)
        match (
            adj[0], adj[1], adj[2], adj[3], adj[4], adj[5], adj[6], adj[7],
        ) {
            (false, false, false, _, _, _, _, _) => (x, y - 1),
            (_, _, _, _, _, false, false, false) => (x, y + 1),
            (false, _, _, false, _, false, _, _) => (x - 1, y),
            (_, _, false, _, false, _, _, false) => (x + 1, y),
            _ => (x, y),
        }
    }

    fn do_round(&mut self) {
        let mut num_moves = HashMap::new();
        let new_positions = self
            .elves
            .iter()
            .cloned()
            .map(|row| (row, self.get_next_pos(row.0, row.1)))
            .collect::<Vec<_>>();

        for (_, pos) in new_positions.iter() {
            if let Some(v) = num_moves.get_mut(&pos) {
                *v += 1;
            } else {
                num_moves.insert(pos, 1);
            }
        }

        // TODO: Optimization maybe? Calculate range after instead
        let mut new_ranges = Ranges::new();

        for (prev, new) in new_positions.iter() {
            if num_moves.get(&new) == Some(&1) {
                new_ranges.update(new.0, new.1);
                self.elves.remove(prev);
                self.elves.insert(*new);
            }
        }
        self.r = new_ranges;
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut elves: Elves = input.parse()?;
    for _ in 0..10 {
        println!("{elves}");
        elves.do_round();
    }
    println!("{elves}");
    let r = elves.r;
    println!("{r:#?}");
    Ok(r.area() - elves.elves.len())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    Err(anyhow!("Not done!"))
}

#[cfg(test)]
mod tests {
    use crate::y2022::day_23::part_1;

    #[test]
    fn ttt() {
        let input = r#".....
..##.
..#..
.....
..##.
....."#;

        assert_eq!(110, part_1(input).unwrap());
    }
}
