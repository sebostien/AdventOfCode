use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        date: (2022, 4),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (526, 886),
    }
}
type Range = (i32, i32);

fn parse_row(row: &str) -> (Range, Range) {
    let mut pairs = row.split(',');
    let mut a = pairs.next().unwrap().split('-');
    let mut b = pairs.next().unwrap().split('-');

    (
        (
            a.next().unwrap().parse().unwrap(),
            a.next().unwrap().parse().unwrap(),
        ),
        (
            b.next().unwrap().parse().unwrap(),
            b.next().unwrap().parse().unwrap(),
        ),
    )
}

fn contained(a: Range, b: Range) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn part_1(input: &str) -> Result<usize, String> {
    let mut sum = 0;

    for line in input.trim().lines() {
        let (a, b) = parse_row(line);

        if contained(a, b) || contained(b, a) {
            sum += 1;
        }
    }

    Ok(sum)
}

fn intersects(a: Range, b: Range) -> bool {
    b.0 <= a.1 && b.1 >= a.0
}

fn part_2(input: &str) -> Result<usize, String> {
    let mut sum = 0;

    for line in input.trim().lines() {
        let (a, b) = parse_row(line);

        if intersects(a, b) {
            sum += 1;
        }
    }

    Ok(sum)
}
