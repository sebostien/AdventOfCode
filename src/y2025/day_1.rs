use crate::Solution;

pub fn get_solution() -> Solution<usize, isize> {
    Solution {
        part_1,
        part_2,
        answer: (1043, 5963),
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut cur = 50;
    let mut pass = 0;

    for rot in input.trim().lines() {
        let num: isize = rot[1..].parse().unwrap();

        match &rot[0..1] {
            "R" => cur += num,
            "L" => cur -= num,
            _ => unreachable!(),
        }

        cur = cur % 100;
        pass += (cur == 0) as usize;
    }

    Ok(pass)
}

fn part_2(input: &str) -> anyhow::Result<isize> {
    let mut cur = 50;
    let mut pass = 0;

    for rot in input.trim().lines() {
        let num: isize = rot[1..].parse().unwrap();

        match &rot[0..1] {
            "R" => {
                cur += num;
                pass += cur / 100;
                cur = cur % 100;
            }
            "L" => {
                pass += ((100 - cur) % 100 + num) / 100;
                cur = ((cur - num) % 100 + 100) % 100;
            }
            _ => unreachable!(),
        }
    }

    Ok(pass)
}
