fn visible(input: &mut Vec<Vec<i8>>) -> usize {
    let mut out = vec![0; input.len()]
        .iter()
        .map(|_| vec![0; input[0].len()])
        .collect::<Vec<_>>();

    let mut up = vec![-1; input[0].len()];
    let mut down = vec![-1; input[0].len()];
    for a in 0..input.len() {
        let mut left = -1;
        let mut right = -1;
        for b in 0..input[a].len() {
            // -->
            if input[a][b] > right {
                right = input[a][b];
                out[a][b] = 1;
            }

            // <--
            let c = input[a].len() - 1 - b;
            if input[a][c] > left {
                left = input[a][c];
                out[a][c] = 1;
            }

            // Down
            if input[a][b] > down[b] {
                down[b] = input[a][b];
                out[a][b] = 1;
            }

            // Up
            let c = input.len() - 1 - a;
            if input[c][b] > up[b] {
                up[b] = input[c][b];
                out[c][b] = 1;
            }
        }
    }

    let mut sum = 0;
    for row in out.iter() {
        for c in row.iter() {
            sum += c;
        }
    }
    sum
}

fn part_1(input: String) -> usize {
    let mut map = input
        .trim()
        .lines()
        .map(|row| {
            row.trim()
                .chars()
                .map(|c| format!("{}", c).parse::<i8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    visible(&mut map)
}

fn scenic(map: &[Vec<i8>], row: usize, col: usize) -> usize {
    let mut a = [0, 0, 0, 0];
    let v = map[row][col];
    for r in map.iter().skip(row + 1) {
        a[2] += 1;
        if r[col] >= v {
            break;
        }
    }
    let mut i = row as isize - 1;
    while i >= 0 {
        a[0] += 1;
        if map[i as usize][col] >= v {
            break;
        }
        i -= 1;
    }
    for i in col + 1..map[row].len() {
        a[3] += 1;
        if map[row][i] >= v {
            break;
        }
    }
    let mut i = col as isize - 1;
    while i >= 0 {
        a[1] += 1;
        if map[row][i as usize] >= v {
            break;
        }
        i -= 1;
    }
    a.iter().product()
}

fn part_2(input: String) -> usize {
    let map = input
        .trim()
        .lines()
        .map(|row| {
            row.trim()
                .chars()
                .map(|c| format!("{}", c).parse::<i8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            sum = sum.max(scenic(&map, row, col));
        }
    }

    sum
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::{get_input_contents, get_year_day};

    #[test]
    fn test_part_1() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_1(input), 1779);
    }

    #[test]
    fn test_part_2() {
        let (year, day) = get_year_day(std::file!());
        let input = get_input_contents(year, day).unwrap();
        assert_eq!(part_2(input), 172224);
    }
}
