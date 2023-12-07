use std::{cmp::Ordering, str::FromStr};

pub fn get_solution() -> crate::Solution<usize, usize> {
    crate::Solution {
        date: (2023, 7),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (251_121_738, 251_421_071),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(usize)]
enum Type {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

use Type::{Five, Four, Full, High, Pair, Three, TwoPair};

impl From<Type> for usize {
    fn from(value: Type) -> Self {
        value as Self
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    bid: usize,
}

impl Hand {
    fn value(&self) -> Type {
        let mut jokers = 0;
        let mut cards = [0; 15];
        for c in self.cards {
            if c == 1 {
                jokers += 1;
            } else {
                cards[c as usize] += 1;
            }
        }

        let mut pair1 = false;
        let mut pair2 = false;
        let mut three = false;

        for count in cards {
            match count {
                5 => return Five,
                4 => {
                    if jokers >= 1 {
                        return Five;
                    } else {
                        return Four;
                    }
                }
                3 => three = true,
                2 => {
                    pair2 = pair1;
                    pair1 = true;
                }
                1 | 0 => {}
                _ => unreachable!(),
            }
        }

        // WOOOWWWW Oh My
        match (three, pair1, pair2, jokers) {
            (false, false, false, 5)
            | (false, false, false, 4)
            | (false, true, false, 3)
            | (true, false, false, 2) => Five,
            (true, true, false, 0) | (true, false, true, 0) | (false, true, true, 1) => Full,
            (true, false, false, 1) | (false, true, false, 2) | (false, false, false, 3) => Four,
            (true, false, false, 0) | (false, true, false, 1) | (false, false, false, 2) => Three,
            (false, true, true, 0) => TwoPair,
            (false, true, false, 0) | (false, false, false, 1) => Pair,
            (false, false, false, 0) => High,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value().cmp(&other.value()) {
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(&other.cards) {
                    if a != b {
                        return a.cmp(b);
                    }
                }
                unreachable!()
            }
            o => o,
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cs, bid) = s.split_once(' ').unwrap();

        let mut cards = [0; 5];

        for (i, c) in cs.chars().enumerate() {
            let c = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                c => c as u8 - b'0',
            };
            cards[i] = c;
        }

        Ok(Self {
            cards,
            bid: bid.parse()?,
        })
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let mut hands = input
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    hands.sort_unstable();

    Ok(hands
        .into_iter()
        .enumerate()
        .fold(0, |sum, (rank, hand)| sum + (rank + 1) * hand.bid))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand2(Hand);

impl FromStr for Hand2 {
    type Err = <Hand as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand = s.parse::<Hand>()?;
        for c in &mut hand.cards {
            if *c == 11 {
                *c = 1;
            }
        }
        Ok(Self(hand))
    }
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let mut hands = input
        .lines()
        .map(Hand2::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    hands.sort_unstable();

    Ok(hands
        .into_iter()
        .enumerate()
        .fold(0, |sum, (rank, hand)| sum + (rank + 1) * hand.0.bid))
}
