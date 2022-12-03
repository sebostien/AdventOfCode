use std::collections::HashSet;

fn priority(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - 38
    } else {
        c as usize - 96
    }
}

fn part_1(input: String) -> usize {
    let mut sum = 0;

    for line in input.trim().lines() {
        let chars = line.trim().chars();
        let middle = line.trim().len() / 2;
        let right: HashSet<char> = chars.clone().skip(middle).collect();
        for c in chars.take(middle) {
            if right.contains(&c) {
                sum += priority(c);
                break;
            }
        }
    }

    sum
}

fn part_2(input: String) -> usize {
    let mut sum = 0;

    let sacks = input
        .trim()
        .lines()
        .map(|s| s.trim().chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    for (i, sack) in sacks.iter().enumerate() {
        if i % 3 == 0 {
            for c in sack.iter() {
                if sacks[i + 1].contains(c) && sacks[i + 2].contains(c) {
                    sum += priority(*c);
                    break;
                }
            }
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
        assert_eq!(part_1(input), 7878);
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_2(input), 2760);
    }
}
