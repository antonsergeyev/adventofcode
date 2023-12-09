use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");

    let sum_of_next_values: i32 = input.lines().map(|l| next_value(parse_line(l))).sum();
    let sum_of_prev_values: i32 = input.lines().map(|l| prev_value(parse_line(l))).sum();

    println!("Next: {}, prev: {}", sum_of_next_values, sum_of_prev_values);
}

fn get_diffs(data: Vec<i32>) -> Vec<Vec<i32>> {
    let mut diffs: Vec<Vec<i32>> = Vec::new();
    diffs.push(data.clone());

    let mut current_vec = data;
    let mut all_zeroes = false;

    while !all_zeroes {
        let mut next_diffs: Vec<i32> = Vec::new();
        all_zeroes = true;

        for (current, next) in current_vec.iter().tuple_windows() {
            let diff = next-current;
            next_diffs.push(diff);
            
            if diff != 0 {
                all_zeroes = false;
            }
        }

        diffs.push(next_diffs.clone());
        current_vec = next_diffs;
    }

    diffs
}

fn next_value(data: Vec<i32>) -> i32 {
    let diffs = get_diffs(data);
    let mut next_value = *diffs.last().unwrap().last().unwrap();

    for level in diffs.iter().rev().skip(1) {
        next_value = level.last().unwrap() + next_value;
    }

    next_value
}

fn prev_value(data: Vec<i32>) -> i32 {
    let diffs = get_diffs(data);
    let mut prev_value = *diffs.last().unwrap().first().unwrap();

    for level in diffs.iter().rev().skip(1) {
        let first = level.first().unwrap();
        // println!("prev: {}, current: {}, result: {}", prev_value, first, first - prev_value);
        prev_value = first - prev_value;
    }

    prev_value
}

fn parse_line(s: &str) -> Vec<i32> {
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prev_value() {
        let mut values = parse_line("10 13 16 21 30 45");
        assert_eq!(5, prev_value(values));

        values = parse_line("1 3 6 10 15 21");
        assert_eq!(0, prev_value(values));
    }

    #[test]
    fn test_next_value() {
        let mut values = parse_line("0 3 6 9 12 15");
        assert_eq!(18, next_value(values));

        values = parse_line("10 13 16 21 30 45");
        assert_eq!(68, next_value(values));
    }
}