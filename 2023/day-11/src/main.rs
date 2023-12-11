use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    let game = parse_game(&input);
    println!("{}", game.sum_of_path_lengths())
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Space,
    Galaxy,
}

#[derive(Debug)]
struct Game {
    cells: Vec<Vec<Cell>>,
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Game {
    fn galaxy_pairs(&self) -> Vec<((usize, usize), (usize, usize))> {
        let mut result: Vec<((usize, usize), (usize, usize))> = Vec::new();

        for (i, g1) in self.galaxies.iter().enumerate() {
            for j in i + 1..self.galaxies.len() {
                result.push((*g1, self.galaxies[j]));
            }
        }

        result
    }

    fn sum_of_path_lengths(&self) -> u64 {
        self.galaxy_pairs().iter().map(|(a, b)| self.path_len(*a, *b)).sum()
    }

    fn path_len(&self, a: (usize, usize), b: (usize, usize)) -> u64 {
        let mut len = a.0.abs_diff(b.0) as u64 + a.1.abs_diff(b.1) as u64;

        let mut row = 0;
        for i in 0..a.0.abs_diff(b.0) {
            row = if a.0 > b.0 { a.0 - i } else { a.0 + i };

            if self.empty_rows.contains(&row) {
                len += 1000000 - 1;
            }
        }

        let mut col = 0;
        for i in 0..a.1.abs_diff(b.1) {
            col = if a.1 > b.1 { a.1 - i } else { a.1 + i };
            
            if self.empty_cols.contains(&col) {
                len += 1000000 - 1;
            }
        }

        // println!("{:?} {:?}: {}, original {}", a, b, len, a.0.abs_diff(b.0) + a.1.abs_diff(b.1));

        len
    }
}

fn parse_game(s: &str) -> Game {
    let mut game = Game {
        cells: Vec::new(),
        galaxies: Vec::new(),
        empty_rows: Vec::new(),
        empty_cols: Vec::new(),
    };

    let mut galaxies_in_columns: HashMap<usize, usize> = HashMap::new();

    for (row_index, line) in s.trim().lines().into_iter().enumerate() {
        let cells: Vec<Cell> = line
            .trim()
            .split("")
            .filter(|c| !c.is_empty())
            .map(|s| if s == "." { Cell::Space } else { Cell::Galaxy })
            .collect();

        if cells.iter().all(|c| matches!(c, Cell::Space)) {
            game.empty_rows.push(row_index);
        }

        for (col_index, cell) in cells.iter().enumerate() {
            if matches!(cell, Cell::Galaxy) {
                game.galaxies.push((row_index, col_index));
                galaxies_in_columns
                    .entry(col_index)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            } else {
                galaxies_in_columns.entry(col_index).or_insert(0);
            }
        }

        game.cells.push(cells);
    }

    for (col_index, galaxy_count) in galaxies_in_columns.iter() {
        if *galaxy_count == 0 {
            game.empty_cols.push(*col_index);
        }
    }

    game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let game = parse_game(GAME);
        assert_eq!(10, game.cells[0].len());
        assert_eq!(9, game.galaxies.len());
        assert_eq!((0,3), game.galaxies[0]);
        assert_eq!((9,0), game.galaxies[7]);
        assert_eq!(2, game.empty_rows.len());
        assert_eq!(3, game.empty_cols.len());

        assert_eq!(36, game.galaxy_pairs().len());
        assert_eq!(((0, 3), (1, 7)), game.galaxy_pairs()[0]);
    }

    #[test]
    fn test_path_len() {
        let game = parse_game(GAME);
        assert_eq!(15, game.path_len(game.galaxies[0], game.galaxies[6]));
        assert_eq!(17, game.path_len(game.galaxies[2], game.galaxies[5]));
        assert_eq!(5, game.path_len(game.galaxies[7], game.galaxies[8]));
        assert_eq!(9, game.path_len(game.galaxies[4], game.galaxies[8]));

        assert_eq!(374, game.sum_of_path_lengths());
    }
}

const GAME: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
