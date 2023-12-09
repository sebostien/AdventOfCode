pub fn get_solution() -> crate::Solution<i64, i64> {
    crate::Solution {
        date: (2023, 9),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (1_934_898_178, 1129),
    }
}

fn history<const PREV: bool>(row: &[i64]) -> i64 {
    if row.iter().all(|&x| x == 0) {
        return 0;
    }

    let row_below = row.windows(2).map(|ab| ab[1] - ab[0]).collect::<Vec<_>>();

    if PREV {
        // Part 2
        row.first().unwrap() - history::<PREV>(&row_below)
    } else {
        row.last().unwrap() + history::<PREV>(&row_below)
    }
}

fn run<const PREV: bool>(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|h| history::<PREV>(&h))
        .sum()
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    Ok(run::<false>(input))
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    Ok(run::<true>(input))
}
