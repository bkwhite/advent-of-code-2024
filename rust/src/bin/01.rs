advent_of_code::solution!(1);

use std::collections::HashMap;

fn get_cols(input: &str) -> [Vec<u32>; 2] {
    let lines = input.lines();
    let mut left_col: Vec<u32> = vec![];
    let mut right_col: Vec<u32> = vec![];

    lines.for_each(|line| {
        for (index, n) in line.split_whitespace().enumerate() {
            if index == 0 {
                left_col.push(n.parse().unwrap());
            }

            if index == 1 {
                right_col.push(n.parse().unwrap());
            }
        }
    });

    left_col.sort();
    right_col.sort();

    return [left_col, right_col];
}

pub fn part_one(input: &str) -> Option<u32> {
    let [left_col, right_col] = get_cols(input);
    let sum: u32 = left_col
        .iter()
        .zip(&right_col)
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut occurrences: HashMap<u32, u32> = HashMap::new();
    let [left_col, right_col] = get_cols(input);

    right_col.iter().for_each(|n| {
        if occurrences.contains_key(n) {
            let count = occurrences.get(n).unwrap();
            occurrences.insert(*n, count + 1);
        } else {
            occurrences.insert(*n, 1);
        }
    });

    let mut sum = 0;

    left_col.iter().for_each(|n| {
        if let Some(occ) = occurrences.get(n) {
            sum += n * occ;
        }
    });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
