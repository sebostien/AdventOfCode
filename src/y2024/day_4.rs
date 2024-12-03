use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<i64, i64> {
    Solution {
        part_1,
        part_2,
        answer: (0, 0),
    }
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    Err(anyhow!("Not Done!"))
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    Err(anyhow!("Not Done!"))
}

#[test]
fn aaa() {
    let input = r#"

    "#;
    let sol = part_1(input).unwrap();

    assert_eq!(sol, 0000000);
}

#[test]
fn bbb() {
    let input = r#"


    "#;
    let sol = part_2(input).unwrap();

    assert_eq!(sol, 00000000);
}
