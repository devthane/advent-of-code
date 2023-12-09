advent_of_code::solution!(4);

/// Card object to keep track of matches and copies
struct Card {
    matches: u32,
    copies: u32,
}

/// parses each number after trimming spaces.
fn parse_nums(data: &str) -> Vec<u32> {
    data.split(" ").filter_map(|s| {
        if let Ok(num) = s.trim().parse() {
            Some(num)
        } else {
            None
        }
    }).collect()
}

/// gets the number of points a card is worth in part 1
fn get_points(data: &str) -> u32 {
    let (target, numbers) = data.split_once("|").unwrap();
    let target_nums: Vec<u32> = parse_nums(target);
    let given_nums: Vec<u32> = parse_nums(numbers);

    let mut value = 0;
    for num in given_nums {
        if target_nums.contains(&num) {
            if value == 0 {
                value = 1;
            } else {
                value *= 2;
            }
        }
    }
    value
}

/// gets the number of matches on a card for part 2
fn count_matches(data: &str) -> u32 {
    let (target, numbers) = data.split_once("|").unwrap();
    let target_nums: Vec<u32> = parse_nums(target);
    let given_nums: Vec<u32> = parse_nums(numbers);

    let mut value = 0;
    for num in given_nums {
        if target_nums.contains(&num) {
            value += 1;
        }
    }

    value
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for line in input.split("\n") {
        let Some((_game, data)) = line.split_once(":") else {
            continue;
        };
        total += get_points(data);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: Vec<Card> = vec![];

    // convert the lines to cards
    for line in input.split("\n") {
        let Some((_game, data)) = line.split_once(":") else {
            continue;
        };
        let matches = count_matches(data);
        cards.push(Card {
            matches,
            // we always have at least one "copy" of the card for the original
            copies: 1,
        });
    };

    let mut total: u32 = 0;
    // iterate over a ranch of indexes so we don't have ownership issues
    for i in 0..cards.len() {
        let matches = cards.get(i).unwrap().matches;
        let copies = cards.get(i).unwrap().copies;

        // do the operation once for every instance of the card
        for _ in 0..copies {
            // add a copy to as many of the following cards as the current card has matches
            for m in i + 1..i + matches as usize + 1 {
                cards.get_mut(m).unwrap().copies += 1;
            }
        }

        total += cards.get(i).unwrap().copies;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
