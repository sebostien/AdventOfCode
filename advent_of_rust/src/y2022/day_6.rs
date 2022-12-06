fn unique(s: &str) -> bool {
    for (i, c) in s.chars().enumerate().skip(1) {
        if s[0..i].contains(c) {
            return false;
        }
    }

    true
}

fn part_1(input: String) -> usize {

    for i in 0..input.len() - 4 {
        if unique(&input[i..i + 4]) {
            return i + 4;
        }
    }

    0
}

fn part_2(input: String) -> usize {
    for i in 0..input.len() - 14 {
        if unique(&input[i..i + 14]) {
            return i + 14;
        }
    }

    0

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::{get_input_contents, get_year_day};

    #[test]
    fn test_part_1() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_1(input), 1544);
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_2(input), 2145);
    }
}
