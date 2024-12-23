use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (65_912, 195_625),
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
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

    Ok(max)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
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

    Ok(max[0] + max[1] + max[2])
}
