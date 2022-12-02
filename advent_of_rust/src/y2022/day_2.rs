enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn to_score(c: &str) -> usize {
    match c {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,

        _ => panic!("Unkown char {}", c),
    }
}

fn part_1(input: String) -> usize {
    let mut score = 0;

    for line in input.trim().lines() {
        let mut cs = line.trim().split(" ");

        let a = to_score(cs.next().unwrap());
        let x = to_score(cs.next().unwrap());
        score += x;
        if a == x {
            score += 3;
        } else if x == 3 && a == 2 || x == 2 && a == 1 || x == 1 && a == 3 {
            score += 6;
        }
    }

    score
}

fn part_2(input: String) -> usize {
    let mut score = 0;

    for line in input.trim().lines() {
        let mut cs = line.trim().split(" ");

        let a = to_score(cs.next().unwrap());
        let x = to_score(cs.next().unwrap());
        score += (x - 1) * 3;

        match x {
            1 => score += if a - 1 == 0 { 3 } else { a - 1 },
            2 => score += a,
            3 => score += if a + 1 == 4 { 1 } else { a + 1 },
            _ => panic!("Unkown score: {}", x),
        }
    }

    score
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::{get_input_contents, get_year_day};

    #[test]
    fn test_part_1() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_1(input), 8392);
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_2(input), 10116);
    }
}
