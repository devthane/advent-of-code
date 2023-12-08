use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use lazy_static::lazy_static;
use regex::{Regex};

lazy_static! {
    static ref SYMBOL: Regex = Regex::new(r"[0-9.]").unwrap();
}

advent_of_code::solution!(3);

/// Grid contains information and operations on a provided input grid
struct Grid<'a> {
    data: Vec<Vec<&'a str>>,
    nums: Vec<NumLoc>,
    gears: HashMap<(usize, usize), bool>,
}

impl<'a> Grid<'a> {
    /// initializes an input grid.
    fn new(input: &'a str) -> Self {
        let mut y: usize = 0;
        let mut data: Vec<Vec<&str>> = vec![vec![]];
        let mut nums: Vec<NumLoc> = vec![];
        let mut gears: HashMap<(usize, usize), bool> = HashMap::new();

        let mut x = 0;
        for c in input.graphemes(true) {
            if c == "\n" {
                y += 1;
                x = 0;
                data.push(vec![]);
                continue;
            }
            if let Ok(digit) = c.parse() {
                if let Some(last_num) = nums.last_mut() {
                    if last_num.row == y && (x == 0 || last_num.end == x - 1) {
                        last_num.append(digit);
                    } else {
                        nums.push(NumLoc::new(y, x, digit));
                    }
                } else {
                    nums.push(NumLoc::new(y, x, digit))
                }
            } else {
                if c == "*" {
                    gears.insert((x.clone(), y.clone()), true);
                }
            }
            x += 1;
            data.get_mut(y).unwrap().push(c);
        }

        let data = data.into_iter().filter(|row| {
            row.len() > 0
        }).collect();

        Self {
            data,
            gears,
            nums,
        }
    }

    /// get the character at a coordinate of the grid
    fn get(&self, x: usize, y: usize) -> Option<&'a str> {
        let Some(row) = self.data.get(y) else {
            return None;
        };
        let c = row.get(x).unwrap();
        Some(*c)
    }

    /// the max x value of the grid
    fn width(&self) -> usize {
        if let Some(row) = self.data.first() {
            row.len()
        } else {
            0
        }
    }

    /// the max height of hte grid
    fn height(&self) -> usize {
        self.data.len()
    }
}

/// NumLoc stores the location of a number in a grid and simplifies appending a digit to the number
#[derive(Clone)]
struct NumLoc {
    num: u32,
    row: usize,
    start: usize,
    end: usize,
}

impl NumLoc {
    /// creates a new NumLoc
    fn new(row: usize, index: usize, digit: u32) -> Self {
        Self {
            row,
            start: index,
            num: digit,
            end: index,
        }
    }

    /// Appends a digit to an existing NumLoc, incrementing the end point by one
    fn append(&mut self, digit: u32) {
        self.num = format!("{}{}", self.num, digit).parse().unwrap();
        self.end += 1;
    }

    /// gets the all the points around the digits.
    fn get_perimeter(&self, max_x: i32, max_y: i32) -> Vec<(usize, usize)> {
        let mut perimeter = vec![];
        for y in (self.row as i32 - 1).max(0)..(self.row as i32 + 2).min(max_y) {
            for x in (self.start as i32 - 1).max(0)..(self.end as i32 + 2).min(max_x) {
                if y == self.row as i32 && x >= self.start as i32 && x <= self.end as i32 {
                    continue;
                }
                perimeter.push((x as usize, y as usize))
            }
        }
        perimeter
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);

    // for each number search the perimeter for a symbol, adding the number to total if found
    let mut total: u32 = 0;
    for num in grid.nums.iter() {
        for (x, y) in num.get_perimeter(grid.height() as i32, grid.width() as i32) {
            let char = grid.get(x, y).unwrap();
            if !SYMBOL.is_match(char) {
                total += num.num;
                break;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);

    // store each num that is adjacent to a gear in a map so we can tell when there is more than one.
    let mut gear_nums: HashMap<(usize, usize), Vec<&NumLoc>> = HashMap::new();
    for num in grid.nums.iter() {
        for loc in num.get_perimeter(grid.height() as i32, grid.width() as i32).iter() {
            if grid.gears.get(loc).is_some() {
                if let Some(nums) = gear_nums.get_mut(loc) {
                    nums.push(num)
                } else {
                    gear_nums.insert(loc.clone(), vec![num]);
                }
            }
        }
    }

    // for each gear with more than one number, multiply the nums to get the ratio then add to total.
    let mut total: u32 = 0;
    for (_, nums) in gear_nums {
        if nums.len() > 1 {
            let mut ratio: u32 = 1;
            for num in nums {
                ratio *= num.num
            }
            total += ratio
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
