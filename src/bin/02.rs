advent_of_code::solution!(2);
pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    // utilizing string splitting since values are consistently positioned
    for line in input.split("\n") {
        // game index: round, round, round
        let Some((raw_game, raw_rounds)) = line.split_once(":") else {
            continue;
        };

        // check each of the rounds for values outside the limits
        let mut fail = false;
        for raw_round in raw_rounds.trim().split(";") {
            for selection in raw_round.trim().split(",") {
                let Some((count, color)) = selection.trim().split_once(" ") else {
                    continue;
                };
                if match color {
                    "red" => count.parse::<u32>().unwrap() > 12,
                    "blue" => count.parse::<u32>().unwrap() > 14,
                    "green" => count.parse::<u32>().unwrap() > 13,
                    _ => panic!("unrecognized color")
                } {
                    fail = true;
                    break;
                }
            }
        }
        if !fail {
            total += raw_game.split(" ").last().unwrap().parse::<u32>().unwrap();
        }
    };
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.split("\n") {
        let Some((_raw_game, raw_rounds)) = line.split_once(":") else {
            continue;
        };

        let mut min_green: Option<u32> = None;
        let mut min_red: Option<u32> = None;
        let mut min_blue: Option<u32> = None;

        for raw_round in raw_rounds.trim().split(";") {
            for selection in raw_round.trim().split(",") {
                let Some((raw_count, color)) = selection.trim().split_once(" ") else {
                    continue;
                };
                // we take a mutable reference to reduce code repetition
                let min = match color {
                    "red" => &mut min_red,
                    "blue" => &mut min_blue,
                    "green" => &mut min_green,
                    _ => panic!("unrecognized color")
                };
                let count = raw_count.parse::<u32>().unwrap();
                if min.is_none() || count > min.unwrap() {
                    min.replace(count);
                }
            }
        }

        total += min_green.unwrap() * min_red.unwrap() * min_blue.unwrap();
    };
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2286));
    }
}
