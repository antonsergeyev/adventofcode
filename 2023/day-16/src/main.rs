use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    println!("Energized: {}", grid_from_str(&input).energize());
}

fn grid_from_str(s: &str) -> Grid {
    let mut tiles = Vec::new();

    for line in s.lines() {
        let mut row = Vec::new();
        for c in line.trim().chars() {
            row.push(tile_from_char(c));
        }

        tiles.push(row);
    }

    Grid {
        tiles: tiles,
        visited: HashSet::new()
    }
}

fn tile_from_char(c: char) -> Tile {
    match c {
        '.' => Tile{kind: Kind::Empty, energized: false},
        '|' => Tile{kind: Kind::VertSplitter, energized: false},
        '-' => Tile{kind: Kind::HorSplitter, energized: false},
        '/' => Tile{kind: Kind::ForwardMirror, energized: false},
        '\\' => Tile{kind: Kind::BackMirror, energized: false},
        _ => panic!("invalid char: {}", c)
    }
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Empty,
    ForwardMirror,
    BackMirror,
    VertSplitter,
    HorSplitter
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    kind: Kind,
    energized: bool
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    visited: HashSet<(usize, usize, Direction)>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
struct Light {
    row: usize,
    col: usize,
    direction: Direction
}

impl Grid {
    fn energize(&mut self) -> usize {
        let mut energized_count = 0;
        let mut lights:Vec<Light> = Vec::new();
        lights.push(Light{
            row: 0, col: 0, direction: Direction::Right,
        });

        let mut iters = 0;

        while let Some(light) = lights.pop() {
            iters += 1;

            if iters > 100000 {
                panic!("too long");
            }

            if self.tiles[light.row][light.col].energized == false {
                self.tiles[light.row][light.col].energized = true;
                energized_count += 1;
            }
            
            for next_light in  self.next(light) {
                if self.visited.contains(&(next_light.row, next_light.col, next_light.direction)) {
                    continue;
                }

                self.visited.insert((next_light.row, next_light.col, next_light.direction));
                // println!("total: {}, energized: {}, next light: {:?}", lights.len(), energized_count, next_light);
                lights.push(next_light);
            }
        }

        energized_count
    }

    fn next(&self, light: Light) -> Vec<Light> {
        let tile = self.tiles[light.row][light.col];
        let mut next1: Option<Light> = None;
        let mut next2: Option<Light> = None;
        let mut result = Vec::new();

        match tile.kind {
            Kind::Empty => next1 = self.move_light(light, light.direction),
            Kind::ForwardMirror => match light.direction { 
                Direction::Top => next1 = self.move_light(light, Direction::Right),
                Direction::Bottom => next1 = self.move_light(light, Direction::Left),
                Direction::Left => next1 = self.move_light(light, Direction::Bottom),
                Direction::Right => next1 = self.move_light(light, Direction::Top),
            },
            Kind::BackMirror => match light.direction {
                Direction::Top => next1 = self.move_light(light, Direction::Left),
                Direction::Bottom => next1 = self.move_light(light, Direction::Right),
                Direction::Left => next1 = self.move_light(light, Direction::Top),
                Direction::Right => next1 = self.move_light(light, Direction::Bottom),
            },
            Kind::VertSplitter => match light.direction {
                Direction::Top | Direction::Bottom => next1 = self.move_light(light, light.direction),
                Direction::Left | Direction::Right => {
                    next1 = self.move_light(light, Direction::Top);
                    next2 = self.move_light(light, Direction::Bottom);
                },
            },
            Kind::HorSplitter => match light.direction {
                Direction::Top | Direction::Bottom => {
                    next1 = self.move_light(light, Direction::Left);
                    next2 = self.move_light(light, Direction::Right);
                },
                Direction::Left  | Direction::Right => next1 = self.move_light(light, light.direction),
            },
        }

        if next1.is_some() {
            result.push(next1.unwrap());
        }

        if next2.is_some() {
            result.push(next2.unwrap());
        }

        result
    }

    fn move_light(&self, light: Light, dir: Direction) -> Option<Light> {
        match dir {
            Direction::Top => if light.row > 0 { Some(Light{row: light.row-1, direction: dir, ..light}) } else { None },
            Direction::Bottom => if light.row < self.tiles.len() - 1 { Some(Light{row: light.row+1, direction: dir, ..light}) } else { None },
            Direction::Right => if light.col < self.tiles[0].len() - 1 { Some(Light{col: light.col+1, direction: dir, ..light}) } else { None },
            Direction::Left => if light.col > 0 { Some(Light{col: light.col-1, direction: dir, ..light}) } else { None },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energize() {
        assert_eq!(3, grid_from_str(r"...").energize());
        assert_eq!(3, grid_from_str(r".-.").energize());
        assert_eq!(2, grid_from_str(r".|.").energize());
        assert_eq!(2, grid_from_str(r".\.").energize());
        assert_eq!(2, grid_from_str(r"./.").energize());

        let mut grid = grid_from_str(
            r".\.
              ...
              ./.
            "
        );
        assert_eq!(5, grid.energize());

        let mut grid = grid_from_str(
            r"..\.
              ..-."
        );
        assert_eq!(7, grid.energize());

        let mut grid = grid_from_str(
            r"...\.
              /..||
              \../.
            "
        );
        assert_eq!(12, grid.energize());
    }

    #[test]
    fn test_energize_2() {
        let mut grid: Grid = grid_from_str(r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....");
        assert_eq!(46, grid.energize());
    }
}