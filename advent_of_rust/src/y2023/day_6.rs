pub fn get_solution() -> crate::Solution<usize, usize> {
    crate::Solution {
        date: (2023, 6),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (771_628, 27_363_861),
    }
}

fn parse<const SPLIT: bool>(input: &str) -> anyhow::Result<(Vec<usize>, Vec<usize>)> {
    let mut nums = input.lines().map(|l| {
        if SPLIT {
            l.split(' ').filter_map(|l| l.parse().ok()).collect()
        } else {
            vec![l
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap()]
        }
    });

    Ok((nums.next().unwrap(), nums.next().unwrap()))
}

fn wins(times: &[usize], dists: &[usize]) -> usize {
    let mut ways = 1;
    for (time, dist) in times.iter().zip(dists) {
        let mut wins = 0;
        for t in 0..=*time {
            let speed = t;
            let d = (time - t) * speed;
            if d > *dist {
                wins += 1;
            }
        }
        ways *= wins;
    }

    ways
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let (times, dists) = parse::<true>(input)?;
    Ok(wins(&times, &dists))
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let (times, dists) = parse::<false>(input)?;
    Ok(wins(&times, &dists))
}

#[test]
fn day6() {
    let input = r#"
Time:      7  15   30
Distance:  9  40  200
        "#
    .trim();

    assert_eq!(46, part_2(input).unwrap());
}
