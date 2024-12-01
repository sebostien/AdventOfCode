use crate::Solution;

pub fn get_solution() -> Solution<String, String> {
    Solution {
        part_1,
        part_2,
        answer: ("SHQWSRBDL".to_string(), "CDTQZHBRS".to_string()),
    }
}

#[derive(Clone, Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = vec![];

    for row in input.trim().lines() {
        let nums = row.trim().split(' ').collect::<Vec<_>>();
        moves.push(Move {
            from: nums[3].parse().unwrap(),
            to: nums[5].parse().unwrap(),
            amount: nums[1].parse().unwrap(),
        });
    }

    moves
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = vec![vec![]; 9];

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
        .map(|r| r.iter().copied().rev().collect())
        .collect()
}

fn part_1(input: &str) -> anyhow::Result<String> {
    let mut input1 = input.split("\n\n");
    let mut stacks = parse_stacks(input1.next().unwrap());
    let moves = parse_moves(input1.next().unwrap());

    for m in moves {
        let len = stacks[m.from - 1].len();
        let mut rest = stacks[m.from - 1]
            .split_off(len - m.amount)
            .iter()
            .rev()
            .copied()
            .collect();
        stacks[m.to - 1].append(&mut rest);
    }

    Ok(stacks.iter().map(|r| r.last().unwrap()).collect::<String>())
}

fn part_2(input: &str) -> anyhow::Result<String> {
    let mut input1 = input.split("\n\n");
    let mut stacks = parse_stacks(input1.next().unwrap());
    let moves = parse_moves(input1.next().unwrap());

    for m in moves {
        let len = stacks[m.from - 1].len();
        let mut rest = stacks[m.from - 1].split_off(len - m.amount);
        stacks[m.to - 1].append(&mut rest);
    }

    Ok(stacks.iter().map(|r| r.last().unwrap()).collect::<String>())
}
