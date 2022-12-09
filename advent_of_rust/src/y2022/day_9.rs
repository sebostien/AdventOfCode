use std::collections::HashSet;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl std::str::FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(format!("Unkown direction: '{}'", s)),
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

    for m in moves.iter() {
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

fn parse_input(input: String) -> Vec<Move> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut x = line.split(' ');
            Move {
                dir: x.next().unwrap().parse().unwrap(),
                n: x.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn part_1(input: String) -> usize {
    let moves = parse_input(input);
    let all = tail_from_moves(&moves);
    let a: HashSet<(isize, isize)> = HashSet::from_iter(all);
    a.len()
}

fn tail_from_tail(moves: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let mut tail = (0, 0);
    let mut new_moves = Vec::new();

    for head in moves.iter() {
        if (tail.0 - head.0).abs() > 1 || (tail.1 - head.1).abs() > 1 {
            tail.0 += (head.0 - tail.0).signum();
            tail.1 += (head.1 - tail.1).signum();
        }
        new_moves.push(tail);
    }

    new_moves
}

fn part_2(input: String) -> usize {
    let moves = parse_input(input);
    let mut curr = tail_from_moves(&moves);
    for _ in 0..8 {
        curr = tail_from_tail(&curr);
    }

    let a: HashSet<(isize, isize)> = HashSet::from_iter(curr);
    a.len()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::{get_input_contents, get_year_day};

    #[test]
    fn test_part_1() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_1(input), 6332);
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_2(input), 2511);
    }
}
