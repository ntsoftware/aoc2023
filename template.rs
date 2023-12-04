#[allow(dead_code)]
const INPUT: &str = include_str!("input");

#[allow(dead_code)]
const TEST: &str = r#"
"#;

fn part1(input: &str) -> u32 {
    0
}

fn part2(input: &str) -> u32 {
    0
}

fn main() {
    println!("part1 = {}", part1(INPUT));
    println!("part2 = {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST), 0);
    }
}
