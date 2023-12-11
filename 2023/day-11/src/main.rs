use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    let game = parse_game(&input);
    println!("{}", game.sum_of_path_lengths());
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Space,
    Galaxy,
}

#[derive(Debug)]
struct Game {
    cells: Vec<Vec<Cell>>,
    // galaxies: Vec<(usize, usize)>,
    // empty_rows: Vec<usize>,
    // empty_cols: Vec<usize>,
}

impl Game {
    fn galaxies(&self) -> Vec<(usize, usize)> {
        let mut list = Vec::new();

        for (row, cells) in self.cells.iter().enumerate() {
            for (col, cell) in cells.iter().enumerate() {
                if matches!(cell, Cell::Galaxy) {
                    list.push((row, col));
                }
            }
        }

        list
    }

    fn galaxy_pairs(&self) -> Vec<((usize, usize), (usize, usize))> {
        let galaxies = self.galaxies();
        let mut result: Vec<((usize, usize), (usize, usize))> = Vec::new();

        for (i, g1) in galaxies.iter().enumerate() {
            for j in i + 1..galaxies.len() {
                result.push((*g1, galaxies[j]));
            }
        }

        result
    }

    fn sum_of_path_lengths(&self) -> u64 {
        self.galaxy_pairs().iter().map(|(a, b)| self.path_len(*a, *b)).sum()
    }

    fn path_len(&self, a: (usize, usize), b: (usize, usize)) -> u64 {
        a.0.abs_diff(b.0) as u64 + a.1.abs_diff(b.1) as u64
    }
}

fn parse_game(s: &str) -> Game {
    let mut game = Game {
        cells: Vec::new(),
        // galaxies: Vec::new(),
        // empty_rows: Vec::new(),
        // empty_cols: Vec::new(),
    };

    let mut galaxies_in_columns: HashMap<usize, usize> = HashMap::new();

    for (row_index, line) in s.trim().lines().into_iter().enumerate() {
        let cells: Vec<Cell> = line
            .trim()
            .split("")
            .filter(|c| !c.is_empty())
            .map(|s| if s == "." { Cell::Space } else { Cell::Galaxy })
            .collect();

        let row_is_empty = cells.iter().all(|c| matches!(c, Cell::Space));

        // if row_is_empty {
        //     game.empty_rows.push(row_index);
        // }

        for (col_index, cell) in cells.iter().enumerate() {
            if matches!(cell, Cell::Galaxy) {
                // game.galaxies.push((row_index, col_index));
                galaxies_in_columns
                    .entry(col_index)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            } else {
                galaxies_in_columns.entry(col_index).or_insert(0);
            }
        }

        let len = cells.len();
        game.cells.push(cells);

        if row_is_empty {
            game.cells.push(vec![Cell::Space; len]);
        }
    }

    let mut col_offset = 0;

    for (col_index, galaxy_count) in galaxies_in_columns.iter() {
        if *galaxy_count == 0 {
            for mut row in game.cells.iter_mut() {
                row.insert(col_index + col_offset, Cell::Space);
            }

            col_offset += 1;
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
        assert_eq!(13, game.cells[0].len());
        assert_eq!(9, game.galaxies().len());

        assert_eq!(36, game.galaxy_pairs().len());
        assert_eq!(((0, 4), (1, 9)), game.galaxy_pairs()[0]);
    }

    #[test]
    fn test_path_len() {
        let game = parse_game(GAME);
        let galaxies = game.galaxies();
        assert_eq!(15, game.path_len(galaxies[0], galaxies[6]));
        assert_eq!(17, game.path_len(galaxies[2], galaxies[5]));
        assert_eq!(5, game.path_len(galaxies[7], galaxies[8]));
        assert_eq!(9, game.path_len(galaxies[4], galaxies[8]));

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
