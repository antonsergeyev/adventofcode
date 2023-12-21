use std::{collections::HashMap, fs};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    let system = system_from_str(&input);
    println!("{}", system.sort_parts());
}

const MAX_VAL: usize = 4000;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Gt,
    Lt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Reject,
    Accept,
    Send(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Condition {
    prop: Option<char>,
    op: Option<Op>,
    value: Option<i32>,
    action: Action,
}

impl Action {
    fn from_string(s: &str) -> Action {
        match s {
            "A" => Action::Accept,
            "R" => Action::Reject,
            _ => Action::Send(s.to_string()),
        }
    }
}

impl Condition {
    fn from_action(action: Action) -> Self {
        Condition {
            prop: None,
            op: None,
            value: None,
            action: action,
        }
    }
}

#[derive(Debug)]
struct System {
    workflows: HashMap<String, Workflow>,
    parts: Vec<HashMap<char, i32>>,
}

impl System {
    fn combinations(&self) -> u64 {
        let mut symbols = HashMap::from([
            ('x', 0),
            ('m', 0),
            ('a', 0),
            ('s', 0),
        ]);

        for symbol in symbols.keys() {
            let mut ranges: Vec<(usize, usize)> = Vec::new();

            for w in self.workflows.values() {
                let ranges_for_worflow = w.ranges_for_symbol(*symbol);
            }
        }

        return 0
    }

    fn get_workflows_leading_to(&self, to: &str) -> Vec<&Workflow> {
        self.workflows.iter().filter_map(|(_, w)| {
            for cond in w.conditions.iter() {
                match &cond.action {
                    Action::Send(target) => if target == to {
                        return Some(w)
                    },
                    _ => {}
                }
            }

            return None
        }).collect()
    }

    fn sort_parts(&self) -> usize {
        let mut result = 0;
        for part in self.parts.iter() {
            match self.sort_part(&part) {
                Action::Accept => result += part.values().sum::<i32>() as usize,
                _ => {},
            }
        }

        return result;
    }
    fn sort_part(&self, part: &HashMap<char, i32>) -> Action {
        let mut w = self.workflows.get("in").unwrap();
        
        loop {
            match w.sort(part) {
                Action::Reject => return Action::Reject,
                Action::Accept => return Action::Accept,
                Action::Send(another_workflow) => {
                    w = self.workflows.get(&another_workflow).unwrap()
                },
            }
        }
    }
}

impl Workflow {
    fn ranges_for_symbol(&self, symbol: char) -> Vec<(usize, usize)> {
        let mut ranges: Vec<(usize, usize)> = Vec::new();

        for cond in self.conditions.iter() {
            match cond.prop {
                Some(prop) => {
                    if prop != symbol {
                        continue
                    }

                    if matches!(cond.op, Some(Op::Gt)) {
                        ranges.push((cond.value.unwrap() as usize, MAX_VAL));
                    } else if matches!(cond.op, Some(Op::Lt)) {
                        ranges.push((0, cond.value.unwrap() as usize));
                    }
                },
                None => if matches!(cond.action, Action::Accept) {
                    ranges.push((0, MAX_VAL));
                },
            }
        }

        return ranges
    }

    fn sort(&self, part: &HashMap<char, i32>) -> Action {
        for cond in self.conditions.iter() {
            match cond.prop {
                Some(prop) => {
                    if !part.contains_key(&prop) {
                        continue
                    }

                    let value = *part.get(&prop).unwrap();

                    if matches!(cond.op, Some(Op::Gt)) && value > cond.value.unwrap() {
                        return cond.action.clone()
                    }

                    if matches!(cond.op, Some(Op::Lt)) && value < cond.value.unwrap() {
                        return cond.action.clone()
                    }
                },
                None => return cond.action.clone(),
            }
        }

        panic!("invalid part")
    }
}

fn workflow_from_str2(s: &str) {
    let regex = Regex::new(r"(?P<procName>\w+)\{((?P<prop>[a-z])(?P<op>[<>])(?P<param>\d+):(?P<nextProc>\w+),?)+(?P<lastProc>\w+)\}").unwrap();

    println!("matches for {}:", s);
    
    for m in regex.captures_iter(s) {
        for mi in m.iter() {
            println!("{}", mi.unwrap().as_str())
        }
    }
    
}

fn workflow_from_str(s: &str) -> Workflow {
    let mut parts = s.split("{");

    Workflow {
        name: parts.next().unwrap().to_string(),
        conditions: parts
            .next()
            .unwrap()
            .trim_end_matches("}")
            .split(",")
            .map(|cond_str| {
                if cond_str.contains(":") {
                    let mut cond_parts = cond_str.split(":");
                    let mut cond = cond_parts.next().unwrap();
                    let action = Action::from_string(cond_parts.next().unwrap());

                    if cond.contains("<") {
                        let mut cond_parts = cond.split("<");
                        Condition {
                            prop: cond_parts.next().unwrap().chars().nth(0),
                            op: Some(Op::Lt),
                            value: Some(cond_parts.next().unwrap().parse().unwrap()),
                            action: action,
                        }
                    } else {
                        let mut cond_parts = cond.split(">");
                        Condition {
                            prop: cond_parts.next().unwrap().chars().nth(0),
                            op: Some(Op::Gt),
                            value: Some(cond_parts.next().unwrap().parse().unwrap()),
                            action: action,
                        }
                    }
                } else {
                    Condition::from_action(Action::from_string(cond_str))
                }
            })
            .collect(),
    }
}

fn part_from_str(s: &str) -> HashMap<char, i32> {
    let mut result = HashMap::new();

    for prop in s.trim_start_matches("{")
        .trim_end_matches("}")
        .split(",") {
            let mut parts = prop.split("=");

            result.insert(
                parts.next().unwrap().chars().nth(0).unwrap(), 
                parts.next().unwrap().parse().unwrap()
            );
        }

    result
}

fn system_from_str(s: &str) -> System {
    let mut result = System {
        parts: Vec::new(),
        workflows: HashMap::new(),
    };
    let mut parts = s.split("\n\n");

    for workflow_line in parts.next().unwrap().lines() {
        let workflow = workflow_from_str(workflow_line.trim());
        result.workflows.insert(workflow.name.clone(), workflow);
    }

    for part_line in parts.next().unwrap().lines() {
        result.parts.push(part_from_str(part_line.trim()));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let system = system_from_str(TEST_INPUT);
        assert_eq!(19114, system.sort_parts());
    }

    #[test]
    fn test_sort_part() {
        let system = system_from_str(TEST_INPUT);
        assert_eq!(Action::Accept, system.sort_part(&system.parts[0]));
    }

    #[test]
    fn test_workflow_sort() {
        let w1 = workflow_from_str("in{s<1351:px,qqz}");
        let w2 = workflow_from_str("qqz{s>2770:qs,m<1801:hdj,R}");
        let w3 = workflow_from_str("lnx{m>1548:A,A}");
        let p = part_from_str("{x=787,m=2655,a=1222,s=2876}");
        assert_eq!(Action::Send("qqz".to_string()), w1.sort(&p));
        assert_eq!(Action::Send("qs".to_string()), w2.sort(&p));
        assert_eq!(Action::Accept, w3.sort(&p));
    }

    #[test]
    fn test_part_from_str() {
        assert_eq!(HashMap::from([('x', 787), ('m', 2655), ('a', 1222)]), part_from_str("{x=787,m=2655,a=1222}"));
    }

    #[test]
    fn test_workflow_from_str() {
        assert_eq!(
            Workflow {
                name: "px".to_string(),
                conditions: vec![
                    Condition {
                        prop: Some('a'),
                        op: Some(Op::Lt),
                        value: Some(2006),
                        action: Action::Send("qkq".to_string())
                    },
                    Condition {
                        prop: Some('m'),
                        op: Some(Op::Gt),
                        value: Some(2090),
                        action: Action::Accept
                    },
                    Condition::from_action(Action::Send("rfg".to_string()))
                ]
            },
            workflow_from_str("px{a<2006:qkq,m>2090:A,rfg}")
        );
    }

    #[test]
    fn test_workflow_from_str2() {
        workflow_from_str2("px{a<2006:qkq,m>2090:A,rfg}");
    }
}

const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";