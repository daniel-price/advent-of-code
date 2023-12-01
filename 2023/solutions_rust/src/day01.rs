use std::collections::HashMap;
use std::fs::{self};

fn day() -> String {
    let file_name = file!();
    file_name.replace("src/", "").replace(".rs", "")
}

fn input(suffix: &str) -> String {
    let file_path = "../inputs/".to_owned() + &day() + "-" + suffix + ".txt";

    fs::read_to_string(file_path.clone())
        .unwrap_or_else(|_| panic!("Could not read file {}", file_path))
        .trim()
        .to_string()
}

fn hashmap() -> HashMap<&'static str, char> {
    HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ])
}

fn get_first_number(line: &str) -> char {
    for (i, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            return c;
        }

        let mut check_string = "".to_owned();
        for char in line.chars().skip(i) {
            if char.is_alphabetic() {
                check_string = check_string.to_owned() + &char.to_string();
                if hashmap().contains_key(&*check_string) {
                    if let Some(number) = hashmap().get(&*check_string) {
                        return *number;
                    }
                }
            }
        }
    }

    panic!("No first number found");
}

fn get_last_number(line: &str) -> char {
    for (i, c) in line.chars().rev().enumerate() {
        if c.is_ascii_digit() {
            return c;
        }

        let mut check_string = "".to_owned();
        for char in line.chars().rev().skip(i) {
            if char.is_alphabetic() {
                check_string = char.to_string() + &check_string;
                if hashmap().contains_key(&*check_string) {
                    return *hashmap().get(&*check_string).unwrap();
                }
            }
        }
    }
    panic!("No last number found");
}

fn part1(input: String) -> i32 {
    let lines: Vec<&str> = input.split('\n').collect();
    let numbers = lines.iter().map(|line| {
        let number_chars = line.chars().filter(|c| c.is_ascii_digit());

        let first_number = number_chars
            .clone()
            .collect::<Vec<_>>()
            .first()
            .expect("No first number")
            .to_string();

        let last_number = number_chars
            .rev()
            .collect::<Vec<_>>()
            .first()
            .expect("No first number")
            .to_string();

        (first_number + &last_number)
            .parse::<i32>()
            .expect("Could not parse number")
    });

    numbers.sum()
}

fn part2(input: String) -> i32 {
    let lines: Vec<&str> = input.split('\n').collect();
    let numbers = lines.iter().map(|line| {
        let first_number = get_first_number(line);
        let last_number = get_last_number(line);

        let number = format!("{}{}", first_number, last_number);
        number.parse::<i32>().expect("Could not parse number")
    });

    numbers.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1sample() {
        let answer: i32 = part1(input("example"));
        assert_eq!(answer, 142);
    }

    #[test]
    fn part1input() {
        let answer: i32 = part1(input("input"));
        assert_eq!(answer, 55090);
    }

    #[test]
    fn part2sample() {
        let answer = part2(input("example-2"));
        assert_eq!(answer, 281);
    }

    #[test]
    fn part2input() {
        let answer = part2(input("input"));
        assert_eq!(answer, 54845);
    }
}
