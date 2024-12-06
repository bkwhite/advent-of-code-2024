use std::collections::HashMap;
use std::slice::Iter;
use std::usize;

advent_of_code::solution!(4);

fn check_neighbors_part_1(
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
        let results = point.check_neighbors_for(target_char, points_map, limit, false);

        for result in results.iter() {
            if target_char == result.character && char_index == 0 {
                let mut new_path = path.clone();
                new_path.push(result.point.clone());

                check_neighbors_part_1(
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

                    return check_neighbors_part_1(
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

fn check_neighbors_part_2(
    point: &Point,
    path: &mut Vec<Point>,
    paths: &mut Vec<Vec<Point>>,
    found_count: &mut usize,
    target: &str,
    char_index: usize,
    last_direction: Option<Direction>,
    points_map: &HashMap<String, char>,
    limit: usize,
) -> Vec<Point> {
    if let Some(target_char) = target.chars().nth(char_index) {
        let results = point.check_neighbors_for(target_char, points_map, limit, true);

        for result in results.iter() {
            if target_char == result.character && char_index == 0 {
                let mut new_path = path.clone();
                new_path.push(result.point.clone());

                check_neighbors_part_2(
                    &result.point,
                    &mut new_path,
                    paths,
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
                        paths.push(path.clone());
                        return path.clone();
                    }

                    return check_neighbors_part_2(
                        &result.point,
                        path,
                        paths,
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

    path.clone()
}

fn build_points_map(input: &str) -> HashMap<String, char> {
    let mut points_map: HashMap<String, char> = HashMap::new();

    for (line_index, line) in input.lines().enumerate() {
        for (char_index, c) in line.chars().enumerate() {
            points_map.insert(format!("{},{}", char_index, line_index), c);
        }
    }

    points_map
}

#[allow(dead_code)]
fn print_debug_map(map: &HashMap<String, char>, limit: usize) {
    for y in 0..limit {
        for x in 0..limit {
            if let Some(value) = map.get(format!("{},{}", x, y).as_str()) {
                print!("{} ", value);
            } else {
                print!("_ ");
            }
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let points_map = build_points_map(input);
    let mut occurrences: u32 = 0;
    let limit = input.lines().count();

    for x in 0..limit {
        for y in 0..limit {
            let point = Point { x, y };

            if !point.check_self('X', &points_map) {
                continue;
            }

            let mut path = vec![];
            let mut found_count = 0;

            path.push(point.clone());

            check_neighbors_part_1(
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
    let mut debug_map: HashMap<String, char> = HashMap::new();
    let mut debug_map_2: HashMap<String, char> = HashMap::new();
    let points_map = build_points_map(input);
    let mut occurrences: u32 = 0;
    let limit = input.lines().count();
    let mut mas_paths: Vec<Vec<Point>> = vec![];
    let mut found_count = 0;

    // find MAS
    for x in 0..limit {
        for y in 0..limit {
            let point = Point { x, y };

            if !point.check_self('M', &points_map) {
                continue;
            }

            let mut path = vec![];

            path.push(point.clone());

            let result_path = check_neighbors_part_2(
                &point,
                &mut path,
                &mut mas_paths,
                &mut found_count,
                "AS",
                0,
                None,
                &points_map,
                limit,
            );
        }
    }

    let mut mas_map: HashMap<String, Vec<Vec<Point>>> = HashMap::new();

    for path in mas_paths {
        for point in path.clone() {
            debug_map.insert(point.to_str(), point.get_char_at_self(&points_map));
        }

        if let Some(second) = path.get(1) {
            if let Some(paths) = mas_map.get_mut(&second.to_str()) {
                paths.push(path.clone());
            } else {
                mas_map.insert(second.to_str(), vec![path.clone()]);
            }
        }
    }

    for (_, paths) in mas_map.clone() {
        if paths.len() == 2 {
            for path in paths {
                for point in path.clone() {
                    debug_map_2.insert(point.to_str(), point.get_char_at_self(&points_map));
                }
            }

            occurrences += 1;
        }
    }

    Some(occurrences)
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
        assert_eq!(result, Some(9));
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
    pub fn iterator(diagonal: bool) -> Iter<'static, Direction> {
        if diagonal {
            static DIRECTIONS: [Direction; 4] = [
                Direction::UpLeft,
                Direction::UpRight,
                Direction::DownLeft,
                Direction::DownRight,
            ];

            return DIRECTIONS.iter();
        }

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

    pub fn get_char_at_self(&self, point_map: &HashMap<String, char>) -> char {
        *point_map.get(&self.to_str()).unwrap()
    }

    pub fn check_neighbors_for(
        &self,
        target: char,
        point_map: &HashMap<String, char>,
        limit: usize,
        diagonal: bool,
    ) -> Vec<Result> {
        let mut results: Vec<Result> = vec![];

        for direction in Direction::iterator(diagonal) {
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
