advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();
    let caps = re.captures_iter(input);
    let mut sum = 0;

    for cap in caps {
        let first = &cap["first"].parse::<u32>().unwrap();
        let second = &cap["second"].parse::<u32>().unwrap();
        sum += first * second;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)|(?<dont>don't\(\))|(?<do>do\(\))",
    )
    .unwrap();
    let caps = re.captures_iter(input);
    let mut sum = 0;

    let mut do_mul = true;

    for cap in caps {
        match cap.name("dont") {
            Some(_) => {
                do_mul = false;
                continue;
            }
            None => {}
        }

        match cap.name("do") {
            Some(_) => {
                do_mul = true;
                continue;
            }
            None => {}
        }

        let first = &cap["first"].parse::<u32>().unwrap();
        let second = &cap["second"].parse::<u32>().unwrap();

        if do_mul {
            sum += first * second;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
