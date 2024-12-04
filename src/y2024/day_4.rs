use crate::Solution;

pub fn get_solution() -> Solution<i64, usize> {
    Solution {
        part_1,
        part_2,
        answer: (2578, 1972),
    }
}

struct Grid {
    grid: Vec<Vec<u8>>,
    rows: isize,
    cols: isize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .trim()
            .lines()
            .map(|line| line.trim().as_bytes().to_vec())
            .collect::<Vec<_>>();

        Self {
            rows: grid.len() as isize,
            cols: grid[0].len() as isize,
            grid,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        self.grid
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
    }

    fn dirs() -> [(isize, isize); 8] {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
    }
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    let grid = Grid::new(input);
    let mut s = 0;
    for y in 0..grid.rows {
        for x in 0..grid.cols {
            if grid.get(x, y) == Some(b'X') {
                for (dx, dy) in Grid::dirs() {
                    let (x, y) = (x + dx, y + dy);
                    if grid.get(x, y) == Some(b'M') {
                        let (x, y) = (x + dx, y + dy);
                        if grid.get(x, y) == Some(b'A') {
                            let (x, y) = (x + dx, y + dy);
                            if grid.get(x, y) == Some(b'S') {
                                s += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(s)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let grid = Grid::new(input);
    let mut s = 0;

    for y in 1..grid.rows - 1 {
        for x in 1..grid.cols - 1 {
            if grid.get(x, y) != Some(b'A') {
                continue;
            }

            let lt = grid.get(x - 1, y - 1);
            let rt = grid.get(x + 1, y - 1);
            let lb = grid.get(x - 1, y + 1);
            let rb = grid.get(x + 1, y + 1);

            let left = lt == Some(b'M') && rb == Some(b'S') || lt == Some(b'S') && rb == Some(b'M');
            let right =
                rt == Some(b'M') && lb == Some(b'S') || rt == Some(b'S') && lb == Some(b'M');
            s += (left && right) as usize;
        }
    }

    Ok(s)
}
