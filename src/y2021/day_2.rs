use crate::Solution;
use std::str::FromStr;

pub fn get_solution() -> Solution<i32, i32> {
    Solution {
        part_1,
        part_2,
        answer: (1_962_940, 1_813_664_422),
    }
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}
use Command::{Down, Forward, Up};

#[derive(Debug)]
enum CommandParseError {
    InvalidFormat,
    InvalidNumber,
    UnkownCommand,
}
use CommandParseError::{InvalidFormat, InvalidNumber, UnkownCommand};

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s.split_once(' ');

        if let Some((command, amount)) = res {
            if let Ok(amount) = amount.parse::<i32>() {
                let command = match command {
                    "forward" => Forward(amount),
                    "down" => Down(amount),
                    "up" => Up(amount),
                    _ => Err(UnkownCommand)?,
                };

                Ok(command)
            } else {
                Err(InvalidNumber)
            }
        } else {
            Err(InvalidFormat)
        }
    }
}

fn part_1(input: &str) -> anyhow::Result<i32> {
    let commands = input
        .lines()
        .map(|line| line.trim().parse::<Command>().unwrap());

    let mut depth = 0;
    let mut horizontal = 0;

    for command in commands {
        match command {
            Forward(x) => horizontal += x,
            Down(x) => depth += x,
            Up(x) => depth -= x,
        }
    }

    Ok(depth * horizontal)
}

fn part_2(input: &str) -> anyhow::Result<i32> {
    let commands = input
        .lines()
        .map(|line| line.trim().parse::<Command>().unwrap());

    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;

    for command in commands {
        match command {
            Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Down(x) => aim += x,
            Up(x) => aim -= x,
        }
    }

    Ok(depth * horizontal)
}
