use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<u32, u32> {
    Solution {
        part_1,
        part_2,
        answer: (0, 0),
    }
}

fn part_1(_input: &str) -> anyhow::Result<u32> {
    Err(anyhow!("Not Done!"))
}

fn part_2(_input: &str) -> anyhow::Result<u32> {
    Err(anyhow!("Not Done!"))
}

#[test]
fn t_2024_2_1() {
    let input = r#"

    "#;
    let sol = part_1(input).unwrap();

    assert_eq!(sol, 0);
}

#[test]
fn t_2024_2_2() {
    let input = r#"

    "#;
    let sol = part_2(input).unwrap();

    assert_eq!(sol, 0);
}
