use pathfinding::{directed::{astar::astar_bag, idastar::idastar}, prelude::astar};
use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
    hash::Hasher,
};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    let grid = game_from_str(&input);

    let mut costs = Vec::new();

    // this is a hack, and none of the multipliers actually work :(
    for i in 400..=550 {
        let mult = i as f32 * 0.009;
        let cost = grid.get_best_path_cost(mult);
        costs.push(cost);
        println!("Cost with distance multiplier {}: {}", mult, cost);
    }
    
    println!("Min cost: {}", costs.iter().min().unwrap());
}

fn game_from_str(s: &str) -> Game {
    let mut result: Vec<Vec<usize>> = Vec::new();
    for line in s.lines() {
        result.push(
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }

    Game { grid: result }
}

#[derive(Debug, Clone, Copy, Eq)]
struct Pos {
    row: usize,
    col: usize,

    direction: Direction,
    straight_steps: usize,
    cost: usize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
        self.direction.hash(state);
    }
}

struct Game {
    grid: Vec<Vec<usize>>,
}

impl Game {
    fn get_best_path_cost(&self, distance_mult: f32) -> usize {
        let goal = Pos {
            row: self.grid.len() - 1,
            col: self.grid[0].len() - 1,
            cost: *self.grid.last().unwrap().last().unwrap(),
            direction: Direction::Right,
            straight_steps: 1,
        };
        let result = astar(
            &Pos {
                row: 0,
                col: 0,
                cost: 0,
                direction: Direction::None,
                straight_steps: 1,
            },
            |p| {
                self.successors(p)
            },
            |p| (self.distance(p, &goal) as f32 * distance_mult).round() as u32,
            |p| p.row == goal.row && p.col == goal.col,
        );

        match result {
            Some((path, cost)) => {
                // for pos in path {
                //     println!("{:?} {} ", pos.direction, pos.cost)
                // }

                return cost as usize;
            }
            None => panic!("no solution"),
        }
    }

    fn distance(&self, p1: &Pos, p2: &Pos) -> u32 {
        (p1.row.abs_diff(p2.row) + p2.col.abs_diff(p2.col)) as u32
    }

    fn successors(&self, p: &Pos) -> Vec<(Pos, u32)> {
        self.next_positions(p)
            .iter()
            .map(|pos| (*pos, pos.cost as u32))
            .collect()
    }

    fn next_positions(&self, pos: &Pos) -> Vec<Pos> {
        let mut result = Vec::new();

        if pos.row > 0
            && pos.direction != Direction::Bottom
            && (pos.direction != Direction::Top || pos.straight_steps < 3)
        {
            result.push(Pos {
                row: pos.row - 1,
                direction: Direction::Top,
                straight_steps: if pos.direction == Direction::Top || pos.direction == Direction::None {
                    pos.straight_steps + 1
                } else {
                    1
                },
                cost: self.grid[pos.row - 1][pos.col],
                ..*pos
            });
        }

        if pos.col > 0
            && pos.direction != Direction::Right
            && (pos.direction != Direction::Left || pos.straight_steps < 3)
        {
            result.push(Pos {
                col: pos.col - 1,
                direction: Direction::Left,
                straight_steps: if pos.direction == Direction::Left || pos.direction == Direction::None {
                    pos.straight_steps + 1
                } else {
                    1
                },
                cost: self.grid[pos.row][pos.col - 1],
                ..*pos
            });
        }

        if pos.row < self.grid.len() - 1
            && pos.direction != Direction::Top
            && (pos.direction != Direction::Bottom || pos.straight_steps < 3)
        {
            result.push(Pos {
                row: pos.row + 1,
                direction: Direction::Bottom,
                straight_steps: if pos.direction == Direction::Bottom || pos.direction == Direction::None
                {
                    pos.straight_steps + 1
                } else {
                    1
                },
                cost: self.grid[pos.row + 1][pos.col],
                ..*pos
            });
        }

        if pos.col < self.grid[0].len() - 1
            && pos.direction != Direction::Left
            && (pos.direction != Direction::Right || pos.straight_steps < 3)
        {
            result.push(Pos {
                col: pos.col + 1,
                direction: Direction::Right,
                straight_steps: if pos.direction == Direction::Right || pos.direction == Direction::None {
                    pos.straight_steps + 1
                } else {
                    1
                },
                cost: self.grid[pos.row][pos.col + 1],
                ..*pos
            });
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_best_path_cost() {
        let grid = game_from_str(
            "2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533",
        );
        assert_eq!(102, grid.get_best_path_cost(3f32));
    }
}
