use core::num;
use std::{io::Empty, fs, env};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .expect("path is required: cargo run -- src/input.txt");
    let input = fs::read_to_string(path).expect("could not read file");
    
    
    let matrix = matrix_from_str(&input);

    println!("Part numbers: {}", get_part_numbers(&matrix).iter().sum::<i32>());
    println!("Gear ratios: {}", get_gear_ratios(&matrix).iter().sum::<i32>());
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
enum Item {
    PartNumber(i32),
    Symbol(char),
    Empty
}

#[derive(Debug)]
struct Matrix {
    items: Vec<Vec<Item>>
}

impl Matrix {
    fn get(&self, row: usize, col: usize) -> Item {
        *self.items.get(row).unwrap().get(col).unwrap()
    }

    fn has_symbol_neighbors(&self, row: usize, col: usize) -> bool {
        self.get_neighbors(row, col).iter().any(|item| matches!(item, Item::Symbol(..)))
    }

    fn gear_ratio(&self, row: usize, col: usize) -> Option<i32> {
        match self.get(row, col) {
            Item::Symbol('*') => {
                let mut first_part_number = -1;

                for neighbor in self.get_neighbors(row, col).iter() {
                    if let Item::PartNumber(n) = neighbor {
                        if first_part_number == -1 {
                            first_part_number = *n;
                        } else if *n == first_part_number {
                            continue;
                        } else {
                            return Some(*n * first_part_number);
                        }
                    }
                }
                
                return None;
            },
            _ => None
        }

    }

    fn get_neighbors(&self, row: usize, col: usize) -> Vec<Item> {
        let mut items = Vec::new();

        if row > 0 {
            // top
            items.push(self.get(row-1, col));
            // top left
            if col > 0 {
                items.push(self.get(row-1, col-1));    
            }
            // top right
            if col < self.items.get(0).unwrap().len() - 1 {
                items.push(self.get(row-1, col+1));    
            }
        }

        if row < self.items.len() - 1 {
            // bottom
            items.push(self.get(row+1, col));
            // bottom left
            if col > 0 {
                items.push(self.get(row+1, col-1));
            }
            // bottom right
            if col < self.items.get(0).unwrap().len() - 1 {
                items.push(self.get(row+1, col+1));
            }
        }

        if col > 0 {
            // left
            items.push(self.get(row, col-1));
        }

        if col < self.items.get(0).unwrap().len() - 1 {
            // right
            items.push(self.get(row, col+1));
        }

        return items;
    }
}

fn get_gear_ratios(m: &Matrix) -> Vec<i32> {
    let mut ratios = Vec::new();

    for (row_index, row) in m.items.iter().enumerate() {
        for (col_index, item) in row.iter().enumerate() {
            if let Some(gear_ratio) = m.gear_ratio(row_index, col_index) {
                ratios.push(gear_ratio);
            }
        }
    }

    return ratios;
}

fn get_part_numbers(m: &Matrix) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for (row_index, row) in m.items.iter().enumerate() {
        let mut number_added: bool = false;

        for (col_index, item) in row.iter().enumerate() {
            match item {
                Item::PartNumber(n) => {
                    if m.has_symbol_neighbors(row_index, col_index) && !number_added {
                        number_added = true;
                        numbers.push(*n);
                    }
                },
                _ => {
                    number_added = false;
                }
            }
        }
    }

    return numbers;
}

fn matrix_from_str(s: &str) -> Matrix {
    let mut engine = Matrix{
        items: Vec::new()
    };
    let line_regexp = Regex::new(
        r"(?m)((?P<empty>\.)|(?P<number>\d+)|(?P<symbol>[^\d\.]))"
    ).unwrap();


    for (row, line) in s.lines().enumerate() {
        let mut col = 0;

        let v:Vec<Item> = line_regexp.captures_iter(line.trim()).flat_map(|m| {
            col += 1;

            if m.name("empty").is_some() {
                return vec![Item::Empty]
            } else if let Some(symbol) = m.name("symbol") {
                return vec![Item::Symbol(symbol.as_str().chars().next().unwrap())];
            } else {
                let part_number_str = m.name("number").unwrap().as_str();
                let part_number_int = part_number_str.parse().unwrap();

                return vec![Item::PartNumber(part_number_int); part_number_str.len()];
            }
        }).collect();

        engine.items.push(v);
    }

    return engine;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_from_str() {
        let m = matrix_from_str(".....+.58.");
        assert_eq!(
            "Matrix { items: [[Empty, Empty, Empty, Empty, Empty, Symbol('+'), Empty, PartNumber(58), PartNumber(58), Empty]] }",
            format!("{:?}", m)
        );
    }

    #[test]
    fn test_get_part_numbers() {
        let m = matrix_from_str("467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..");
        assert_eq!(vec![467i32, 35i32, 633i32, 617i32, 592i32, 755i32, 664i32, 598i32], get_part_numbers(&m));
        assert_eq!(vec![16345i32, 451490i32], get_gear_ratios(&m));
    }
}