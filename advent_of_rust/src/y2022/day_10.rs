fn run_1(input: &Vec<(&str, Option<&str>)>) -> isize {
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

    sum
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

fn part_1(input: String) -> isize {
    let moves = parse_input(&input);
    run_1(&moves)
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

fn part_2(input: String) -> Vec<String> {
    let moves = parse_input(&input);
    run_2(&moves)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::{get_input_contents, get_year_day};

    #[test]
    fn test_part_1() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_1(input), 16020);
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();

        let x = part_2(input.clone());
        println!("{:?}", x);

        let answer = vec![
            "####  ##  #### #  # ####  ##  #    ###  ",
            "#    #  #    # #  #    # #  # #    #  # ",
            "###  #      #  #  #   #  #  # #    #  # ",
            "#    #     #   #  #  #   #### #    ###  ",
            "#    #  # #    #  # #    #  # #    # #  ",
            "####  ##  ####  ##  #### #  # #### #  # ",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        assert_eq!(part_2(input), answer);
    }
}
