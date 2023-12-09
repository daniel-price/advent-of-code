fn get_next_value(values: Vec<i128>) -> i128 {
    let differences = get_differences(&values);
    let last_value = values[values.len() - 1];

    let all_zeroes = differences
        .iter()
        .filter(|&difference| difference != &0)
        .collect::<Vec<&i128>>()
        .len()
        == 0;
    if all_zeroes {
        let last_difference = differences[differences.len() - 1];
        return last_value + last_difference;
    }
    last_value + get_next_value(differences)
}

fn get_previous_value(values: Vec<i128>) -> i128 {
    let differences = get_differences(&values);
    let first_value = values[0];

    let all_zeroes = differences
        .iter()
        .filter(|&difference| difference != &0)
        .collect::<Vec<&i128>>()
        .len()
        == 0;
    if all_zeroes {
        let first_difference = differences[0];
        return first_value - first_difference;
    }
    first_value - get_previous_value(differences)
}

fn get_differences(values: &Vec<i128>) -> Vec<i128> {
    let mut differences = Vec::new();
    for i in 0..values.len() - 1 {
        let difference = values[i + 1] - values[i];
        differences.push(difference);
    }
    differences
}

fn part1(input: String) -> i128 {
    let histories = input.lines().map(|line| {
        line.split(" ")
            .map(|s| s.parse::<i128>().unwrap())
            .collect::<Vec<i128>>()
    });

    histories
        .map(|history| {
            let next_value = get_next_value(history);
            next_value
        })
        .sum()
}

fn part2(input: String) -> i128 {
    let histories = input.lines().map(|line| {
        line.split(" ")
            .map(|s| s.parse::<i128>().unwrap())
            .collect::<Vec<i128>>()
    });

    histories
        .map(|history| {
            let next_value = get_previous_value(history);
            next_value
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::input;

    #[test]
    fn part1sample() {
        let answer = part1(input("example", file!()));
        assert_eq!(answer, 114);
    }

    #[test]
    fn part1input() {
        let answer = part1(input("input", file!()));
        assert_eq!(answer, 1901217887);
    }

    #[test]
    fn part2sample() {
        let answer = part2(input("example", file!()));
        assert_eq!(answer, 2);
    }

    #[test]
    fn part2input() {
        let answer = part2(input("input", file!()));
        assert_eq!(answer, 905);
    }
}

