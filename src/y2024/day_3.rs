use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (174_561_379, 106_921_067),
    }
}

fn bytes_to_num(bytes: &[u8]) -> usize {
    bytes
        .into_iter()
        .copied()
        .map(usize::from)
        .rev()
        .enumerate()
        .map(|(i, t)| 10_usize.pow(i as u32) * t)
        .sum()
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut s = 0;
    let mut a = [0, 0, 0];
    let mut ai = 0;
    let mut b = [0, 0, 0];
    let mut bi = 0;
    let mut state = 0;
    for c in input.bytes() {
        match (state, c) {
            (_, b'm') => {
                ai = 0;
                bi = 0;
                state = 1;
            }
            (1, b'u') => state = 2,
            (2, b'l') => state = 3,
            (3, b'(') => state = 4,
            (4, _) if (b'0'..=b'9').contains(&c) => {
                if ai <= 2 {
                    a[ai] = c - b'0';
                    ai += 1;
                    state = 4;
                } else {
                    state = 0;
                }
            }
            (4, b',') => state = 5,
            (5, _) if (b'0'..=b'9').contains(&c) => {
                if bi <= 2 {
                    b[bi] = c - b'0';
                    bi += 1;
                    state = 5;
                } else {
                    state = 5;
                }
            }
            (5, b')') => {
                state = 0;
                let a = bytes_to_num(&a[0..ai]);
                ai = 0;
                let b = bytes_to_num(&b[0..bi]);
                bi = 0;
                s += a * b;
            }
            (_, _) => {
                state = 0;
                ai = 0;
                bi = 0;
            }
        }
    }

    Ok(s)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let xs = input
        .trim()
        .split("do()")
        .map(|dos| part_1(dos.split_once("don't()").map(|(l, _)| l).unwrap_or(dos)).unwrap())
        .sum();

    Ok(xs)
}
