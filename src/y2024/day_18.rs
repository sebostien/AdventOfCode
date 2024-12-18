use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::Solution;

pub fn get_solution() -> Solution<usize, String> {
    Solution {
        part_1: part_1::<71, 1024>,
        part_2: part_2::<71>,
        answer: (310, "16,46".to_string()),
    }
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .map(|c| {
            let (x, y) = c.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Tile {
    Visited(usize),
    Unvisited,
    Blocked,
}

fn solve<const W: usize>(map: &mut [Vec<Tile>]) -> usize {
    let mut to_visit = vec![((0, 0), 0)];

    while let Some((pos, w)) = to_visit.pop() {
        match map[pos.1][pos.0] {
            Tile::Visited(prev) => map[pos.1][pos.0] = Tile::Visited(prev.min(w)),
            Tile::Unvisited => map[pos.1][pos.0] = Tile::Visited(w),
            Tile::Blocked => unreachable!(),
        }

        if pos.0 == W - 1 && pos.1 == W - 1 {
            continue;
        }

        let next = [
            (pos.0.saturating_sub(1), pos.1),
            (pos.0, pos.1.saturating_sub(1)),
            ((pos.0 + 1).min(W - 1), pos.1),
            (pos.0, (pos.1 + 1).min(W - 1)),
        ];

        for new_pos in next {
            match map[new_pos.1][new_pos.0] {
                Tile::Visited(prev) if prev > w + 1 => {
                    map[new_pos.1][new_pos.0] = Tile::Visited(w + 1);
                    to_visit.push((new_pos, w + 1));
                }
                Tile::Unvisited => {
                    map[new_pos.1][new_pos.0] = Tile::Visited(w + 1);
                    to_visit.push((new_pos, w + 1));
                }
                _ => {}
            }
        }
    }

    match map[W - 1][W - 1] {
        Tile::Visited(w) => w,
        _ => usize::MAX,
    }
}

fn part_1<const W: usize, const B: usize>(input: &str) -> anyhow::Result<usize> {
    let bytes = parse(input);
    let mut map = vec![vec![Tile::Unvisited; W]; W];

    for byte in &bytes[0..B] {
        map[byte.1][byte.0] = Tile::Blocked;
    }

    Ok(solve::<W>(&mut map))
}

fn part_2<const W: usize>(input: &str) -> anyhow::Result<String> {
    let bytes = parse(input);
    let map = vec![vec![Tile::Unvisited; W]; W];

    Ok((0..bytes.len())
        .into_par_iter()
        .find_first(|&i| {
            let mut map = map.clone();
            for byte in bytes[0..i].iter() {
                map[byte.1][byte.0] = Tile::Blocked;
            }
            let p = solve::<W>(&mut map);

            p == usize::MAX
        })
        .map(|i| {
            let (x, y) = bytes[i - 1];
            format!("{x},{y}")
        })
        .unwrap())
}
