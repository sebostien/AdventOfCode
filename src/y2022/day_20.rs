use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (8392, 10116),
    }
}

fn index_wrapped(i: isize, len: usize) -> usize {
    // TODO: Use mod
    if i < 0 {
        index_wrapped(i + len as isize, len)
    } else if i >= len as isize {
        index_wrapped(i - len as isize, len)
    } else {
        i as usize
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    // let mut nums = input
    //     .lines()
    //     .map(|x| x.parse::<isize>().unwrap())
    //     .collect::<Vec<_>>();
    // let len = nums.len();
    //
    // let order = vec![0; len]
    //     .iter()
    //     .enumerate()
    //     .map(|(i, _)| i)
    //     .collect::<Vec<_>>();
    //
    // for i in order {
    //     let n = nums[i];
    //     let j = index_wrapped(i as isize + n, len);
    //     println!("Insert {n} at pos {j} from {i}");
    //     match i.cmp(&j) {
    //         Ordering::Less => {
    //             nums.insert(j, n);
    //             nums.remove(i);
    //         }
    //         Ordering::Greater => {
    //             nums.remove(i);
    //             nums.insert(j, n);
    //         }
    //         Ordering::Equal => {}
    //     }
    // }

    Err(anyhow!("Not Done!"))
}

fn part_2(_input: &str) -> anyhow::Result<usize> {
    Err(anyhow!("Not Done!"))
}
