use crate::Solution;

pub fn get_solution() -> Solution<usize, String> {
    Solution {
        date: (2022, 10),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (
            16020,
            vec![
                "####  ##  #### #  # ####  ##  #    ###  ".to_string(),
                "#    #  #    # #  #    # #  # #    #  # ".to_string(),
                "###  #      #  #  #   #  #  # #    #  # ".to_string(),
                "#    #     #   #  #  #   #### #    ###  ".to_string(),
                "#    #  # #    #  # #    #  # #    # #  ".to_string(),
                "####  ##  ####  ##  #### #  # #### #  # ".to_string(),
            ]
            .join("\n"),
        ),
    }
}

fn run_1(input: &Vec<(&str, Option<&str>)>) -> usize {
    let mut x: isize = 1;
    let mut c = 1;
    let mut i = 0;
    let mut addx = 0;

    let mut sum = 0;

    while i < input.len() || addx != 0 {
        if [20, 60, 100, 140, 180, 220].contains(&c) {
            sum += c * x;
        }

        if addx != 0 {
            x += addx;
            addx = 0;
        } else {
            let row = input[i];
            match row.0 {
                "noop" => {}
                "addx" => {
                    addx = row.1.unwrap().parse().unwrap();
                }
                _ => unreachable!("Unkown instruction: {:?}", row),
            }
            i += 1;
        }

        c += 1;
    }

    sum as usize
}

fn parse_input(input: &str) -> Vec<(&str, Option<&str>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut x = line.split(' ');
            (x.next().unwrap(), x.next())
        })
        .collect()
}

fn part_1(input: &str) -> Result<usize, String> {
    let moves = parse_input(input);
    Ok(run_1(&moves))
}

fn run_2(input: &Vec<(&str, Option<&str>)>) -> Vec<String> {
    let mut x: isize = 1;
    let mut c: isize = 1;
    let mut i = 0;
    let mut j = 0;
    let mut addx = 0;

    let mut s = vec!["".to_string(); 6];

    while i < input.len() || addx != 0 {
        s[j] += if (x - ((c - 1) % 40) as isize).abs() <= 1 {
            "#"
        } else {
            " "
        };

        if addx != 0 {
            x += addx;
            addx = 0;
        } else {
            let row = input[i];
            match row.0 {
                "noop" => {}
                "addx" => {
                    addx = row.1.unwrap().parse().unwrap();
                }
                _ => unreachable!("Unkown instruction: {:?}", row),
            }
            i += 1;
        }
        j = (c / 40) as usize;
        c += 1;
    }

    s
}

fn part_2(input: &str) -> Result<String, String> {
    let moves = parse_input(input);
    Ok(run_2(&moves).join("\n"))
}
