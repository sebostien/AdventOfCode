use std::cmp::Ordering;

use anyhow::anyhow;

use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        date: (2022, 13),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (5625, 23111),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum List {
    I(usize),
    L(Vec<List>),
}

impl std::str::FromStr for List {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut root: Vec<Self> = vec![];
        let chars = s.chars().collect::<Vec<_>>();

        let mut i = 0;
        while i < chars.len() {
            match chars.get(i) {
                Some('[') => {
                    let mut depth = 0;
                    let mut j = 0;
                    for (k, &c) in chars.iter().skip(i).enumerate() {
                        if c == '[' {
                            depth += 1;
                        } else if c == ']' {
                            depth -= 1;
                            if depth == 0 {
                                j = k;
                                break;
                            }
                        }
                    }
                    root.push(s[i + 1..i + j].parse()?);
                    i += j + 1;
                }
                Some(']') => {
                    return Err(anyhow!("Mismatched ']'"));
                }
                Some(',') => {}
                Some(_) => {
                    let mut done = false;
                    if s.len() >= i + 2 {
                        if let Ok(n) = s[i..i + 2].parse() {
                            root.push(Self::I(n));
                            i += 1;
                            done = true;
                        }
                    }
                    if let Ok(n) = s[i..=i].parse() {
                        root.push(Self::I(n));
                        done = true;
                    }
                    if !done {
                        return Err(anyhow!("Expected a number at position {i}\n{s}"));
                    }
                }
                None => unreachable!(),
            }

            i += 1;
        }

        Ok(Self::L(root))
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::I(l), Self::I(r)) => l.cmp(r),
            (Self::I(_), Self::L(_)) => Self::L(vec![self.clone()]).cmp(other),
            (Self::L(_), Self::I(_)) => self.cmp(&Self::L(vec![other.clone()])),
            (Self::L(l), Self::L(r)) => {
                for i in 0..l.len().min(r.len()) {
                    let c = l[i].cmp(&r[i]);
                    match c {
                        Ordering::Less | Ordering::Greater => return c,
                        Ordering::Equal => {}
                    }
                }

                l.len().cmp(&r.len())
            }
        }
    }
}

struct Pairs(Vec<(List, List)>);

impl std::str::FromStr for Pairs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = vec![];

        for lines in s.trim().split("\n\n") {
            let mut p = lines.split('\n');
            let left = p.next().unwrap().parse()?;
            let right = p.next().unwrap().parse()?;

            match (left, right) {
                (List::L(left), List::L(right)) => {
                    pairs.push((left[0].clone(), right[0].clone()));
                }
                _ => Err(anyhow!("Unexpected None"))?,
            }
        }

        Ok(Self(pairs))
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let pairs = input.parse::<Pairs>()?;
    let mut sum = 0;
    for (i, row) in pairs.0.iter().enumerate() {
        if let Ordering::Less = row.0.cmp(&row.1) {
            sum += i + 1;
        }
    }

    Ok(sum)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let packets = input
        .replace("\n\n", "\n")
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<List>, _>>()?;

    let d2: List = "[[2]]".parse()?;
    let d6: List = "[[6]]".parse()?;

    let mut a = 1;
    let mut b = 2;

    for p in packets {
        if p.cmp(&d2) == Ordering::Less {
            a += 1;
            b += 1;
        } else if p.cmp(&d6) == Ordering::Less {
            b += 1;
        }
    }

    Ok(a * b)
}
