use lazy_static::lazy_static;
use regex::{Regex};

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"\d").unwrap();

    static ref DIGIT_PATTERN: Regex = Regex::new(r"\d$").unwrap();
    static ref WORD_PATTERN: Regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)$").unwrap();
}

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");

    let mut total: u32 = 0;
    for line in lines {
        let matches: Vec<u32> = PATTERN.find_iter(line).map(|num| {
            num.as_str().parse::<u32>().unwrap()
        }).collect();

        if matches.len() == 0 {
            continue;
        }

        // create number from first and last number and add to the total.
        // this conveniently duplicates the number if there is only one match, as desired.
        total += format!("{}{}", matches.first().unwrap(), matches.last().unwrap()).parse::<u32>().unwrap()
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");

    let mut total: u32 = 0;
    for line in lines {
        let mut matches: Vec<u32> = vec![];
        let mut buf = String::new();
        for char in line.chars() {
            // regex crate does not support lookahead/lookbehind so it is necessary to search
            // after adding each character to handle overlap.
            buf.push(char);
            if let Some(m) = DIGIT_PATTERN.find(buf.as_str()) {
                if let Ok(num) = m.as_str().parse() {
                    matches.push(num);
                    continue;
                }
            }
            if let Some(m) = WORD_PATTERN.find(buf.as_str()) {
                matches.push(match m.as_str() {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => panic!("unrecognized word")
                })
            }
        }
        if matches.len() == 0 {
            continue;
        }
        total += format!("{}{}", matches.first().unwrap(), matches.last().unwrap()).parse::<u32>().unwrap();
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
