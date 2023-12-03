const INPUT: &str = include_str!("input");

#[allow(dead_code)]
const TEST: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn parse_line(line: &str) -> (Vec<(i32, i32, u32)>, Vec<(i32, char)>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut len = 0;
    for (i, c) in line.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            if len == 0 {
                numbers.push((i as i32, 1, d));
            } else {
                let last = numbers.last_mut().unwrap();
                last.1 = last.1 + 1;
                last.2 = last.2 * 10 + d;
            }
            len += 1;
        }
        else {
            len = 0;
            if c != '.' {
                symbols.push((i as i32, c));
            }
        }
    }
    (numbers, symbols)
}

#[derive(Debug)]
struct Number {
    x: i32,
    y: i32,
    len: i32,
    val: u32,
}

#[derive(Debug)]
struct Symbol {
    x: i32,
    y: i32,
    c: char,
}

impl Number {
    fn is_adjacent_to(&self, s: &Symbol) -> bool {
        let x1 = self.x - 1;
        let x2 = self.x + self.len;
        let y1 = self.y - 1;
        let y2 = self.y + 1;
        x1 <= s.x && s.x <= x2 && y1 <= s.y && s.y <= y2
    }
}

impl Symbol {
    fn gear_power(&self, numbers: &[Number]) -> Option<u32> {
        if self.c == '*' {
            let adj = self.get_adjacent(numbers);
            if adj.len() == 2 {
                return Some(adj[0].val * adj[1].val);
            }
        }
        None
    }

    fn get_adjacent<'a>(&self, numbers: &'a [Number]) -> Vec<&'a Number> {
        numbers.iter()
            .filter(|n| n.is_adjacent_to(self))
            .collect()
    }
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let y = i as i32;
        let (line_numbers, line_symbols) = parse_line(line);
        for (x, len, val) in line_numbers {
            numbers.push(Number { x, y, len, val });
        }
        for (x, c) in line_symbols {
            symbols.push(Symbol { x, y, c });
        }
    }
    (numbers, symbols)
}

fn part1(input: &str) -> u32 {
    let (numbers, symbols) = parse_input(input);
    numbers.iter()
        .filter_map(|n| {
            if symbols.iter().any(|s| n.is_adjacent_to(s)) {
                Some(n.val)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let (numbers, symbols) = parse_input(input);
    symbols.iter()
        .filter_map(|s| s.gear_power(&numbers))
        .sum()
}

fn main() {
    println!("part1 = {}", part1(INPUT));
    println!("part2 = {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = ".....+.58.";
        let (numbers, symbols) = parse_line(line);
        assert_eq!(vec![(7, 2, 58)], numbers);
        assert_eq!(vec![(5, '+')], symbols);
    }

    #[test]
    fn test_part1() {
        let answer = part1(TEST);
        assert_eq!(answer, 4361);
    }
}