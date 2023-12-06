use std::num::ParseIntError;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("input");

#[derive(Debug)]
enum ParseError {
    InvalidInt,
    InvalidRange,
}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidInt
    }
}

fn part1(input: &str) -> u64 {
    let almanac = parse(input).unwrap();
    let locations = almanac.seeds
        .iter()
        .map(|&x| almanac.plant_seed(x));
    locations.min().unwrap()
}

fn part2(input: &str) -> u64 {
    println!("brute force fiesta");
    let mut mins = Vec::new();
    let almanac = parse(input).unwrap();
    for r in almanac.seeds.chunks(2) {
        let start = r[0];
        let len = r[1];
        println!("evaluating {} seeds (start={})", len, start);
        let min = (start..start+len)
            .into_iter()
            .map(|x| almanac.plant_seed(x))
            .min()
            .unwrap();
        mins.push(min);
    }
    mins.into_iter().min().unwrap()
}

fn main() {
    println!("part1 = {}", part1(INPUT));
    println!("part2 = {}", part2(INPUT));
}

fn parse_seeds(input: &str) -> Result<Vec<u64>,  ParseIntError> {
    input
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(str::parse)
        .collect()
}

fn parse_map(input: &str) -> Result<(&str, Vec<Range>), ParseError> {
    let mut name = "";
    let mut ranges = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            name = line.trim_end_matches(" map:");
        } else {
            ranges.push(line.parse()?);
        }
    }
    Ok((name, ranges))
}

fn parse(input: &str) -> Result<Almanac, ParseError> {
    let mut seeds = Vec::new();
    let mut maps = HashMap::new();
    for part in input.split("\n\n") {
        if part.starts_with("seeds:") {
            seeds = parse_seeds(part)?;
        } else {
            let (name, ranges) = parse_map(part)?;
            maps.insert(name.to_owned(), ranges);
        }
    }
    Ok(Almanac { seeds, maps })
}

struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, Vec<Range>>,
}

impl Almanac {
    fn map(&self, src_idx: u64, category: &str) -> u64 {
        if let Some(ranges) = self.maps.get(category) {
            for r in ranges {
                if r.src_start <= src_idx && src_idx < r.src_start + r.len {
                    return r.dst_start + src_idx - r.src_start
                }
            }
        }
        src_idx
    }

    fn plant_seed(&self, x: u64) -> u64 {
        let x = self.map(x, "seed-to-soil");
        let x = self.map(x, "soil-to-fertilizer");
        let x = self.map(x, "fertilizer-to-water");
        let x = self.map(x, "water-to-light");
        let x = self.map(x, "light-to-temperature");
        let x = self.map(x, "temperature-to-humidity");
        let x = self.map(x, "humidity-to-location");
        x
    }
}

#[derive(Debug, PartialEq)]
struct Range {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl FromStr for Range {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xs: Result<Vec<_>, ParseIntError> = s
            .split(' ')
            .map(str::parse)
            .collect();
        let xs = xs?;
        if xs.len() == 3 {
            Ok(Self {
                dst_start: xs[0],
                src_start: xs[1],
                len: xs[2],
            })
        } else {
            Err(ParseError::InvalidRange)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_parse_seeds() {
        const INPUT: &str = "seeds: 79 14 55 13";
        let seeds = parse_seeds(INPUT).unwrap();
        assert_eq!(seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_parse_map() {
        const INPUT: &str = r#"seed-to-soil map:
50 98 2
52 50 48"#;
        let (name, ranges) = parse_map(INPUT).unwrap();
        assert_eq!(name, "seed-to-soil");
        assert_eq!(ranges, vec![
            Range { dst_start: 50, src_start: 98, len: 2 },
            Range { dst_start: 52, src_start: 50, len: 48 },
        ]);
    }

}