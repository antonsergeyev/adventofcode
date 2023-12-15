use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    println!("{}", hash_sum(parse_input(&input)));
}

fn parse_input(s: &str) -> Vec<&str> {
    s.split(",").collect()
}

fn hash_sum(parts: Vec<&str>) -> u32 {
    parts.iter().fold(0, |acc, part| {
        acc + hash(part)
    })
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| {
        let mut next = acc + c as u32;
        next = next * 17;
        next = next % 256;

        next
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
    }

    #[test]
    fn test_hash_sum() {
        assert_eq!(1320, hash_sum(parse_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")));
    }
}
