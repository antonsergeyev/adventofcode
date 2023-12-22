use core::panic;
use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    let game = game_from_str(&input);
    println!("{}", game.walk(64));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock
}

struct Game {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize)
}

impl Game {
    fn walk(&self, steps: usize) -> usize {
        let mut prev_visited: HashSet<(usize, usize)> = HashSet::new();
        prev_visited.insert(self.start);
        let mut next_visited: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..steps {
            next_visited.clear();
            next_visited.insert(self.start);

            for tile in prev_visited.clone().iter() {
                for successor in self.get_successors(*tile).iter() {
                    if !prev_visited.contains(successor) {
                        next_visited.insert(*successor);
                    }
                }
            }

            prev_visited = next_visited.clone();
        }
        
        // println!("{:?}", next_visited);
        next_visited.len()
    }

    fn get_successors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        if pos.0 > 0 {
            result.push((pos.0 - 1, pos.1));
        }

        if pos.1 > 0 {
            result.push((pos.0, pos.1-1));
        }

        if pos.0 < self.tiles.len() {
            result.push((pos.0+1, pos.1));
        }

        if pos.1 < self.tiles[0].len() {
            result.push((pos.0, pos.1+1));
        }

        return result.into_iter().filter(|pos| matches!(self.tiles[pos.0][pos.1], Tile::Garden)).collect();
    }
}

fn game_from_str(s: &str) -> Game {
    let mut game = Game{
        start: (0, 0),
        tiles: Vec::new(),
    };

    for (row_idx, line) in s.lines().enumerate() {
        let mut row = Vec::new();
        for (col_idx, c) in line.trim().chars().enumerate() {
            if c == '.' {
                row.push(Tile::Garden);
            } else if c == '#' {
                row.push(Tile::Rock);
            } else if c == 'S' {
                row.push(Tile::Garden);
                game.start = (row_idx, col_idx);
            } else {
                panic!("invalid symbol: {}", c)
            }
        }

        game.tiles.push(row);
    }

    return game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk() {
        let game = game_from_str("...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........");
    assert_eq!(16, game.walk(6));
    }
}