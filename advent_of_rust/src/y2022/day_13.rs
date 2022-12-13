use std::cmp::Ordering;

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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut root: Vec<List> = vec![];
        let chars = s.chars().collect::<Vec<_>>();

        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                '[' => {
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
                ']' => {
                    return Err("Mismatched ']'".to_string());
                }
                ',' => {}
                _ => {
                    let mut done = false;
                    if s.len() >= i + 2 {
                        if let Ok(n) = s[i..i + 2].parse() {
                            root.push(List::I(n));
                            i += 1;
                            done = true
                        }
                    }
                    if let Ok(n) = s[i..i + 1].parse() {
                        root.push(List::I(n));
                        done = true
                    }
                    if !done {
                        return Err(format!("Expected a number at position {}\n{}", i, s));
                    }
                }
            }

            i += 1;
        }

        Ok(List::L(root))
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into()
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List::I(l), List::I(r)) => l.cmp(r),
            (List::I(_), List::L(_)) => List::L(vec![self.clone()]).cmp(other),
            (List::L(_), List::I(_)) => self.cmp(&List::L(vec![other.clone()])),
            (List::L(l), List::L(r)) => {
                for i in 0..l.len().min(r.len()) {
                    let c = l[i].cmp(&r[i]);
                    match c {
                        Ordering::Less | Ordering::Greater => return c,
                        _ => {}
                    }
                }

                l.len().cmp(&r.len())
            }
        }
    }
}

struct Pairs(Vec<(List, List)>);

impl std::str::FromStr for Pairs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = vec![];

        for lines in s.trim().split("\n\n") {
            let mut p = lines.split('\n');
            let a = p.next().unwrap().parse()?;
            let b = p.next().unwrap().parse()?;

            match (a, b) {
                (List::L(l), List::L(r)) => {
                    pairs.push((l[0].clone(), r[0].clone()));
                }
                _ => unreachable!(),
            }
        }

        Ok(Self(pairs))
    }
}

fn part_1(input: &str) -> Result<usize, String> {
    let pairs = input.parse::<Pairs>()?;
    let mut sum = 0;
    for (i, row) in pairs.0.iter().enumerate() {
        if let Ordering::Less = row.0.cmp(&row.1) {
            sum += i + 1;
        }
    }

    Ok(sum)
}

fn part_2(input: &str) -> Result<usize, String> {
    let packets = input
        .replace("\n\n", "\n")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<List>>();

    let d2: List = "[[2]]".parse()?;
    let d6: List = "[[6]]".parse()?;

    let mut a = 1;
    let mut b = 2;

    for p in packets.iter() {
        if p.cmp(&d2) == Ordering::Less {
            a += 1;
            b += 1;
        } else if p.cmp(&d6) == Ordering::Less {
            b += 1;
        }
    }

    Ok(a * b)
}
