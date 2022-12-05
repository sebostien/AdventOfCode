#[derive(Clone, Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = vec![];

    for row in input.trim().split("\n") {
        let nums = row.trim().split(" ").collect::<Vec<_>>();
        moves.push(Move {
            from: nums[3].parse().unwrap(),
            to: nums[5].parse().unwrap(),
            amount: nums[1].parse().unwrap(),
        });
    }

    moves
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = vec![0; 9].iter().map(|_| vec![]).collect::<Vec<_>>();

    for line in input.lines() {
        let l = line.chars().collect::<Vec<_>>();
        let mut j = 0;
        while j * 4 + 1 < line.len() {
            let c = l[j * 4 + 1];
            if c == '1' {
                break;
            }
            if c != ' ' {
                stacks[j].push(c);
            }
            j += 1;
        }
    }

    stacks
        .iter()
        .map(|r| r.iter().map(|c| c.to_owned()).rev().collect())
        .collect()
}

fn part_1(input: String) -> String {
    let mut input1 = input.split("\n\n");
    let mut stacks = parse_stacks(input1.next().unwrap());
    let moves = parse_moves(input1.next().unwrap());

    for m in moves {
        let len = stacks[m.from - 1].len();
        let mut rest = stacks[m.from - 1]
            .split_off(len - m.amount)
            .iter()
            .rev()
            .map(|c| c.to_owned())
            .collect();
        stacks[m.to - 1].append(&mut rest);
    }

    stacks.iter().map(|r| r.last().unwrap()).collect::<String>()
}

fn part_2(input: String) -> String {
    let mut input1 = input.split("\n\n");
    let mut stacks = parse_stacks(input1.next().unwrap());
    let moves = parse_moves(input1.next().unwrap());

    for m in moves {
        let len = stacks[m.from - 1].len();
        let mut rest = stacks[m.from - 1].split_off(len - m.amount);
        stacks[m.to - 1].append(&mut rest);
    }

    stacks.iter().map(|r| r.last().unwrap()).collect::<String>()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::{get_input_contents, get_year_day};

    #[test]
    fn test_part_1() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_1(input), "SHQWSRBDL");
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_2(input), "CDTQZHBRS");
    }
}
