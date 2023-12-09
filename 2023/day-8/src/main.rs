use std::{str::FromStr, collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .expect("path is required: cargo run -- src/input.txt");
    let input = fs::read_to_string(path).expect("could not read file");

    let (directions, nodes) = parse_game(&input);
    let steps = go(directions, nodes);

    println!("{}", steps);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    value: String,
    left: String,
    right: String,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("bad input: {}", input))
        }
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(input: &str) -> Result<Node, Self::Err> {
        let mut parts = input.split("=");
        let value = parts.next().unwrap().trim();
        let mut left_and_right = parts.next().unwrap().trim().split(",");
        let left = left_and_right.next().unwrap().trim().trim_matches('(');
        let right = left_and_right.next().unwrap().trim().trim_matches(')');

        Ok(Node { value: value.to_string(), left: left.to_string(), right: right.to_string() })
    }
}

fn parse_game(s: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    let mut lines = s.lines();
    let directions = lines.next().unwrap().
        split("").
        filter(|c|!c.is_empty()).
        map(|d| d.parse::<Direction>().unwrap()).collect();
    
    let nodes: HashMap<String, Node> = lines.filter(|l|!l.is_empty()).
        map(|l| {
            let node = l.parse::<Node>().unwrap();

            (node.value.clone(), node)
        }).collect();

    (directions, nodes)
}

fn go(
    directions: Vec<Direction>, 
    nodes: HashMap<String, Node>
) -> u32 {
    let mut steps = 0;
    let mut current_nodes: Vec<String> = nodes.clone().keys().into_iter().
        filter(|s| s.ends_with("A")).
        map(|s| s.clone()).
        collect();

    for d in directions.iter().cycle() {
        let mut z_count = 0;
        let mut next_nodes:Vec<String> = Vec::new();

        for (i, node_name) in current_nodes.iter().enumerate() {
            let node = nodes.get(node_name).unwrap();
            let next_node = if d == &Direction::Left {
                nodes.get(&node.left).unwrap()
            } else {
                nodes.get(&node.right).unwrap()
            };

            if next_node.value.ends_with("Z") {
                z_count += 1
            }

            next_nodes.push(next_node.value.clone());
        }

        // println!("{}/{}", z_count, current_nodes.len());
        steps += 1;

        if z_count == current_nodes.len() {
            break
        }

        current_nodes = next_nodes;
    }
    
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go() {
        let (directions, nodes) = parse_game(GAME_3);
        let steps = go(directions, nodes);
        assert_eq!(6, steps);
    }

    #[test]
    fn test_parse_game() {
        let (directions, nodes) = parse_game(GAME_3);
        assert_eq!(2, directions.len());
        assert_eq!(8, nodes.len());
    }

    #[test]
    fn test_node_from_str() {
        assert_eq!(Node{
            value: "AAA".to_string(),
            left: "BBB".to_string(),
            right: "CCC".to_string(),
        }, "AAA = (BBB, CCC)".parse().unwrap());
    }

    #[test]
    fn test_direction_from_str() {
        assert_eq!(Direction::Left, "L".parse().unwrap());
        assert_eq!(Direction::Right, "R".parse().unwrap());
    }
}

const GAME_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const GAME_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const GAME_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";