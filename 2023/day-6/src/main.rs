use std::{env, fs, cmp, collections::HashMap};

fn main() {
    let input = "Time:        63     78     94     68
    Distance:   411   1274   2047   1035";
    let ways_to_win_1 = ways_to_win_in_races(parse_races(input));
    println!("Part 1: {}", ways_to_win_1);

    let input2 = "Time:        63789468
    Distance:   411127420471035";
    let ways_to_win_2 = ways_to_win_in_races(parse_races(input2));
    println!("Part 2: {}", ways_to_win_2);
}

#[derive(Debug, Clone, Copy)]
struct Race(i64, i64);

fn parse_races(s: &str) -> Vec<Race> {
    let mut lines = s.lines();
    let times: Vec<i64> = lines.next().unwrap().split(":").nth(1).unwrap().split_whitespace().map(|t| t.trim().parse().unwrap()).collect();
    let distances: Vec<i64> = lines.next().unwrap().split(":").nth(1).unwrap().split_whitespace().map(|t| t.trim().parse().unwrap()).collect();

    times.into_iter().zip(distances.into_iter()).map(|pair| Race(pair.0, pair.1) ).collect()
}


// example: time = 7, record distance = 9
// time to hold (tth) from 0 to time:
// tth = speed = 0, time left = 7-0 = 7, distance = 7*0 = 0
// tth = speed = 1, time left = 7-1 = 6, distance = 6*1 = 6
// tth = speed = 2, time left = 7-2 = 5, distance = 5*2 = 10
// tth = speed = 3, time left = 7-3 = 4, distance = 4*3 = 12
// tth = speed = 4, time left = 7-4 = 3, distance = 3*4 = 12
// tth = speed = 5, time left = 7-5 = 2, distance = 2*5 = 10
fn get_hold_duration_range_above_record(time: i64, record_distance: i64) -> (f64, f64) {
    // let x = time to hold = speed = 1, y = distance = (time - speed) * speed

    // ax^2+bx+c=y
    // if x=0 then y=0 => c = 0, ax^2+bx = y
    // if x=1 then y=(time-x)*x =              time-1  = a+b     => a = time-1-b
    // if x=2 then y=(time-x)*x = (time-2)*2 = 2time-4 = 4a+2b

    // 2time-4 = 4a+2b
    // 2time-4 = 4time-4-4b + 2b
    // time-2 = 2time-2-2b  + b = 2time-2-b
    // time-2time = -2-b+2
    // -time = -b
    // time = b
    // a = time-1-b = -1
    // -x^2+time*x = 0

    let b = time as f64;
    let a = -1f64;

    let d = b*b-(4f64*a*(-record_distance as f64));
    let sqrt_d = (d as f64).sqrt();
    let x1 = (-b+sqrt_d)/-2f64;
    let x2 = (-b-sqrt_d)/-2f64;

    println!(
        "time={}, record_distance={}, b={}, d={}, sqrtd={}, x1={}, x2={}", 
        time, record_distance, b, d, sqrt_d, x1, x2
    );

    (x1, x2)
}

fn ways_to_win(x1: f64, x2: f64) -> u64 {
    let ways_float = (x2-x1);
    let ways_int = if ways_float.fract() > 0f64 { ways_float.round().trunc() as u64 } else { ways_float.round().trunc() as u64 - 1 };
    println!(
        "ways to win between {} and {}: {} float or {} int, fract {}", 
        x1, x2, ways_float, ways_int, ways_float.fract()
    );
    ways_int
    
}

fn ways_to_win_in_races(races: Vec<Race>) -> u64 {
    races.iter().map(|r| {
        let duration_range = get_hold_duration_range_above_record(r.0, r.1);
        ways_to_win(duration_range.0, duration_range.1)
    }).product()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_races() {
        let races = parse_races("Time:      7  15   30
        Distance:  9  40  200");
        assert_eq!("[Race(7, 9), Race(15, 40), Race(30, 200)]", format!("{:?}", races));

        assert_eq!(4*8*9, ways_to_win_in_races(races));
    }

    #[test]
    fn test_get_hold_duration_range_above_record() {
        let (mut x1, mut x2) = get_hold_duration_range_above_record(7, 9);
        // assert_eq!((1.6972244, 5.3027754), (x1, x2));
        assert_eq!(4, ways_to_win(x1, x2));

        (x1, x2) = get_hold_duration_range_above_record(15, 40);
        // assert_eq!((-0.5311289, 7.531129), (x1, x2));
        assert_eq!(8, ways_to_win(x1, x2));

        (x1, x2) = get_hold_duration_range_above_record(30, 200);
        // assert_eq!((-1.5, 8.5), (x1, x2));
        assert_eq!(9, ways_to_win(x1, x2));
    }
}