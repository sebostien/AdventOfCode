use std::cmp::Ordering;
use std::collections::HashSet;

use crate::Solution;

pub fn get_solution() -> Solution<i64, i64> {
    Solution {
        part_1,
        part_2,
        answer: (5955, 4030),
    }
}

struct Rules(Vec<(i64, i64)>);

impl Rules {
    fn new(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|x| x.trim().split_once("|").unwrap())
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .collect::<Vec<_>>(),
        )
    }

    fn is_sorted(&self, xs: &[i64]) -> bool {
        let mut prev = HashSet::new();
        for &p in xs {
            for (x, y) in &self.0 {
                if *x == p && prev.contains(y) {
                    return false;
                }
            }
            prev.insert(p);
        }

        true
    }

    fn sort(&self, xs: &mut [i64]) -> i64 {
        xs.sort_unstable_by(|a, b| {
            if self.0.iter().find(|(z, o)| z == a && o == b).is_some() {
                Ordering::Less
            } else if self.0.iter().find(|(z, o)| z == b && o == a).is_some() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        xs[xs.len() / 2]
    }
}

fn part_1(input: &str) -> anyhow::Result<i64> {
    let (rules, patterns) = input.trim().split_once("\n\n").unwrap();
    let rules = Rules::new(rules);

    Ok(patterns
        .trim()
        .lines()
        .filter_map(|pattern| {
            let xs = pattern
                .trim()
                .split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            rules.is_sorted(&xs).then_some(xs[xs.len() / 2])
        })
        .sum())
}

fn part_2(input: &str) -> anyhow::Result<i64> {
    let (rules, patterns) = input.trim().split_once("\n\n").unwrap();
    let rules = Rules::new(rules);

    Ok(patterns
        .trim()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter_map(|mut xs| {
            if rules.is_sorted(&xs) {
                None
            } else {
                Some(rules.sort(&mut xs))
            }
        })
        .sum())
}
