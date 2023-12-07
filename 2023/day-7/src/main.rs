use std::{cmp::Ordering, str::FromStr, env, fs};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .expect("path is required: cargo run -- src/input.txt");
    let input = fs::read_to_string(path).expect("could not read file");

    let games = game_from_str(&input);
    println!("Part 1 total winnings: {}", games.get_ranks().iter().sum::<u32>());
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    Value(u8),
    T,
    J,
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

impl Hand {
    fn get_type(&self) -> HandType {
        let counts = self.cards.iter().counts();
        let max = counts
            .clone()
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        let pair_count = counts.clone().into_values().filter(|c| *c == 2).count();
        let unique_count = counts.clone().into_values().filter(|c| *c == 1).count();

        if max.1 == 5 {
            return HandType::FiveOfAKind;
        } else if max.1 == 4 {
            return HandType::FourOfAKind;
        } else if max.1 == 3 && pair_count == 1 {
            return HandType::FullHouse;
        } else if max.1 == 3 && unique_count == 2 {
            return HandType::ThreeOfAKind;
        } else if pair_count == 2 {
            return HandType::TwoPair;
        } else if pair_count == 1 && unique_count == 3 {
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
        println!("COMPARING HANDS ORD {:?} {:?}", self, other);
        let type_a = self.get_type();
        let type_b = other.get_type();

        if type_a != type_b {
            println!("comparing type {:?} and {:?}", type_a, type_b);
            return type_a.cmp(&type_b);
        }

        return self.cards.cmp(&other.cards);
    }
}

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn get_ranks(&self) -> Vec<u32> {
        self.hands
            .iter()
            .sorted()
            .enumerate()
            .map(|(i, h)| {
                println!("bid {} for cards {:?}: {}", i+1, h.cards, h.bid);
                h.bid * (i + 1) as u32
            })
            .collect()
    }
}

fn game_from_str(s: &str) -> Game {
    Game {
        hands: s.lines().
            map(|line| hand_from_str(line.trim())).
            collect(),
    }
}

fn hand_from_str(s: &str) -> Hand {
    let mut parts = s.split_whitespace();
    Hand {
        cards: parts
            .next()
            .unwrap()
            .split("")
            .filter(|c| !c.is_empty())
            .map(|c| c.parse().unwrap())
            .collect(),
        bid: parts.next().unwrap().parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_from_str() {
        assert_eq!("Hand { cards: [Value(3), Value(2), T, Value(3), K], bid: 765 }", format!("{:?}", hand_from_str("32T3K 765")));
        assert_eq!("Hand { cards: [Q, Q, Q, J, A], bid: 483 }", format!("{:?}", hand_from_str("QQQJA 483")));

        assert_eq!(HandType::OnePair, hand_from_str("32T3K 765").get_type());
        assert_eq!(HandType::ThreeOfAKind, hand_from_str("QQQJA 483").get_type());
        assert_eq!(HandType::ThreeOfAKind, hand_from_str("T55J5 483").get_type());
        assert_eq!(HandType::TwoPair, hand_from_str("KK677 483").get_type());

        assert_eq!(HandType::HighCard, hand_from_str("2345A 483").get_type());
    }

    #[test]
    fn test_cmp_hands() {
        assert_eq!(Ordering::Less, hand_from_str("32T3K 765").cmp(&hand_from_str("QQQJA 483")));
        assert_eq!(Ordering::Greater, hand_from_str("QQQJA 483").cmp(&hand_from_str("T55J5 483")));
        assert_eq!(Ordering::Equal, hand_from_str("23456 483").cmp(&hand_from_str("23456 483")));
        assert_eq!(Ordering::Greater, hand_from_str("2345A 483").cmp(&hand_from_str("23456 483")));
        assert_eq!(Ordering::Greater, hand_from_str("T55J5 684").cmp(&hand_from_str("32T3K 765")));

        assert_eq!(true, hand_from_str("KTJJT 765") < hand_from_str("KK677 28"));
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

        assert_eq!(
            "[765, 440, 84, 2736, 2415]",
            format!("{:?}", game.get_ranks())
        );
        assert_eq!(6440u32, game.get_ranks().iter().sum());
        
    }
}
