use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .expect("path is required: cargo run -- src/input.txt");
    let input = fs::read_to_string(path).expect("could not read file");

    let almanac = almanac_from_str(&input);
    println!("Lowest location: {}", almanac.get_seeds_destinations().iter().min().unwrap());
}

#[derive(Debug, Clone, Copy)]
struct Range {
    source: u64,
    dest: u64,
    len: u64,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    stages: Vec<Stage>,
}

#[derive(Debug, Clone)]
struct Stage {
    ranges: Vec<Range>,
}

impl Stage {
    fn get_destination(&self, source: u64) -> u64 {
        self.ranges.iter().find_map(|r| {
            if source >= r.source && source < r.len + r.source {
                Some(r.dest + (source - r.source))
            } else {
                None
            }
        }).unwrap_or(source)
    }
}

impl Almanac {
    fn get_destination(&self, seed: u64) -> u64 {
        self.stages.iter().fold(seed, |src, stage| {
            stage.get_destination(src)
        })
    }

    fn get_seeds_destinations(&self) -> Vec<u64> {
        self.seeds.iter().map(|seed| self.get_destination(*seed)).collect()
    }
}

fn range_from_str(s: &str) -> Range {
    let mut parts = s.split_whitespace();

    Range {
        dest: parts.next().unwrap().parse().unwrap(),
        source: parts.next().unwrap().parse().unwrap(),
        len: parts.next().unwrap().parse().unwrap(),
    }
}

fn seeds_from_str(s: &str) -> Vec<u64> {
    s.split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn almanac_from_str(s: &str) -> Almanac {
    let mut lines = s.lines();
    let seeds = seeds_from_str(lines.next().unwrap());
    let mut stages: Vec<Stage> = Vec::new();
    let mut stage = Stage { ranges: Vec::new() };

    for line in lines {
        let trimmed = line.trim();

        if trimmed == "" {
            if stage.ranges.len() > 0 {
                stages.push(stage);
            }

            stage = Stage { ranges: Vec::new() };

            continue;
        }

        if line.contains("map") {
            continue;
        }

        stage.ranges.push(range_from_str(line.trim()));
    }

    if stage.ranges.len() > 0 {
        stages.push(stage);
    }

    Almanac {
        seeds: seeds,
        stages: stages,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almanac_from_str() {
        let almanac = almanac_from_str(
            "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4",
        );
        assert_eq!("[79, 14, 55, 13]", format!("{:?}", almanac.seeds));
        assert_eq!(7, almanac.stages.len());

        assert_eq!(50, almanac.stages.get(0).unwrap().get_destination(98));
        assert_eq!(100500, almanac.stages.get(0).unwrap().get_destination(100500));

        assert_eq!(82, almanac.get_destination(79));

        let dests = almanac.get_seeds_destinations();
        assert_eq!(vec![82u64, 43u64, 86u64, 35u64], dests);
    }
}
