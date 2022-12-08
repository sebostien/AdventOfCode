fn part_1(input: String) -> usize {
    let lines = input.lines();

    let mut max = 0;
    let mut current = 0;
    for line in lines {
        if line.is_empty() {
            max = max.max(current);
            current = 0;
        } else {
            let i = line.parse::<usize>().unwrap();
            current += i;
        }
    }

    max
}

fn part_2(input: String) -> usize {
    let lines = input.lines();

    let mut max = vec![];
    let mut current = 0;
    for line in lines {
        if line.is_empty() {
            max.push(current);
            current = 0;
        } else {
            let i = line.parse::<usize>().unwrap();
            current += i;
        }
    }

    max.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    max[0] + max[1] + max[2]
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::{get_input_contents, get_year_day};

    #[test]
    fn test_part_1() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_1(input), 65912);
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_2(input), 195625);
    }
}
