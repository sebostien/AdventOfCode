use std::collections::HashSet;

pub fn get_solution() -> crate::Solution<u64, u64> {
    crate::Solution {
        date: (2023, 11),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (10_292_708, 790_194_712_336),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Empty {
    Row(usize),
    Col(usize),
}

#[derive(Debug, Clone)]
struct Universe {
    galaxies: HashSet<(usize, usize)>,
    empty: HashSet<Empty>,
}

fn parse(input: &str) -> Universe {
    use Empty::{Col, Row};
    let mut galaxies = HashSet::new();
    let mut empty = HashSet::new();
    let mut cols = vec![true; input.trim().chars().position(|c| c == '\n').unwrap()];

    for (y, row) in input.trim().lines().enumerate() {
        let mut all = true;
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                galaxies.insert((x, y));
                all = false;
                cols[x] = false;
            }
        }

        if all {
            empty.insert(Row(y));
        }
    }

    for (i, col) in cols.into_iter().enumerate() {
        if col {
            empty.insert(Col(i));
        }
    }

    Universe { galaxies, empty }
}

fn sum_pairs<const S: u64>(universe: &Universe) -> u64 {
    let mut sum = 0;
    for (x1, y1) in &universe.galaxies {
        for (x2, y2) in &universe.galaxies {
            if x1 != x2 || y1 != y2 {
                let mut empty = 0;
                for x in *x1.min(x2)..=*x1.max(x2) {
                    if universe.empty.contains(&Empty::Col(x)) {
                        empty += S - 1;
                    }
                }
                for y in *y1.min(y2)..=*y1.max(y2) {
                    if universe.empty.contains(&Empty::Row(y)) {
                        empty += S - 1;
                    }
                }
                sum += x1.abs_diff(*x2) as u64 + y1.abs_diff(*y2) as u64 + empty;
            }
        }
    }
    sum / 2
}

fn part_1(input: &str) -> anyhow::Result<u64> {
    let universe = parse(input);
    Ok(sum_pairs::<2>(&universe))
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let universe = parse(input);
    Ok(sum_pairs::<1_000_000>(&universe))
}

// #[test]
// fn day11() {
//     let input = r#"
// ...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....
//     "#;
//     assert_eq!(374, part_1(input).unwrap());
//     assert_eq!(8410, part_2(input).unwrap());
// }
