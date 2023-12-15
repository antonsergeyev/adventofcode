use std::{fs};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    println!("Hashsum: {}", hash_sum(input_to_strings(&input)));

    let mut hm = Hashmap::new();
    hm.apply_sequence(parse_steps(&input));
    println!("Focusing power: {}", hm.focusing_power());
}

fn input_to_strings(s: &str) -> Vec<&str> {
    s.split(",").collect()
}

fn parse_steps(s: &str) -> Vec<Step> {
    input_to_strings(s)
        .iter()
        .map(|s| {
            if s.contains("=") {
                let mut parts = s.split("=");

                Step::Add(
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().parse().unwrap(),
                )
            } else {
                Step::Remove(s.trim_end_matches("-").to_string())
            }
        })
        .collect()
}

fn hash_sum(parts: Vec<&str>) -> usize {
    parts.iter().fold(0, |acc, part| acc + hash(part))
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| {
        let mut next = acc + c as usize;
        next = next * 17;
        next = next % 256;

        next
    })
}

#[derive(Debug, Clone)]
enum Step {
    Remove(String),
    Add(String, usize),
}

#[derive(Debug)]
struct Hashmap {
    boxes: Vec<Vec<(String, usize)>>,
}

impl Hashmap {
    fn new() -> Self {
        let mut hm = Hashmap {
            boxes: Vec::with_capacity(256),
        };
        for _ in 0..256 {
            hm.boxes.push(Vec::new());
        }

        return hm;
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .fold(0, |acc_out, (box_idx, items)| {
                acc_out
                    + items
                        .iter()
                        .enumerate()
                        .fold(0, |acc_in, (lens_idx, (_, focal_length))| {
                            acc_in + (box_idx + 1) * (lens_idx + 1) * focal_length
                        })
            })
    }

    fn apply_sequence(&mut self, steps: Vec<Step>) {
        for step in steps.iter() {
            self.apply_step(step);
        }
    }

    fn apply_step(&mut self, step: &Step) {
        match step {
            Step::Remove(label) => {
                let box_idx = hash(&label);
                let pos = self.boxes[box_idx].iter().position(|i| i.0 == *label);
                if let Some(idx) = pos {
                    self.boxes[box_idx].remove(idx);
                }
            }
            Step::Add(label, value) => {
                let box_idx = hash(&label);
                let pos = self.boxes[box_idx].iter().position(|i| i.0 == *label);
                if let Some(idx) = pos {
                    self.boxes[box_idx][idx].1 = *value;
                } else {
                    self.boxes[box_idx].push((label.clone(), *value));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_apply_step() {
        let steps = parse_steps("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        let mut hm = Hashmap::new();
        hm.apply_step(&steps[0]);
        assert_eq!(r#"[("rn", 1)]"#, format!("{:?}", hm.boxes[0]));

        hm.apply_step(&steps[1]);
        hm.apply_step(&steps[2]);
        hm.apply_step(&steps[3]);

        assert_eq!(r#"[("rn", 1), ("cm", 2)]"#, format!("{:?}", hm.boxes[0]));
        assert_eq!(r#"[("qp", 3)]"#, format!("{:?}", hm.boxes[1]));
    }

    #[test]
    fn test_hashmap_focusing_power() {
        let steps = parse_steps("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        let mut hm = Hashmap::new();
        hm.apply_sequence(steps);
        assert_eq!(145, hm.focusing_power());
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
    }

    #[test]
    fn test_hash_sum() {
        assert_eq!(
            1320,
            hash_sum(input_to_strings(
                "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            ))
        );
    }

    #[test]
    fn test_input_to_steps() {
        assert_eq!(
            "[Add(\"rn\", 1), Remove(\"cm\"), Add(\"qp\", 3)]",
            format!("{:?}", parse_steps("rn=1,cm-,qp=3"))
        );
    }
}
