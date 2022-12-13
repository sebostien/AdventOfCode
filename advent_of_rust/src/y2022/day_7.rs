use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        date: (2022, 7),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (1423358, 545729),
    }
}

use std::collections::{HashMap, HashSet};

type FS = HashMap<String, usize>;

fn run(input: std::str::Lines) -> FS {
    let mut pwd = vec!["/"];
    let mut fs = HashMap::new();
    let mut listed = HashSet::new();

    fs.insert("/".to_string(), 0);

    for line in input {
        let mut s = line.trim().split(' ');

        match s.next().unwrap() {
            "$" => match s.next().unwrap() {
                "cd" => match s.next().unwrap() {
                    "/" => pwd.truncate(1),
                    ".." => {
                        pwd.pop();
                    }
                    x => pwd.push(x),
                },
                "ls" => {}
                _ => panic!("Unkown command: {}", line),
            },
            "dir" => {
                let pp = format!("{}/{}/", pwd.join("/"), s.next().unwrap()).replace("//", "/");
                fs.entry(pp).or_insert(0);
            }
            x => {
                let size: usize = x.parse().unwrap();
                let p = pwd.join("/").replace("//", "/") + "/";
                for (n1, s1) in fs.iter_mut() {
                    if p.starts_with(n1) && !listed.contains(n1) {
                        *s1 += size;
                    }
                    listed.insert(n1.to_string() + x);
                }
            }
        }
    }

    fs
}

fn part_1(input: &str) -> Result<usize, String> {
    let fs = run(input.trim().lines());

    let mut sum = 0;
    for (_, s) in fs.iter() {
        if s < &100000 {
            sum += s;
        }
    }

    Ok(sum)
}

fn part_2(input: &str) -> Result<usize, String> {
    let fs = run(input.trim().lines());

    let total = fs.get("/").unwrap();
    let diff = 70000000 - total;
    let mut closest = total;

    for (_, s) in fs.iter() {
        if diff + s > 30000000 && diff + s < diff + closest {
            closest = s;
        }
    }

    Ok(*closest)
}

