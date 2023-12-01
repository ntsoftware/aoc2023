const INPUT: &str = include_str!("input");

fn part1() -> u32 {
    let mut sum = 0;
    for line in INPUT.lines() {
        let first = line
            .chars()
            .find_map(|c| c.to_digit(10))
            .unwrap();
        let last = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .last()
            .unwrap();
        sum += 10 * first + last;
    }
    sum
}

const PATTERNS: &[(&str, u32)] = &[
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn part2() -> u32 {
    let mut sum = 0;
    for line in INPUT.lines() {
        let (_, first) = PATTERNS
            .iter()
            .filter_map(|&(pat, digit)| line.find(pat).map(|i| (i, digit)))
            .min_by_key(|&(i, _)| i)
            .unwrap();
        let (_, last) = PATTERNS
            .iter()
            .filter_map(|&(pat, digit)| line.rfind(pat).map(|i| (i, digit)))
            .max_by_key(|&(i, _)| i)
            .unwrap();
        sum += 10 * first + last;
    }
    sum
}

fn main() {
    println!("part1 = {}", part1());
    println!("part2 = {}", part2());
}
