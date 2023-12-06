use std::collections::HashMap;

pub fn get_solution() -> crate::Solution<usize, usize> {
    crate::Solution {
        date: (2023, 3),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (532_445, 79_842_967),
    }
}

fn adj_symbol(
    xs: &[Vec<u8>],
    row: usize,
    col: usize,
    symbol: impl Fn(&u8) -> bool,
) -> Vec<(usize, usize)> {
    let mut out = vec![];
    for (r, c) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        if let (Some(row), Some(col)) = (row.checked_add_signed(r), col.checked_add_signed(c)) {
            if xs.get(row).and_then(|x| x.get(col)).is_some_and(&symbol) {
                out.push((row, col));
            }
        }
    }
    out
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let lines = input
        .trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (row, line) in lines.iter().enumerate() {
        let mut p = String::new();
        let mut has = false;
        for (col, &c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                p.push(c as char);
                has |=
                    !adj_symbol(&lines, row, col, |c| !c.is_ascii_digit() && *c != b'.').is_empty();
            } else if !p.is_empty() {
                if has {
                    sum += p.parse::<usize>().unwrap();
                }
                p = String::new();
                has = false;
            }
        }
        if has {
            sum += p.parse::<usize>().unwrap();
        }
    }

    Ok(sum)
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let lines = input
        .trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut gears = HashMap::<(usize, usize), Vec<usize>>::new();

    for (row, line) in lines.iter().enumerate() {
        let mut p = String::new();
        let mut gear = Vec::<(usize, usize)>::new();
        for (col, &c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                p.push(c as char);
                gear.append(&mut adj_symbol(&lines, row, col, |&c| c == b'*'));
            } else if !p.is_empty() {
                let n = p.parse::<usize>().unwrap();
                gear.sort_unstable();
                gear.dedup();
                for gear in &gear {
                    if let Some(p) = gears.get_mut(gear) {
                        p.push(n);
                    } else {
                        gears.insert(*gear, vec![n]);
                    }
                }
                gear = vec![];
                p = String::new();
            }
        }
        if !p.is_empty() {
            let n = p.parse::<usize>().unwrap();
            for gear in &gear {
                if let Some(p) = gears.get_mut(gear) {
                    p.push(n);
                } else {
                    gears.insert(*gear, vec![n]);
                }
            }
        }
    }

    let mut sum = 0;
    for (_, vs) in gears {
        if vs.len() == 2 {
            sum += vs.iter().product::<usize>();
        }
    }

    Ok(sum)
}
