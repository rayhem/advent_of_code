use crate::utils::error::Error;
use crate::utils::solution::Solution;
use itertools::Itertools;
use std::cmp::Ordering;
use std::str::FromStr;

pub struct Day07 {}

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(total_winnings(input.lines().flat_map(parse_bid)).to_string())
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl FromStr for HandType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let counts = s
            .chars()
            .sorted()
            .dedup_with_count()
            .map(|(count, _)| count)
            .collect::<Vec<_>>();

        Ok(match counts.len() {
            1 => HandType::FiveOfAKind,
            2 => match counts.iter().max() {
                Some(3) => HandType::FullHouse,
                Some(4) => HandType::FourOfAKind,
                _ => unreachable!("Excluded by number of groups"),
            },
            3 => match counts.iter().max() {
                Some(2) => HandType::TwoPair,
                Some(3) => HandType::ThreeOfAKind,
                _ => unreachable!("Excluded by number of groups"),
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => Err("Expected hand to have 5 cards")?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand {
            cards: s.to_string(),
            hand_type: HandType::from_str(s)?,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        fn number_faces(ch: char) -> u8 {
            match ch {
                'T' => b'9' + 1,
                'J' => b'9' + 2,
                'Q' => b'9' + 3,
                'K' => b'9' + 4,
                other => other as u8,
            }
        }

        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => {
                let (s, o) = self
                    .cards
                    .chars()
                    .zip(other.cards.chars())
                    .find(|(a, b)| a != b)?;
                number_faces(s).partial_cmp(&number_faces(o))
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

type Bid = usize;

fn parse_bid(s: &str) -> Result<(Hand, Bid), Error> {
    let mut tokens = s.split_whitespace();
    Ok((
        Hand::from_str(tokens.next().ok_or(Error::WrongSizeIterator)?).unwrap(),
        tokens
            .next()
            .ok_or(Error::WrongSizeIterator)?
            .parse::<usize>()?,
    ))
}

fn total_winnings<T: IntoIterator<Item = (Hand, Bid)>>(bids: T) -> usize {
    bids.into_iter()
        .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types() {
        assert_eq!(HandType::from_str(&"AAAAA"), Ok(HandType::FiveOfAKind));
        assert_eq!(HandType::from_str(&"AA8AA"), Ok(HandType::FourOfAKind));
        assert_eq!(HandType::from_str(&"23332"), Ok(HandType::FullHouse));
        assert_eq!(HandType::from_str(&"TTT98"), Ok(HandType::ThreeOfAKind));
        assert_eq!(HandType::from_str(&"23432"), Ok(HandType::TwoPair));
        assert_eq!(HandType::from_str(&"A23A4"), Ok(HandType::OnePair));
        assert_eq!(HandType::from_str(&"23456"), Ok(HandType::HighCard))
    }

    #[test]
    fn example() {
        const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

        assert_eq!(total_winnings(INPUT.lines().flat_map(parse_bid)), 6440);
    }
}

crate::verify!(Day07, crate::my_input!("2023", "Day07"), "251029473", "");
