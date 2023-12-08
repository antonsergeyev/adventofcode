use std::{cmp::Ordering, collections::HashMap, env, fs, str::FromStr};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .expect("path is required: cargo run -- src/input.txt");
    let input = fs::read_to_string(path).expect("could not read file");

    let games = game_from_str(&input);
    println!("total winnings: {}", games.get_ranks().iter().sum::<u64>());
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    J,
    Value(u8),
    T,
    Q,
    K,
    A,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(input: &str) -> Result<Card, Self::Err> {
        match input {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            _ => {
                if let Ok(value) = input.parse::<u8>() {
                    Ok(Card::Value(value))
                } else {
                    Err(format!("bad input: {}", input))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

#[derive(Debug, Clone)]
struct HandStats {
    jokers: usize,
    same: usize,
    pairs: usize,
}

impl Hand {
    fn get_stats(&self) -> HandStats {
        let counts = self.cards.iter().counts();
        let counts_without_jokers = counts
            .clone()
            .into_iter()
            .filter(|(card, _count)| !matches!(card, Card::J))
            .collect::<HashMap<&Card, usize>>();

        HandStats {
            jokers: *counts.get(&Card::J).unwrap_or(&0),
            same: counts_without_jokers
                .clone()
                .into_iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(_card, count)| count)
                .unwrap_or(0),
            pairs: counts_without_jokers
                .clone()
                .into_values()
                .filter(|c| *c == 2)
                .count(),
        }
    }
    fn get_type(&self)  -> HandType {
        let stats = self.get_stats();
        let t = self.get_type_no_jokers(&stats);

        if stats.jokers == 0 {
            return t
        }

        match t {
            HandType::HighCard => match stats.jokers {
                1 => HandType::OnePair,
                2 => HandType::ThreeOfAKind,
                3 => HandType::FourOfAKind,
                _ => HandType::FiveOfAKind,
            }
            HandType::OnePair => match stats.jokers {
                1 => HandType::ThreeOfAKind,
                2 => HandType::FourOfAKind,
                _ => HandType::FiveOfAKind,
            },
            HandType::TwoPair => HandType::FullHouse,
            HandType::ThreeOfAKind => if stats.jokers == 1 { HandType::FourOfAKind } else {HandType::FiveOfAKind},
            HandType::FourOfAKind => HandType::FiveOfAKind,
            _ => HandType::FiveOfAKind,
        }
    }
    fn get_type_no_jokers(&self, stats: &HandStats) -> HandType {
        if stats.same == 5 {
            return HandType::FiveOfAKind;
        } else if stats.same == 4 {
            return HandType::FourOfAKind;
        } else if stats.same == 3 && stats.pairs == 1 {
            return HandType::FullHouse;
        } else if stats.same == 3 {
            return HandType::ThreeOfAKind;
        } else if stats.pairs == 2 {
            return HandType::TwoPair;
        } else if stats.pairs == 1 {
            return HandType::OnePair;
        } else {
            return HandType::HighCard;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_a = self.get_type();
        let type_b = other.get_type();

        if type_a != type_b {
            return type_a.cmp(&type_b);
        }

        for (i, c) in self.cards.iter().enumerate() {
            let cmp = c.cmp(&other.cards[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        return Ordering::Equal;
    }
}

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn get_ranks(&self) -> Vec<u64> {
        self.hands
            .iter()
            .sorted()
            .enumerate()
            .map(|(i, h)| {
                (h.bid as u64) * (i + 1) as u64
            })
            .collect()
    }
}

fn game_from_str(s: &str) -> Game {
    Game {
        hands: s.lines().map(|line| hand_from_str(line.trim())).collect(),
    }
}

fn hand_from_str(s: &str) -> Hand {
    let mut parts = s.split_whitespace();
    let cards: Vec<Card> = parts
        .next()
        .unwrap()
        .split("")
        .filter(|c| !c.is_empty())
        .map(|c| c.parse().unwrap())
        .collect();

    Hand {
        cards: cards.clone(),
        bid: parts.next().unwrap().parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_from_str() {
        assert_eq!(HandType::FourOfAKind, hand_from_str("QQQJA 483").get_type());

        assert_eq!("Hand { cards: [Value(3), Value(2), T, Value(3), K], bid: 765 }", format!("{:?}", hand_from_str("32T3K 765")));
        assert_eq!("Hand { cards: [Q, Q, Q, J, A], bid: 483 }", format!("{:?}", hand_from_str("QQQJA 483")));

        assert_eq!(HandType::OnePair, hand_from_str("32T3K 765").get_type());
        assert_eq!(HandType::TwoPair, hand_from_str("KK677 483").get_type());

        assert_eq!(HandType::FourOfAKind, hand_from_str("QQQJA 483").get_type());
        assert_eq!(HandType::FourOfAKind, hand_from_str("T55J5 483").get_type());
        assert_eq!(HandType::FourOfAKind, hand_from_str("KTJJT 483").get_type());

        assert_eq!(HandType::HighCard, hand_from_str("2345A 483").get_type());

        assert_eq!(HandType::FiveOfAKind, hand_from_str("JJJJJ 483").get_type());
        assert_eq!(HandType::FiveOfAKind, hand_from_str("JJAAA 483").get_type());
        assert_eq!(HandType::FiveOfAKind, hand_from_str("JAAAA 483").get_type());
        assert_eq!(HandType::FiveOfAKind, hand_from_str("JJJJA 483").get_type());

        assert_eq!(HandType::FiveOfAKind, hand_from_str("JJJJA 483").get_type());
        assert_eq!(HandType::FiveOfAKind, hand_from_str("JJJJA 483").get_type());
        assert_eq!(HandType::FiveOfAKind, hand_from_str("JJAAA 483").get_type());

        assert_eq!(
            HandType::ThreeOfAKind,
            hand_from_str("JJAKT 483").get_type()
        );
    }

    #[test]
    fn test_cmp_hands() {
        assert_eq!(
            Ordering::Less,
            hand_from_str("T55J5 1").cmp(&hand_from_str("QQQJA 1"))
        );
        assert_eq!(
            Ordering::Less,
            hand_from_str("QQQJA 1").cmp(&hand_from_str("KTJJT 1"))
        );

        // assert_eq!(Ordering::Less, hand_from_str("32T3K 765").cmp(&hand_from_str("QQQJA 483")));
        // assert_eq!(Ordering::Greater, hand_from_str("QQQJA 483").cmp(&hand_from_str("T55J5 483")));
        // assert_eq!(Ordering::Equal, hand_from_str("23456 483").cmp(&hand_from_str("23456 483")));
        // assert_eq!(Ordering::Greater, hand_from_str("2345A 483").cmp(&hand_from_str("23456 483")));
        // assert_eq!(Ordering::Greater, hand_from_str("T55J5 684").cmp(&hand_from_str("32T3K 765")));

        // assert_eq!(true, hand_from_str("KTJJT 765") < hand_from_str("KK677 28"));
    }

    #[test]
    fn test_game_from_str() {
        let game = game_from_str(
            "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483",
        );
        assert_eq!(5, game.hands.len());

        game.hands.iter().for_each(|h| println!("{:?}", h));

        assert_eq!(
            "Hand { cards: [Value(3), Value(2), T, Value(3), K], bid: 765 }",
            format!("{:?}", game.hands[0])
        );

        // assert_eq!(
        //     "[765, 440, 84, 2736, 2415]",
        //     format!("{:?}", game.get_ranks())
        // );
        assert_eq!(5905u64, game.get_ranks().iter().sum());
    }
}
