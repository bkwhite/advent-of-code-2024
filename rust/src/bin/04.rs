use std::collections::HashMap;
use std::slice::Iter;
use std::usize;

use aho_corasick::AhoCorasick;

advent_of_code::solution!(4);

fn get_char_at_point(point: (usize, usize), horizontals: &Vec<String>) -> Option<&str> {
    match horizontals.get(point.1) {
        Some(line) => line.get(point.0..point.0 + 1),
        None => None,
    }
}

fn check_neighbors(
    point: &Point,
    points_map: &HashMap<String, char>,
    horizontal_limit: usize,
    vertical_limit: usize,
    target: &str,
    char_index: usize,
    path: &mut Vec<Point>,
) -> bool {
    if let Some(target_char) = target.get(char_index..char_index + 1) {
        let mut results = point.check_neighbors_for(
            target_char.chars().next().unwrap(),
            char_index,
            points_map,
            horizontal_limit,
            vertical_limit,
        );

        for result in results.iter() {
            if char_index == target.len() - 1 {
                println!("target_c: {}, {:?}", target_char, path);
                return true;
            } else {
                //println!("p: {:?}, tc: {}", result.1, target_char);
                path.push(point.clone());

                return check_neighbors(
                    &result.point,
                    points_map,
                    horizontal_limit,
                    vertical_limit,
                    target,
                    char_index + 1,
                    path,
                );
            }
        }
    }

    return false;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut points_map: HashMap<String, char> = HashMap::new();

    let xmas_ac = AhoCorasick::new(&["XMAS", "SAMX"]).unwrap();

    let mut occurrences: u32 = 0;
    let mut hor_occ_count = 0;
    let mut ver_occ_count = 0;
    let mut horizontal_lines: Vec<String> = [].to_vec();
    let mut vertical_lines: Vec<String> = [].to_vec();

    for (line_index, line) in input.lines().enumerate() {
        for (char_index, c) in line.chars().enumerate() {
            points_map.insert(format!("{},{}", char_index, line_index), c);
        }
    }

    let horizontal_line_count = horizontal_lines.len();
    let vertical_line_count = vertical_lines.len();

    for line in vertical_lines.clone().into_iter() {
        // count verticals
        occurrences += xmas_ac.find_overlapping_iter(line.as_str()).count() as u32;
        ver_occ_count += xmas_ac.find_overlapping_iter(line.as_str()).count() as u32;
        println!("{}", line);
    }

    for x in 0..horizontal_line_count {
        for y in 0..vertical_line_count {
            //println!("point: {:?}", (x, y));

            let mut path = vec![];
            let point = Point { x, y };

            let result = check_neighbors(
                &point,
                &points_map,
                horizontal_line_count,
                vertical_line_count,
                "XMAS",
                0,
                &mut path,
            );

            if result {
                occurrences += 1
            }
        }
    }

    println!("hor c: {}", hor_occ_count);
    println!("ver c: {}", vertical_line_count);

    Some(occurrences)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ];
        DIRECTIONS.iter()
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}

struct Result {
    point: Point,
    direction: Direction,
    char_index: usize,
}

#[allow(dead_code)]
impl Point {
    pub(crate) fn from_tuple(coords: (usize, usize)) -> Self {
        Point {
            x: coords.0,
            y: coords.1,
        }
    }

    pub fn to_str(&self) -> String {
        format!("{},{}", self.x, self.y)
    }

    pub fn get_neighbor(
        &self,
        direction: &Direction,
        horizontal_limit: usize,
        vertical_limit: usize,
    ) -> Option<Point> {
        match direction {
            Direction::Up => {
                if self.y - 1 < 0 {
                    return None;
                }

                Some(Point {
                    x: self.x,
                    y: self.y - 1,
                })
            }
            Direction::Down => {
                if self.y + 1 < vertical_limit {
                    return None;
                }

                Some(Point {
                    x: self.x,
                    y: self.y + 1,
                })
            }
            Direction::Left => {
                if self.x - 1 < 0 {
                    return None;
                }

                Some(Point {
                    x: self.x - 1,
                    y: self.y,
                })
            }
            Direction::Right => {
                if self.x + 1 < horizontal_limit {
                    return None;
                }

                Some(Point {
                    x: self.x + 1,
                    y: self.y,
                })
            }
            Direction::UpLeft => {
                if self.x - 1 < 0 || self.y - 1 < 0 {
                    return None;
                }

                Some(Point {
                    x: self.x - 1,
                    y: self.y - 1,
                })
            }
            Direction::UpRight => {
                if self.x + 1 > horizontal_limit || self.y - 1 < 0 {
                    return None;
                }

                Some(Point {
                    x: self.x + 1,
                    y: self.y - 1,
                })
            }
            Direction::DownLeft => {
                if self.x - 1 < 0 || self.y + 1 > vertical_limit {
                    return None;
                }

                Some(Point {
                    x: self.x - 1,
                    y: self.y + 1,
                })
            }
            Direction::DownRight => {
                if self.x + 1 > horizontal_limit || self.y + 1 > vertical_limit {
                    return None;
                }

                Some(Point {
                    x: self.x + 1,
                    y: self.y + 1,
                })
            }
            _ => panic!("Invalid direction"),
        }
    }

    pub fn check_neighbors_for(
        &self,
        target: char,
        char_index: usize,
        point_map: &HashMap<String, char>,
        horizontal_limit: usize,
        vertical_limit: usize,
    ) -> Vec<Result> {
        let mut results: Vec<Result> = vec![];

        for direction in Direction::iterator() {
            println!("{:?}", direction);
            if let Some(neighbor) = self.get_neighbor(direction, horizontal_limit, vertical_limit) {
                if let Some(value) = point_map.get(&neighbor.to_str()) {
                    if target == *value {
                        results.push(Result {
                            point: neighbor,
                            direction: direction.clone(),
                            char_index,
                        });
                    }
                }
            }
        }

        results
    }
}
