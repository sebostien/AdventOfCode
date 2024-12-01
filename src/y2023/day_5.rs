use std::{ops::Range, str::FromStr};

pub fn get_solution() -> crate::Solution<usize, usize> {
    crate::Solution {
        part_1,
        part_2,
        answer: (313_045_984, 20_283_860),
    }
}

#[derive(Debug)]
struct Ranges {
    source_range: Range<usize>,
    dest_range: Range<usize>,
}

impl FromStr for Ranges {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xs = s.split(' ');
        let dest = xs.next().unwrap().parse()?;
        let source = xs.next().unwrap().parse()?;
        let len: usize = xs.next().unwrap().parse()?;

        Ok(Self {
            source_range: source..source + len,
            dest_range: dest..dest + len,
        })
    }
}

#[derive(Debug)]
struct Map {
    source: usize,
    dest: usize,
    ranges: Vec<Ranges>,
}

const START: (&str, usize) = ("seed", 0);
const END: (&str, usize) = ("location", 1);

fn parse(s: &str) -> anyhow::Result<(Vec<usize>, Vec<Map>)> {
    let mut xs = s.trim().split("\n\n");

    let seeds = xs
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut sources = vec![START.0, END.0];

    let maps = xs
        .filter_map(|x| {
            let mut xs = x.lines();
            let (source, dest) = xs.next().unwrap().split_once("-to-")?;
            let ranges = xs.filter_map(|x| x.parse().ok()).collect();
            let dest = &dest[0..dest.find(' ').unwrap_or(dest.len())];

            let source = if let Some(x) = sources.iter().position(|x| *x == source) {
                x
            } else {
                sources.push(source);
                sources.len() - 1
            };

            let dest = if let Some(x) = sources.iter().position(|x| *x == dest) {
                x
            } else {
                sources.push(dest);
                sources.len() - 1
            };

            Some(Map {
                source,
                dest,
                ranges,
            })
        })
        .collect::<Vec<_>>();

    Ok((seeds, maps))
}

fn convert_seed(maps: &[Map], source: usize, seed: usize) -> (usize, usize) {
    if let Some(map) = maps.iter().find(|x| x.source == source) {
        for range in &map.ranges {
            if range.source_range.contains(&seed) {
                return (
                    map.dest,
                    seed - range.source_range.start + range.dest_range.start,
                );
            }
        }
        (map.dest, seed)
    } else {
        unreachable!();
    }
}

fn lowest_location(maps: &[Map], mut start_seeds: impl Iterator<Item = usize>) -> usize {
    let mut seeds = Vec::<(usize, usize)>::new();
    let mut min = usize::MAX;

    loop {
        let (source, seed) = if let Some(x) = seeds.pop() {
            x
        } else if let Some(x) = start_seeds.next().map(|x| (START.1, x)) {
            x
        } else {
            break;
        };

        let (dest, new_seed) = convert_seed(maps, source, seed);
        if dest == END.1 {
            min = min.min(new_seed);
        } else {
            seeds.push((dest, new_seed));
        }
    }

    min
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let (seeds, maps) = parse(input)?;
    Ok(lowest_location(&maps, seeds.into_iter()))
}

fn part_2(input: &str) -> anyhow::Result<usize> {
    let (seeds, maps) = parse(input)?;

    let seeds = seeds
        .chunks(2)
        .map(|xs| xs[0]..xs[0] + xs[1])
        .flat_map(IntoIterator::into_iter);

    Ok(lowest_location(&maps, seeds))
}
