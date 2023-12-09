pub fn get_solution() -> crate::Solution<i64, i64> {
    crate::Solution {
        date: (2023, 10),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (0, 0),
    }
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    Ok(input.len())
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    Ok(input.len())
}

#[test]
fn day10() {
    let input = r#"

    "#
    .trim();

    assert_eq!(0, part_1(input).unwrap());
    assert_eq!(0, part_2(input).unwrap());
}
