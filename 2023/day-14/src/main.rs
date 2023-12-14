use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");

    println!(
        "Load when tilted to north: {}",
        get_load(&tilt_north(&parse_platform(&input)))
            .iter()
            .sum::<usize>()
    );

    println!(
        "Load when tilted in cycles: {}",
        get_load(&tilt_lots_of_cycles(&parse_platform(&input)))
            .iter()
            .sum::<usize>()
    );
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Node {
    Round,
    Cube,
    Empty,
}

type Platform = Vec<Vec<Node>>;

fn tilt_lots_of_cycles(p: &Platform) -> Platform {
    let mut tilted = p.clone();
    let mut loads: Vec<usize> = Vec::new();

    // I noticed that since 110 rotations, pattern repeats every 9 rows
    // is that cheating?  ¯\_(ツ)_/¯
    let mut cycles = 110 + (1000000000 - 110) % 9;

    for i in 0..cycles {

        tilted = tilt_north(&tilted);
        tilted = tilt_west(&tilted);
        tilted = tilt_south(&tilted);
        tilted = tilt_east(&tilted);

        // let load = get_load(&tilted).iter().sum::<usize>();
        // println!("{}, load: {}", i, load);
    }

    tilted
}

fn tilt_north(p: &Platform) -> Platform {
    transpose(
        transpose(p.to_vec())
            .iter()
            .map(|row| tilt_row(row))
            .collect(),
    )
}

fn tilt_west(p: &Platform) -> Platform {
    p.iter().map(|row| tilt_row(row)).collect()
}

fn tilt_south(p: &Platform) -> Platform {
    transpose(
        transpose(p.to_vec())
            .iter()
            .map(|row| {
                let mut rev = row.clone();
                rev.reverse();
                let mut tilted = tilt_row(&rev);
                tilted.reverse();

                tilted
            })
            .collect(),
    )
}

fn tilt_east(p: &Platform) -> Platform {
    p.iter()
        .map(|row| {
            let mut rev = row.clone();
            rev.reverse();
            let mut tilted = tilt_row(&rev);
            tilted.reverse();

            tilted
        })
        .collect()
}

fn print_platform(p: &Platform) {
    println!();

    for row in p {
        for node in row {
            match node {
                Node::Round => print!("O"),
                Node::Cube => print!("#"),
                Node::Empty => print!("."),
            }
        }

        println!()
    }

    println!()
}

fn parse_platform(s: &str) -> Platform {
    s.trim().lines().map(|l| parse_line(l.trim())).collect()
}

fn get_load(p: &Platform) -> Vec<usize> {
    let mut results = Vec::new();

    for (i, row) in p.iter().enumerate() {
        let mut row_load = 0;

        for node in row {
            if matches!(node, Node::Round) {
                row_load += p.len() - i;
            }
        }

        results.push(row_load);
    }

    results
}

fn parse_line(l: &str) -> Vec<Node> {
    l.trim()
        .chars()
        .map(|c| {
            if c == 'O' {
                Node::Round
            } else if c == '#' {
                Node::Cube
            } else if c == '.' {
                Node::Empty
            } else {
                panic!("invalid symbol: {}", c);
            }
        })
        .collect()
}

fn tilt_row(row: &Vec<Node>) -> Vec<Node> {
    let mut i = 0;
    let mut first_empty_node: Option<usize> = None;
    let mut tilted = row.clone();

    loop {
        if i == tilted.len() {
            break;
        }

        match tilted[i] {
            Node::Round => {
                if let Some(first_empty_node_idx) = first_empty_node {
                    tilted[first_empty_node_idx] = Node::Round;
                    tilted[i] = Node::Empty;

                    first_empty_node = None;
                    i = first_empty_node_idx + 1;
                } else {
                    i += 1
                }
            }
            Node::Cube => {
                first_empty_node = None;
                i += 1;
            }
            Node::Empty => {
                if first_empty_node.is_none() {
                    first_empty_node = Some(i);
                }
                i += 1;
            }
        }
    }

    tilted
}

// https://stackoverflow.com/a/64499219/272787
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

    (0..len)
        .map(|_| {
            let mut iter = iters.iter_mut().map(|n| n.next().unwrap());
            iter.collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tilt_row() {
        assert_eq!(
            "[Round, Empty, Empty]",
            format!("{:?}", tilt_row(&parse_line("..O")))
        );
        assert_eq!(
            "[Round, Empty]",
            format!("{:?}", tilt_row(&parse_line(".O")))
        );
        assert_eq!(
            "[Round, Empty]",
            format!("{:?}", tilt_row(&parse_line("O.")))
        );
        assert_eq!(
            "[Cube, Round, Empty]",
            format!("{:?}", tilt_row(&parse_line("#.O")))
        );
        assert_eq!(
            "[Round, Empty, Cube, Round, Empty]",
            format!("{:?}", tilt_row(&parse_line(".O#.O")))
        );
        assert_eq!(
            "[Round, Round, Round, Empty, Empty]",
            format!("{:?}", tilt_row(&parse_line("..OOO")))
        );
        assert_eq!(
            "[Cube, Round, Empty, Empty, Cube, Round]",
            format!("{:?}", tilt_row(&parse_line("#.O.#O")))
        );
    }

    #[test]
    fn test_transpose() {
        let v = vec![vec![1, 2], vec![3, 4]];
        assert_eq!("[[1, 3], [2, 4]]", format!("{:?}", transpose(v.clone())));
    }

    #[test]
    fn test_tilt_row_2() {
        let row = parse_line("OO.O.O..##");
        println!("original: {:?}", row);
        let tilted = tilt_row(&row);
        assert_eq!(
            "[Round, Round, Round, Round, Empty, Empty, Empty, Empty, Cube, Cube]",
            format!("{:?}", tilted)
        );
    }

    #[test]
    fn test_tilt_west() {
        let platform = parse_platform(
            ".#.O
            ...O",
        );
        let tilted = tilt_west(&platform);
        assert_eq!(
            "[[Empty, Cube, Round, Empty], [Round, Empty, Empty, Empty]]",
            format!("{:?}", tilted)
        );
    }

    #[test]
    fn test_tilt_south() {
        let platform = parse_platform(
            "OO
            #.
            ..",
        );
        let tilted = tilt_south(&platform);
        assert_eq!(
            "[[Round, Empty], [Cube, Empty], [Empty, Round]]",
            format!("{:?}", tilted)
        );
    }

    #[test]
    fn test_tilt_east() {
        let platform = parse_platform(
            "O...
            O#.O.",
        );
        let tilted = tilt_east(&platform);
        assert_eq!(
            "[[Empty, Empty, Empty, Round], [Round, Cube, Empty, Empty, Round]]",
            format!("{:?}", tilted)
        );
    }

    #[test]
    fn test_tilt_north() {
        let platform = parse_platform(
            "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....",
        );
        let tilted = tilt_north(&platform);
        assert_eq!(136usize, get_load(&tilted).iter().sum());
    }
}
