use std::ops::RangeInclusive;

use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        date: (2022, 15),
        part_1: Box::new(part_1::<2_000_000>),
        part_2: Box::new(part_2),
        answer: (6_275_922, 0),
    }
}

#[derive(Debug, Default)]
struct MultiRange(Vec<RangeInclusive<isize>>);

impl MultiRange {
    fn size(self) -> usize {
        let mut a = self
            .0
            .iter()
            .flat_map(std::clone::Clone::clone)
            .collect::<Vec<_>>();

        a.sort_unstable();
        a.dedup();
        a.len()
    }

    fn extend(&mut self, other: RangeInclusive<isize>) {
        self.0.push(other);
    }

    fn subtract(&mut self, beacon: isize) {
        let mut new_ranges = vec![];
        for range in &self.0 {
            // Split range into two new ones, excluding the beacon
            if range.contains(&beacon) {
                #[allow(clippy::range_minus_one)]
                let left = *range.start()..=beacon - 1;
                if !left.is_empty() {
                    new_ranges.push(left);
                }
                let right = beacon + 1..=*range.end();
                if !right.is_empty() {
                    new_ranges.push(right);
                }
            } else {
                new_ranges.push(range.clone());
            }
        }

        let _ = std::mem::replace(&mut self.0, new_ranges);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    x: isize,
    y: isize,
    bx: isize,
    by: isize,
}

impl std::str::FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ns = s
            .split('=')
            .filter_map(|f| {
                f.chars()
                    .filter(|&c| c.is_ascii_digit() || c == '-')
                    .collect::<String>()
                    .parse()
                    .ok()
            })
            .collect::<Vec<_>>();

        Ok(Self {
            x: ns[0],
            y: ns[1],
            bx: ns[2],
            by: ns[3],
        })
    }
}

fn part_1<const ROW: isize>(input: &str) -> anyhow::Result<usize> {
    let sensors = input
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Sensor>, _>>()?;

    let mut ranges = MultiRange::default();
    for s in sensors {
        let dist = (s.x - s.bx).abs() + (s.y - s.by).abs();

        let on_row = dist - (ROW - s.y).abs();

        ranges.extend((s.x - on_row)..=(s.x + on_row));

        if s.by == ROW {
            ranges.subtract(s.bx);
        }
    }

    Ok(ranges.size())
}

fn part_2(_input: &str) -> anyhow::Result<usize> {
    Err(anyhow!("Not Done!"))
}

#[test]
fn test_parse() {
    let input = r#"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
     "#;

    assert_eq!(part_1::<10>(input).unwrap(), 26);
}
