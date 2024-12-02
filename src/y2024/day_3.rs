use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (0, 0),
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let reports = input
        .trim()
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse::<usize>().unwrap()));

    Ok(0)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    Err(anyhow!("Not Done!"))
}

#[test]
fn t_2024_2_1() {
    let input = r#"


    "#;
    let sol = part_1(input).unwrap();

    assert_eq!(sol, 2);
}

#[test]
fn t_2024_2_2() {
    let input = r#"


    "#;
    let sol = part_2(input).unwrap();

    assert_eq!(sol, 4);
}
