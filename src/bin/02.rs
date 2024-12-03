advent_of_code::solution!(2);

fn check_numbers(numbers: &Vec<i32>) -> bool {
    let mut last: i32 = 0;
    let mut last_diff: i32 = 0;
    let mut safe = true;

    for (index, current) in numbers.into_iter().enumerate() {
        if index == 0 {
            last = *current;
            continue;
        }

        let diff = last - current;

        if (diff.abs() < 1 || diff.abs() > 3)
            || (diff > 0 && last_diff < 0 || diff < 0 && last_diff > 0)
        {
            safe = false;
        }

        last = *current;
        last_diff = diff;
    }

    safe
}

fn is_line_safe(line: &str, tolerate: bool) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut is_safe = check_numbers(&numbers);

    if !tolerate && !is_safe {
        return false;
    }

    if tolerate && !is_safe {
        for index in 0..numbers.len() {
            let mut without = numbers.clone();
            without.remove(index);

            if check_numbers(&without) {
                is_safe = true;
                break;
            }
        }
    }

    if !is_safe {
        return false;
    }

    true
}

fn count_safe_reports(input: &str, tolerate: bool) -> u32 {
    let mut safe_count: u32 = 0;
    let lines = input.lines();

    for line in lines.into_iter() {
        let safe = is_line_safe(line, tolerate);

        if safe {
            safe_count += 1;
        }
    }

    safe_count
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(count_safe_reports(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(count_safe_reports(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
