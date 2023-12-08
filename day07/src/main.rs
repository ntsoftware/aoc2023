use std::str::FromStr;
use std::cmp::{Ordering, PartialOrd};

const INPUT: &str = include_str!("input");

fn main() {
    println!("Hello, world!");
}

/*
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
*/

fn card_strength(card: char) -> u64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("invalid card")
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq)]
struct Hand(Vec<u64>);

#[derive(Debug)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self( s.chars().map(card_strength).collect() ))
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut num2 = 0;
        let mut num3 = 0;
        let mut num4 = 0;
        let mut num5 = 0;
        for i in 2..=14 {
            match self.0.iter().filter(|&n| *n == i).count() {
                2 => num2 += 1,
                3 => num3 += 1,
                4 => num4 += 1,
                5 => num5 += 1,
                _ => {}
            }
        }
        if num5 == 1 {
            HandType::FiveOfAKind
        } else if num4 == 1 {
            HandType::FourOfAKind
        } else if num3 == 1 && num2 == 1 {
            HandType::FullHouse
        } else if num3 == 1 {
            HandType::ThreeOfAKind
        } else if num2 == 2 {
            HandType::TwoPair
        } else if num2 == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.get_type();
        let b = other.get_type();
        if a < b {
            Some(Ordering::Less)
        } else if a > b {
            Some(Ordering::Greater)
        } else {
            for (a, b) in std::iter::zip(self.0.iter(), other.0.iter()) {
                if a < b {
                    return Some(Ordering::Less)
                } else if a > b {
                    return Some(Ordering::Greater)
                }
            }
            Some(Ordering::Equal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hand_parse() {
        assert_eq!("32T3K".parse::<Hand>().unwrap().0, vec![3, 2, 10, 3, 13]);
    }

    #[test]
    fn test_hand_type() {
        assert_eq!("32T3K".parse::<Hand>().unwrap().get_type(), HandType::OnePair);
        assert_eq!("T55J5".parse::<Hand>().unwrap().get_type(), HandType::ThreeOfAKind);
        assert_eq!("KK677".parse::<Hand>().unwrap().get_type(), HandType::TwoPair);
        assert_eq!("KTJJT".parse::<Hand>().unwrap().get_type(), HandType::TwoPair);
        assert_eq!("QQQJA".parse::<Hand>().unwrap().get_type(), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_hand_cmp() {
        let h1 = "32T3K".parse::<Hand>().unwrap();
        let h2 = "T55J5".parse::<Hand>().unwrap();
        let h3 = "KK677".parse::<Hand>().unwrap();
        let h4 = "KTJJT".parse::<Hand>().unwrap();
        let h5 = "QQQJA".parse::<Hand>().unwrap();
        assert!(h1 < h2);
        assert!(h1 < h3);
        assert!(h3 > h4);
    }
}