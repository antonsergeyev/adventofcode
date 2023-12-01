use std::fs;
use std::env;

fn main() {
    let args: Vec<String>  = env::args().collect();
    let path = args.get(1).expect("path is required: cargo run -- src/input.txt");
    let readme = fs::read_to_string(path).expect("could not read file");
    println!("{}", get_text_value(readme.as_str()));
}

fn get_line_value(s: &str) -> u32 {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();

    digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0)
}

fn get_text_value(s: &str) -> u32 {
    s.lines().map(|line| get_line_value(line)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_value() {
        assert_eq!(12, get_line_value("t1est2"));
        assert_eq!(55, get_line_value("5"));
        assert_eq!(99, get_line_value("asdfas9"));
        assert_eq!(0, get_line_value("asdfadsf"));
        assert_eq!(77, get_line_value("treb7uchet"));
    }

    #[test]
    fn test_get_text_value() {
        assert_eq!(
            142,
            get_text_value(
                "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            )
        );
    }
}
