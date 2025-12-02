use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (19605500130, 36862281418),
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut num: usize = 0;

    let ranges = input.trim().split(',');
    for range in ranges {
        let (a, b) = range.split_once('-').unwrap();
        let a: usize = a.parse()?;
        let b: usize = b.parse()?;

        for id in a..=b {
            let s: &str = &id.to_string();
            let half = s.len() / 2;
            let a = &s[0..half];
            let b = &s[half..];

            if s.len() % 2 == 0 && !b.starts_with('0') && a == b {
                num = num.checked_add(id).unwrap();
            }
        }
    }

    Ok(num)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut num: usize = 0;

    let ranges = input.trim().split(',');
    for range in ranges {
        let (a, b) = range.split_once('-').unwrap();
        let a: usize = a.parse()?;
        let b: usize = b.parse()?;

        for id in a..=b {
            let s: &str = &id.to_string();

            for i in 1..=(s.len() / 2) {
                if s[0..i].repeat(s.len() / i) == s {
                    num += id;
                    break;
                }
            }
        }
    }

    Ok(num)
}
