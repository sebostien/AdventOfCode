use std::ops::RangeInclusive;

use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1: part_1::<2_000_000>,
        part_2,
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
