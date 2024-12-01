use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (1_307_354, 482_500),
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let numbers = input.lines().collect::<Vec<&str>>();

    let mut one_bits: Vec<i32> = vec![0; numbers[0].len()];

    for num in &numbers {
        for (i, c) in num.char_indices() {
            if c == '1' {
                one_bits[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    one_bits
        .iter()
        .map(|one| (*one as f32 / numbers.len() as f32).round() as usize)
        .rev()
        .enumerate()
        .for_each(|(exp, n)| {
            gamma += n * 2_usize.pow(exp as u32);
            epsilon += (1 - n) * 2_usize.pow(exp as u32);
        });

    Ok(gamma * epsilon)
}

fn filter<const FEWER: bool>(nums: &[usize], mut i: usize) -> usize {
    let mut nums = nums.to_vec();

    while nums.len() > 1 {
        let ones: usize = nums.iter().map(|n| usize::from(n & i != 0)).sum();
        nums = nums
            .iter()
            .filter_map(|&x| {
                if (ones >= (nums.len() + 1) / 2) == FEWER {
                    (x & i != 0).then_some(x)
                } else {
                    (x & i == 0).then_some(x)
                }
            })
            .collect::<Vec<_>>();

        i >>= 1;
    }

    nums[0]
}

fn parse_base_2(s: &str) -> usize {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            '0' => 0,
            '1' => 1,
            _ => unreachable!("Invalid input")
        } * 2usize.pow(i as u32))
        .sum()
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let numbers = input.lines().map(parse_base_2).collect::<Vec<usize>>();

    let x = filter::<false>(&numbers, 1 << 11);
    let y = filter::<true>(&numbers, 1 << 11);

    Ok(x * y)
}
