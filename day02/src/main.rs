use std::str::FromStr;

const INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq)]
struct Game {
    r: u32,
    g: u32,
    b: u32,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.r <= 12 && self.g <= 13 && self.b <= 14
    }
}

#[derive(Debug)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut r, mut g, mut b) = (0, 0, 0);
        for x in s.split(", ") {
            let (v, c) = x.split_once(' ').ok_or(ParseGameError)?;
            let v = v.parse().unwrap();
            match c {
                "red" => r = v,
                "green" => g = v,
                "blue" => b = v,
                _ => return Err(ParseGameError),
            }
        }
        Ok(Self { r, g, b })
    }
}

#[derive(Debug)]
struct ParseLineError;

fn parse_line(line: &str) -> Result<(u32, Vec<Game>), ParseLineError> {
    let (id, games) = line.split_once(": ").ok_or(ParseLineError)?;
    let id = id.trim_start_matches("Game ").parse().unwrap();
    let games = games
        .split("; ")
        .map(|g| g.parse().unwrap())
        .collect();
    Ok((id, games))
}

fn power(games: &[Game]) -> u32 {
    let r = games.iter().map(|g| g.r).max().unwrap();
    let g = games.iter().map(|g| g.g).max().unwrap();
    let b = games.iter().map(|g| g.b).max().unwrap();
    r*g*b
}

fn part1() -> u32 {
    INPUT.lines()
        .filter_map(|line| {
            let (id, games) = parse_line(line).unwrap();
            if games.iter().all(|g| g.is_possible()) {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

fn part2() -> u32 {
    INPUT.lines()
        .map(|line| {
            let (_, games) = parse_line(line).unwrap();
            power(&games)
        })
        .sum()
}

fn main() {
    println!("part1 = {}", part1());
    println!("part2 = {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let g: Game = "1 red, 2 green, 6 blue".parse().unwrap();
        assert_eq!(Game { r:1, g:2, b:6 }, g);
    }

    #[test]
    fn test_parse_line() {
        let (id, games) = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        assert_eq!(id, 1);
        assert_eq!(vec![
            Game { b:3, r:4, g:0 },
            Game { r:1, g:2, b:6 },
            Game { g:2, r:0, b:0 },
        ], games);
    }

    #[test]
    fn test_power() {
        let (_, games) = parse_line("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").unwrap();
        let p = power(&games);
        assert_eq!(p, 630);
    }
}