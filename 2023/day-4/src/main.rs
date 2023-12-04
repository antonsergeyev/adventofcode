use std::{collections::{HashSet, VecDeque, HashMap, hash_map::Entry}, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .expect("path is required: cargo run -- src/input.txt");
    let input = fs::read_to_string(path).expect("could not read file");

    let points_part1: u32 = input
        .lines()
        .map(parse_card)
        .map(|card| card.get_points())
        .sum();

    println!("Total points: {}", points_part1);

    println!("Card count: {}", count_cards(&input));
}

#[derive(Debug)]
#[derive(Clone)]
struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn get_matches(&self) -> Vec<u32> {
        self.winning_numbers
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&self.my_numbers.iter().collect::<HashSet<_>>())
            .map(|number| **number)
            .collect()
    }

    fn get_points(&self) -> u32 {
        self.get_matches()
            .iter()
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
    }
}

fn count_cards(s: &str) -> u32 {
    let mut copies: HashMap<usize, u32> = HashMap::new();

    for (i, line) in s.lines().enumerate() {
        let card = parse_card(line);

        let mut next_copies_count = 1;

        match copies.entry(i) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
                println!("inserting card {} times {}", i+1, *entry.get());
                next_copies_count = *entry.get();
            },
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
        
        for next_card in i+1..i + card.get_matches().len() + 1 {
            let _ = *copies.entry(next_card).
                    and_modify(|v| *v = *v + next_copies_count).
                    or_insert(next_copies_count);
        }
    }

    copies.values().sum()
}

fn parse_card(s: &str) -> Card {
    let mut parts = s.split(":").nth(1).unwrap().split("|");

    Card {
        winning_numbers: parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|number| number.trim().parse().unwrap())
            .collect(),
        my_numbers: parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|number| number.trim().parse().unwrap())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let card1 = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(
            "Card { winning_numbers: [41, 48, 83, 86, 17], my_numbers: [83, 86, 6, 31, 17, 9, 48, 53] }",
            format!("{:?}", card1),
        );

        assert_eq!(8, card1.get_points());

        let card2 = parse_card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(2, card2.get_points());

        let card6 = parse_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(0, card6.get_points());
    }

    #[test]
    fn test_count_cards() {
        let count = count_cards(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(30, count);
    }
}
