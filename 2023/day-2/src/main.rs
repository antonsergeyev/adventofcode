use std::{str::FromStr, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .expect("path is required: cargo run -- src/input.txt");
    let input = fs::read_to_string(path).expect("could not read file");
    let games = parse_games(input.as_str());
    let ids = ids_of_possible_games(&games, 12, 13, 14);
    println!("Sum of IDs of possible games: {}", ids.iter().sum::<u32>());

    let powers_sum: u32 = games.iter().
        map(|game| game.min_set_of_cubes().power()).sum();
    println!("Sum of the powers of minimum sets of cubes: {}", powers_sum);
}

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<CubeSet>
}

#[derive(Debug)]
struct CubeSet {
    blue: u32,
    red: u32,
    green: u32
}

fn parse_games(s: &str) -> Vec<Game> {
    s.lines().map(|line| Game::from_str(line).unwrap()).collect()
}

fn ids_of_possible_games(games: &Vec<Game>, red: u32, green: u32, blue: u32) -> Vec<u32> {
    games.iter().filter(|game| {
        game.rounds.iter().all(|round| {
            round.blue <= blue && round.green <= green && round.red <= red
        })
    }).map(|game| game.id).collect()
}

impl Game {
    fn min_set_of_cubes(&self) -> CubeSet {
        CubeSet{
            blue: self.rounds.iter().map(|r| r.blue).max().unwrap(), 
            green: self.rounds.iter().map(|r| r.green).max().unwrap(), 
            red: self.rounds.iter().map(|r| r.red).max().unwrap()
        }
    }
}

impl CubeSet {
    fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

impl FromStr for CubeSet {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round = CubeSet{blue: 0, red: 0, green: 0};

        for color_str  in s.split(",") {
            let mut count_and_color = color_str.trim().split(" ");
            let count: u32 = count_and_color.next().
                ok_or(ParseError)?.
                parse::<u32>().map_err(|_| ParseError)?;

            match count_and_color.next() {
                Some("blue") => round.blue = count,
                Some("green") => round.green = count,
                Some("red") => round.red = count,
                _ => return Err(ParseError),
            }
        }

        Ok(round)
    }
}

impl FromStr for Game {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let id: u32 = parts.next().ok_or(ParseError)?.
            trim().
            trim_start_matches("Game ").
            parse::<u32>().map_err(|_| ParseError)?;

        let mut game = Game { id: id, rounds: Vec::new() };

        for round_str in parts.next().ok_or(ParseError)?.split(";") {
            game.rounds.push(CubeSet::from_str(round_str)?);
        }

        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_str() {
        let game = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        assert_eq!(1, game.id);
        assert_eq!(3, game.rounds.len());
        assert_eq!(3, game.rounds[0].blue);
        assert_eq!(4, game.rounds[0].red);
        assert_eq!(0, game.rounds[0].green);
        assert_eq!(6, game.rounds[1].blue);
        assert_eq!(1, game.rounds[1].red);
        assert_eq!(2, game.rounds[1].green);

        let min_set = game.min_set_of_cubes();
        assert_eq!(4, min_set.red);
        assert_eq!(2, min_set.green);
        assert_eq!(6, min_set.blue);

        assert_eq!(48, min_set.power());
    }

    #[test]
    fn test_game_from_str_invalid() {
        assert!(Game::from_str("Game abc: def").is_err());
        assert!(Game::from_str("Game 123: 3 blue, 1 orange, 1 green").is_err());
    }

    #[test]
    fn test_ids_of_possible_games() {
        let games = parse_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let ids = ids_of_possible_games(&games, 12, 13, 14);
        assert_eq!(vec![1,2,5], ids);
    }
}