use crate::Solution;

pub fn get_solution() -> Solution<usize, usize> {
    Solution {
        part_1,
        part_2,
        answer: (1_423_358, 545_729),
    }
}

use std::collections::{HashMap, HashSet};

type FS = HashMap<String, usize>;

fn run(input: std::str::Lines<'_>) -> FS {
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
                _ => panic!("Unkown command: {line}"),
            },
            "dir" => {
                let pp = format!("{}/{}/", pwd.join("/"), s.next().unwrap()).replace("//", "/");
                fs.entry(pp).or_insert(0);
            }
            x => {
                let size: usize = x.parse().unwrap();
                let p = pwd.join("/").replace("//", "/") + "/";
                for (n1, s1) in &mut fs {
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

fn part_1(input: &str) -> anyhow::Result<usize> {
    let fs = run(input.trim().lines());

    let mut sum = 0;
    for s in fs.values() {
        if s < &100_000 {
            sum += s;
        }
    }

    Ok(sum)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let fs = run(input.trim().lines());

    let total = fs.get("/").unwrap();
    let diff = 70_000_000 - total;
    let mut closest = total;

    for s in fs.values() {
        if diff + s > 30_000_000 && diff + s < diff + closest {
            closest = s;
        }
    }

    Ok(*closest)
}
