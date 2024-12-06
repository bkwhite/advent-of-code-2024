use std::collections::HashMap;
use std::slice::Iter;
use std::usize;

advent_of_code::solution!(4);

fn check_neighbors(
    point: &Point,
    path: &mut Vec<Point>,
    found_count: &mut usize,
    target: &str,
    char_index: usize,
    last_direction: Option<Direction>,
    points_map: &HashMap<String, char>,
    limit: usize,
) {
    if let Some(target_char) = target.chars().nth(char_index) {
        let results = point.check_neighbors_for(target_char, points_map, limit);

        for result in results.iter() {
            if target_char == result.character && char_index == 0 {
                let mut new_path = path.clone();
                new_path.push(result.point.clone());

                check_neighbors(
                    &result.point,
                    &mut new_path,
                    found_count,
                    target,
                    char_index + 1,
                    Some(result.direction.clone()),
                    points_map,
                    limit,
                );
            }

            if let Some(last_direction) = last_direction {
                if last_direction == result.direction {
                    path.push(result.point.clone());

                    if char_index == target.len() - 1 {
                        *found_count += 1;
                        return;
                    }

                    return check_neighbors(
                        &result.point,
                        path,
                        found_count,
                        target,
                        char_index + 1,
                        Some(result.direction.clone()),
                        points_map,
                        limit,
                    );
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut points_map: HashMap<String, char> = HashMap::new();
    let mut occurrences: u32 = 0;
    let limit = input.lines().count();

    for (line_index, line) in input.lines().enumerate() {
        for (char_index, c) in line.chars().enumerate() {
            points_map.insert(format!("{},{}", char_index, line_index), c);
        }
    }

    for x in 0..limit {
        for y in 0..limit {
            let point = Point { x, y };

            if !point.check_self('X', &points_map) {
                continue;
            }

            let mut path = vec![];
            let mut found_count = 0;

            path.push(point.clone());

            check_neighbors(
                &point,
                &mut path,
                &mut found_count,
                "MAS",
                0,
                None,
                &points_map,
                limit,
            );

            occurrences += found_count as u32;
        }
    }

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

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Debug)]
struct Result {
    point: Point,
    direction: Direction,
    character: char,
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

    pub fn get_neighbor(&self, direction: &Direction, limit: usize) -> Option<Point> {
        match direction {
            Direction::Up => {
                if self.y == 0 {
                    return None;
                }

                Some(Point {
                    x: self.x,
                    y: self.y - 1,
                })
            }
            Direction::Down => {
                if self.y + 1 > limit {
                    return None;
                }

                Some(Point {
                    x: self.x,
                    y: self.y + 1,
                })
            }
            Direction::Left => {
                if self.x == 0 {
                    return None;
                }

                Some(Point {
                    x: self.x - 1,
                    y: self.y,
                })
            }
            Direction::Right => {
                if self.x + 1 > limit {
                    return None;
                }

                Some(Point {
                    x: self.x + 1,
                    y: self.y,
                })
            }
            Direction::UpLeft => {
                if self.x == 0 || self.y == 0 {
                    return None;
                }

                Some(Point {
                    x: self.x - 1,
                    y: self.y - 1,
                })
            }
            Direction::UpRight => {
                if self.x + 1 > limit || self.y == 0 {
                    return None;
                }

                Some(Point {
                    x: self.x + 1,
                    y: self.y - 1,
                })
            }
            Direction::DownLeft => {
                if self.x == 0 || self.y + 1 > limit {
                    return None;
                }

                Some(Point {
                    x: self.x - 1,
                    y: self.y + 1,
                })
            }
            Direction::DownRight => {
                if self.x + 1 > limit || self.y + 1 > limit {
                    return None;
                }

                Some(Point {
                    x: self.x + 1,
                    y: self.y + 1,
                })
            }
        }
    }

    pub fn check_self(&self, target: char, point_map: &HashMap<String, char>) -> bool {
        if let Some(value) = point_map.get(&self.to_str()) {
            if target == *value {
                return true;
            }
        }

        false
    }

    pub fn check_neighbors_for(
        &self,
        target: char,
        point_map: &HashMap<String, char>,
        limit: usize,
    ) -> Vec<Result> {
        let mut results: Vec<Result> = vec![];

        for direction in Direction::iterator() {
            if let Some(neighbor) = self.get_neighbor(direction, limit) {
                if let Some(value) = point_map.get(&neighbor.to_str()) {
                    if target == *value {
                        results.push(Result {
                            point: neighbor,
                            direction: direction.clone(),
                            character: *value,
                        });
                    }
                }
            }
        }

        results
    }
}
