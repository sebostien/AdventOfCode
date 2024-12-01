use crate::Solution;

pub fn get_solution() -> Solution<usize, String> {
    Solution {
        part_1,
        part_2,
        answer: (
            16020,
            [
                "####  ##  #### #  # ####  ##  #    ###  ",
                "#    #  #    # #  #    # #  # #    #  # ",
                "###  #      #  #  #   #  #  # #    #  # ",
                "#    #     #   #  #  #   #### #    ###  ",
                "#    #  # #    #  # #    #  # #    # #  ",
                "####  ##  ####  ##  #### #  # #### #  # ",
            ]
            .map(String::from)
            .join("\n"),
        ),
    }
}

fn run_1(input: &Vec<(&str, Option<&str>)>) -> usize {
    let mut x: isize = 1;
    let mut cycle = 1;
    let mut inst = 0;
    let mut addx = 0;

    let mut sum = 0;

    while inst < input.len() || addx != 0 {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            sum += cycle * x;
        }

        if addx == 0 {
            let row = input[inst];
            match row.0 {
                "noop" => {}
                "addx" => {
                    addx = row.1.unwrap().parse().unwrap();
                }
                _ => unreachable!("Unkown instruction: {row:?}"),
            }
            inst += 1;
        } else {
            x += addx;
            addx = 0;
        }

        cycle += 1;
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

fn part_1(input: &str) -> anyhow::Result<usize> {
    let moves = parse_input(input);
    Ok(run_1(&moves))
}

fn run_2(input: &Vec<(&str, Option<&str>)>) -> Vec<String> {
    let mut x: isize = 1;
    let mut cycle: isize = 1;
    let mut inst = 0;
    let mut j = 0;
    let mut addx = 0;

    let mut crt = vec![String::new(); 6];

    while inst < input.len() || addx != 0 {
        crt[j] += if (x - ((cycle - 1) % 40)).abs() <= 1 {
            "#"
        } else {
            " "
        };

        if addx == 0 {
            let row = input[inst];
            match row.0 {
                "noop" => {}
                "addx" => {
                    addx = row.1.unwrap().parse().unwrap();
                }
                _ => unreachable!("Unkown instruction: {:?}", row),
            }
            inst += 1;
        } else {
            x += addx;
            addx = 0;
        }
        j = (cycle / 40) as usize;
        cycle += 1;
    }

    crt
}

fn part_2(input: &str) -> anyhow::Result<String> {
    let moves = parse_input(input);
    Ok(run_2(&moves).join("\n"))
}
