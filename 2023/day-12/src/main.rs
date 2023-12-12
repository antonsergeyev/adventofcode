use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    println!("{}", count_arrangements(parse(&input)));
}

#[derive(Debug, Clone, Copy)]
enum Item {
    Ok,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone)]
struct Line {
    items: Vec<Item>,
    damaged_groups: Vec<usize>,
}

fn count_arrangements(lines: Vec<Line>) -> usize {
    lines.iter().map(|l| get_arrangements(l).len()).sum()
}

fn parse(s: &str) -> Vec<Line> {
    s.lines()
        .map(|line| {
            let mut parts = line.trim().split_whitespace();

            Line {
                items: parts
                    .next()
                    .unwrap()
                    .trim()
                    .split("")
                    .filter(|c| !c.is_empty())
                    .map(|c| {
                        if c == "." {
                            Item::Ok
                        } else if c == "?" {
                            Item::Unknown
                        } else if c == "#" {
                            Item::Damaged
                        } else {
                            panic!("invalid symbol {}", c);
                        }
                    })
                    .collect(),
                damaged_groups: parts
                    .next()
                    .unwrap()
                    .trim()
                    .split(",")
                    .filter(|c| !c.is_empty())
                    .map(|c| c.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn get_arrangements(line: &Line) -> Vec<Vec<Item>> {
    let mut result: &mut Vec<Vec<Item>> = &mut Vec::new();
    build_arrangements(line, 0, result, vec![], 0, 0);

    return result.to_vec();
}

fn build_arrangements<'a>(
    line: &Line,
    item_idx: usize,
    mut result: &'a mut Vec<Vec<Item>>,
    mut current_arrangement: Vec<Item>,
    mut consecutive_damaged_count: usize,
    mut damaged_group_idx: usize,
) {
    if item_idx == line.items.len() {
        if damaged_group_idx < line.damaged_groups.len() - 1 {
            // println!("impossible arrangement - not all damage groups added {:?}", current_arrangement);
        } else if (consecutive_damaged_count < line.damaged_groups[damaged_group_idx]) {
        //    println!("impossible arrangement - need to insert more damaged elements {:?}", current_arrangement);
        } else {
            // println!("arrangement completed {:?}", current_arrangement);
            result.push(current_arrangement);
        }

        return;
    }

    let item = line.items[item_idx];
    let is_next_group = consecutive_damaged_count == line.damaged_groups[damaged_group_idx];
    let mut is_damaged_allowed = true;
    let mut is_ok_allowed = consecutive_damaged_count == 0;

    if is_next_group {
        if damaged_group_idx < line.damaged_groups.len() - 1 {
            consecutive_damaged_count = 0;
            damaged_group_idx += 1;
        }

        is_damaged_allowed = false;
        is_ok_allowed = true;
    }

    match item {
        Item::Ok => {
            if !is_ok_allowed {
                return;
            }

            current_arrangement.push(item);
            build_arrangements(
                line, 
                item_idx + 1, 
                result, 
                current_arrangement, 
                consecutive_damaged_count, 
                damaged_group_idx
            );
        }
        Item::Damaged => {
            if !is_damaged_allowed {
                return;
            }

            current_arrangement.push(item);
            build_arrangements(
                line,
                item_idx + 1,
                result,
                current_arrangement,
                consecutive_damaged_count + 1,
                damaged_group_idx,
            );
        }
        Item::Unknown => {
            if is_ok_allowed {
                let mut with_ok = current_arrangement.clone();
                with_ok.push(Item::Ok);

                build_arrangements(
                    line, 
                    item_idx + 1, 
                    result, 
                    with_ok, 
                    consecutive_damaged_count, 
                    damaged_group_idx
                );
            }

            if is_damaged_allowed {
                let mut with_damaged = current_arrangement.clone();
                with_damaged.push(Item::Damaged);
                build_arrangements(
                    line,
                    item_idx + 1,
                    result,
                    with_damaged,
                    consecutive_damaged_count + 1,
                    damaged_group_idx,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_arrangements() {
        let line = Line {
            items: vec![
                Item::Unknown,
                Item::Unknown,
                Item::Unknown,
                Item::Ok,
                Item::Damaged,
                Item::Damaged,
                Item::Damaged,
            ],
            damaged_groups: vec![1, 1, 3],
        };
        assert_eq!(
            "[[Damaged, Ok, Damaged, Ok, Damaged, Damaged, Damaged]]",
            format!("{:?}", get_arrangements(&line))
        );
    }

    #[test]
    fn test_get_arrangements_2() {
        let line = Line {
            items: vec![
                Item::Ok,
                Item::Unknown,
                Item::Unknown,
                Item::Ok,
                Item::Ok,
                Item::Unknown,
                Item::Unknown,
                Item::Ok,
                Item::Ok,
                Item::Ok,
                Item::Unknown,
                Item::Damaged,
                Item::Damaged,
                Item::Ok,
            ],
            damaged_groups: vec![1, 1, 3],
        };
        let arrs = get_arrangements(&line);
        assert_eq!(4, arrs.len());
    }

    #[test]
    fn test_count_arrangements() {
        let lines = parse(
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1",
        );
        assert_eq!(1, get_arrangements(&lines[0]).len());
        assert_eq!(4, get_arrangements(&lines[1]).len());
        assert_eq!(1, get_arrangements(&lines[2]).len());
        assert_eq!(1, get_arrangements(&lines[3]).len());
        assert_eq!(4, get_arrangements(&lines[4]).len());
        assert_eq!(10, get_arrangements(&lines[5]).len());
        let count = count_arrangements(lines);
        assert_eq!(21, count);
    }
}
