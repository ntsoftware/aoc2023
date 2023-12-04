const INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    given: Vec<u32>,
}

impl Card {
    fn winning_count(&self) -> usize {
        self.given.iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    fn points(&self) -> u32 {
        match self.winning_count() {
            0 => 0,
            n => 1 << (n-1),
        }
    }
}

fn parse_card(line: &str) -> Card {
    let (id, rest) = line.split_once(':').unwrap();
    let id = id.trim_start_matches("Card").trim_start().parse().unwrap();
    let (winning, given) = rest.split_once(" | ").unwrap();
    let winning = winning.split(' ')
        .filter_map(|n| n.parse().ok())
        .collect();
    let given = given.split(' ')
        .filter_map(|n| n.parse().ok())
        .collect();
    Card { id, winning, given }
}

fn parse_input(input: &str) -> Vec<Card> {
    input.lines()
        .map(parse_card)
        .collect()
}

fn part1(input: &str) -> u32 {
    parse_input(input).iter()
        .map(|card| card.points())
        .sum()
}

fn part2(input: &str) -> u32 {
    let cards = parse_input(input);
    let mut nums = Vec::new();
    nums.resize(cards.len(), 1);
    for (i, card) in cards.iter().enumerate() {
        let n = card.winning_count();
        for j in i..i+n {
            nums[j+1] += nums[i];
        }
    }
    nums.iter().sum()
}

fn main() {
    println!("part1 = {}", part1(INPUT));
    println!("part2 = {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse_card() {
        let card = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(Card {
            id: 1,
            winning: vec![41, 48, 83, 86, 17],
            given: vec![83, 86, 6, 31, 17, 9, 48, 53],
        }, card);
    }
}