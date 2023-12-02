use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .expect("path is required: cargo run -- src/input.txt");
    let readme = fs::read_to_string(path).expect("could not read file");
    println!("{}", get_text_value(readme.as_str()));
}

const WORDS2DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_line_value(s: &str) -> u32 {
    let bytes = s.as_bytes();
    // all digits in a string are collected into a vec
    // TODO may just look at the beginning and the end ignoring everything in between
    let mut digits: Vec<u32> = Vec::new();

    for (i, c) in bytes.iter().enumerate() {
        if c.is_ascii_digit() {
            digits.push((*c as char).to_digit(10).unwrap());
        }

        let word_position = WORDS2DIGITS.iter().position(|word| {
            if i + word.len() > bytes.len() {
                return false;
            }

            return word.as_bytes() == &bytes[i..(i+word.len())];
        });
        if let Some(word_index) = word_position {
            digits.push(word_index as u32 + 1);
        }
    }

    return digits.first().unwrap_or(&0) * 10 + 
        digits.last().unwrap_or(&0);
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

        assert_eq!(11, get_line_value("one"));
        assert_eq!(12, get_line_value("onetwo"));

        assert_eq!(24, get_line_value("xtwone3four"));
        assert_eq!(29, get_line_value("two1nine"));
        assert_eq!(83, get_line_value("8wo3"));
        assert_eq!(83, get_line_value("eightwo3"));
        assert_eq!(83, get_line_value("eightwothree"));
        assert_eq!(42, get_line_value("4nineeightseven2"));
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
