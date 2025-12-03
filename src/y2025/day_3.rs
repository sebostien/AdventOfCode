use crate::Solution;

pub fn get_solution() -> Solution<i64, i64> {
    Solution {
        part_1,
        part_2,
        answer: (17_321, 171_989_894_144_198),
    }
}

fn parse(bank: &str) -> Vec<i64> {
    bank.trim()
        .as_bytes()
        .iter()
        .map(|x| i64::from(x - b'0'))
        .collect::<Vec<_>>()
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    Ok(input
        .trim()
        .lines()
        .map(parse)
        .map(|bank| {
            let mut m = 0;

            for i in 0..(bank.len() - 1) {
                let a = bank[i];
                for b in &bank[i + 1..] {
                    m = m.max(a * 10 + b);
                }
            }

            m
        })
        .sum())
}

fn find_jolt(nums: &[i64], to_take: usize) -> i64 {
    if to_take == 0 {
        0
    } else {
        let mut j = 0;

        for i in 1..(nums.len() - (to_take - 1)) {
            if nums[i] > nums[j] {
                j = i;
            }
        }

        nums[j] * 10i64.pow((to_take - 1) as u32) + find_jolt(&nums[j + 1..], to_take - 1)
    }
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    Ok(input
        .trim()
        .lines()
        .map(parse)
        .map(|bank| find_jolt(&bank, 12))
        .sum())
}
