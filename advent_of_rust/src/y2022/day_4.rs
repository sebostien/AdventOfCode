fn parse_row(row: &str) -> ((i32, i32), (i32, i32)) {
    let mut pairs = row.split(',');
    let mut a = pairs.next().unwrap().split("-");
    let mut b = pairs.next().unwrap().split("-");

    return (
        (
            a.next().unwrap().parse().unwrap(),
            a.next().unwrap().parse().unwrap(),
        ),
        (
            b.next().unwrap().parse().unwrap(),
            b.next().unwrap().parse().unwrap(),
        ),
    );
}

fn contained(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn part_1(input: String) -> usize {
    let mut sum = 0;

    for line in input.trim().lines() {
        let (a, b) = parse_row(line);

        if contained(a, b) || contained(b, a) {
            sum += 1;
        }
    }

    sum
}

fn intersects(a: (i32, i32), b: (i32, i32)) -> bool {
    b.0 <= a.1 && b.1 >= a.0
}

fn part_2(input: String) -> usize {
    let mut sum = 0;

    for line in input.trim().lines() {
        let (a, b) = parse_row(line);

        if intersects(a, b) {
            sum += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::{get_input_contents, get_year_day};

    #[test]
    fn test_part_1() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_1(input), 526);
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_2(input), 886);
    }
}
