use std::{fs, str::FromStr, fmt::{self, write, Display}};


fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    let game: Game = input.parse().unwrap();
    let route = game.get_route();
    println!("Route: {:?}", route);
    println!("Farthest point: {:?}", game.route_farthest_point(route));
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Start,
    Ground,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

#[derive(Debug)]
struct Game {
    tiles: Vec<Vec<Tile>>,
    start: (i32, i32),
}

impl FromStr for Game {
    type Err = String;

    fn from_str(input: &str) -> Result<Game, Self::Err> {
        let mut game = Game{start:(0, 0), tiles: Vec::new()};

        for (row, line) in input.trim().lines().enumerate().into_iter() {
            let tiles: Vec<Tile> = line.trim().split("").
                filter(|t| !t.is_empty()).
                map(|t| t.parse().unwrap()).collect();

            if let Some(start_col) = tiles.iter().position(|t| matches!(t, Tile::Start)) {
                game.start = (row as i32, start_col as i32) ;
            }

            game.tiles.push(tiles);
        }

        Ok(game)
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(input: &str) -> Result<Tile, Self::Err> {
        match input {
            "S" => Ok(Tile::Start),
            "." => Ok(Tile::Ground),
            "|" => Ok(Tile::NS),
            "-" => Ok(Tile::EW),
            "L" => Ok(Tile::NE),
            "J" => Ok(Tile::NW),
            "7" => Ok(Tile::SW),
            "F" => Ok(Tile::SE),
            _ => Err(format!("bad input: {}", input))
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Tile::Start => write!(f, "S"),
            Tile::Ground => write!(f, "."),
            Tile::NS => write!(f, "|"),
            Tile::EW => write!(f, "-"),
            Tile::NE => write!(f, "L"),
            Tile::NW => write!(f, "J"),
            Tile::SW => write!(f, "7"),
            Tile::SE => write!(f, "F"),
        }
    }
}

impl Game {
    fn route_farthest_point(&self, route: Vec<Tile>) -> i32 {
        if route.len() % 2 == 0 {
            return route.len() as i32 / 2
        }

        return route.len() as i32 / 2 + 1;
    }
    fn get_route(&self) -> Vec<Tile> {
        for x in -1..=1 {
            for y in -1..=1 {
                if x == y {
                    continue;
                }

                if !self.can_enter(self.start, (self.start.0+x, self.start.1+y)) {
                    // println!("cant move from start tile {:?} to {:?}", self.start, (self.start.0+x, self.start.1+y));
                    continue;
                }

                // println!("moving from start tile {:?} to {:?}", self.start, (self.start.0+x, self.start.1+y));
                let candidate_route = self.walk(self.start, (self.start.0+x, self.start.1+y));

                if candidate_route.len() > 0 && matches!(candidate_route.last().unwrap(), Tile::Start) {
                    return candidate_route 
                } else {
                    println!("---- done with candidate route starting from {:?}----", (self.start.0+x, self.start.1+y));
                }
            }
        }
        
        panic!("no route found")
    }

    fn walk(&self, mut prev_pos: (i32, i32), mut pos: (i32, i32)) -> Vec<Tile> {
        let mut result: Vec<Tile> = Vec::new();

        loop {
            let next_tile_coords = self.get_next_tile_coords(prev_pos, pos);

            match next_tile_coords {
                Some(coords) => {   
                    let next_tile = self.tiles[coords.0 as usize][coords.1 as usize];
                    result.push(next_tile);

                    if matches!(next_tile, Tile::Start) {
                        println!(
                            "made a loop {:?} {:?} to {:?} {:?}",
                            pos,
                            self.tiles[pos.0 as usize][pos.1 as usize],
                            coords,
                            next_tile
                        );     

                        break
                    }

                    // println!(
                    //     "moving from {:?} {:?} to {:?} {:?}",
                    //     pos,
                    //     self.tiles[pos.0 as usize][pos.1 as usize],
                    //     coords,
                    //     next_tile
                    // );     

                    prev_pos = pos;
                    pos = coords;
                },
                None => {
                    println!(
                        "cant move from {:?} {}",
                        pos,
                        self.tiles[pos.0 as usize][pos.1 as usize],
                    );     

                    // self.print_around(pos.0, pos.1);

                    break
                },
            }
        }

        result
    }

    fn get_next_tile_coords(&self, prev_pos: (i32, i32), pos: (i32, i32)) -> Option<(i32, i32)> {
        let current_tile = self.tiles[pos.0 as usize][pos.1 as usize];

        let candidate_pos = match &current_tile {
            // |        north -> south                                      south -> north
            Tile::NS => if prev_pos.0 < pos.0 { Some((pos.0+1, pos.1)) } else { Some((pos.0-1, pos.1))},
            // -        west -> east                                        east -> west
            Tile::EW => if prev_pos.1 < pos.1 { Some((pos.0, pos.1+1)) } else { Some((pos.0, pos.1-1))},
            // L        north -> east                                       east -> north
            Tile::NE => if prev_pos.0 < pos.0 { Some((pos.0, pos.1+1)) } else { Some((pos.0-1, pos.1))},
            // J        north -> west                                       west -> north
            Tile::NW => if prev_pos.0 < pos.0 { Some((pos.0, pos.1-1)) } else { Some((pos.0-1, pos.1))},
            // 7        south -> west                                       west -> south
            Tile::SW => if prev_pos.0 > pos.0 { Some((pos.0, pos.1-1)) } else { Some((pos.0+1, pos.1))},
            // F        south -> east                                       east -> south
            Tile::SE => if prev_pos.0 > pos.0 { Some((pos.0, pos.1+1)) } else { Some((pos.0+1, pos.1))},
            _ => None,
        };

        println!("candidate pos from {:?} {:?}: {:?}", current_tile, pos, candidate_pos);

        match candidate_pos {
            Some(next_pos) if self.can_enter(pos, next_pos) => Some(next_pos),
            _ => None,
        }
    }

    fn can_enter(&self, from: (i32, i32), to: (i32, i32)) -> bool {
        if to.0 < 0 || to.1 < 0 || to.0 as usize >= self.tiles.len() || to.1 as usize >= self.tiles[0].len() {
            // println!("position {:?} outside of map", to);
            return false
        }
        
        // println!(
        //     "trying to enter tile {:?} from {} {} to {} {}", 
        //     self.tiles[to.0 as usize][to.1 as usize], from.0, from.1, to.0, to.1
        // );
        self.tiles[to.0 as usize][to.1 as usize].can_enter_from_offset(from.0 - to.0, from.1 - to.1)
    }

    fn print_around(&self, row_index: i32, col_index: i32) {
        println!();
        print!("   ");

        for i in col_index-1..col_index+2 {
            print!("{:<3}", i);
        }

        println!();

        for (r, row) in self.tiles.iter().skip(row_index as usize -1).take(3).enumerate() {
            print!("{:<3}", row_index + r as i32 - 1);

            for (c, tile) in row.iter().skip(col_index as usize - 1).take(3).enumerate() {
                print!("{:>3}  ", tile);
            }

            println!();
        }
    }
}

impl Tile {
    fn can_enter_from_offset(&self, row: i32, col: i32) -> bool {
        match &self {
            // |
            Tile::NS => col == 0 && (row == -1 || row == 1),
            // - 
            Tile::EW => {
                return row == 0 && (col == -1 || col == 1);
            }
            // L 
            Tile::NE => {
                return (col == 0 && row == -1) || (col == 1 && row == 0);
            }
            // J
            Tile::NW => (col == 0 && row == -1) || (col == -1 && row == 0),
            // 7
            Tile::SW => {
                return (col == -1 && row == 0) || (col == 0 && row == 1);
            }
            // F
            Tile::SE => (col == 1 && row == 0) || (col == 0 && row == 1),
            Tile::Start => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_get_route() {
        let game: Game = GAME_1.parse().unwrap();
        let result = game.get_route();
        assert_eq!("[SW, NS, NW, EW, NE, NS, Start]", format!("{:?}", result));
        assert_eq!(4, game.route_farthest_point(result));
    }

    #[test]
    fn test_game_get_route_2() {
        let game: Game = GAME_2.parse().unwrap();
        let result = game.get_route();
        assert_eq!("[NS, SE, EW, SW, NE, SW, NW, SE, NW, EW, Start]", format!("{:?}", result));
        assert_eq!(6, game.route_farthest_point(result));
    }

    #[test]
    fn test_game_walk() {
        let game: Game = GAME_1.parse().unwrap();

        let mut result = game.walk((1, 1), (0, 1));
        assert_eq!("[]", format!("{:?}", result));

        let mut result = game.walk((1, 1), (1, 2));
        assert_eq!("[SW, NS, NW, EW, NE, NS, Start]", format!("{:?}", result));
    }

    #[test]
    fn test_game_from_str() {
        let game: Game = GAME_1.parse().unwrap();
        assert_eq!(5, game.tiles.len());
        assert_eq!(5, game.tiles[0].len());
        assert_eq!("[Ground, Start, EW, SW, Ground]", format!("{:?}", game.tiles[1]));
        assert_eq!((1, 1), game.start);
    }

    #[test]
    fn test_can_enter() {
        let game: Game = GAME_1.parse().unwrap();
        assert_eq!(false, game.can_enter((1, 1), (0, 1)));
        assert_eq!(false, game.can_enter((1, 1), (0, 0)));
        assert_eq!(true, game.can_enter((1, 1), (1, 2)));
        assert_eq!(true, game.can_enter((1, 1), (2, 1)));
    }
}

const GAME_1: &str = "
.....
.S-7.
.|.|.
.L-J.
.....";

const GAME_2: &str = "
.F-7..
.|.L7.
.|.FJ.
.S-J..";

// const GAME_2: &str = "
// ......
// .F-7..
// .|.|..
// .|.L7.
// .|..|.
// .|..J.
// .S-J..
// ......";

const GAME_3: &str = "
.....
.F-7.
.|.|.
.S-J.
.....";

const GAME_COMPLEX: &str = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";