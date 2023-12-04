use std::collections::HashMap;

fn part1(input: String) -> i32 {
    input
        .split('\n')
        .map(|line| {
            let numbers = line.split(": ").collect::<Vec<&str>>()[1];
            let split_numbers = numbers.split('|').collect::<Vec<&str>>();
            let winning_numbers = split_numbers[0].split(' ').collect::<Vec<&str>>();
            let card_numbers = split_numbers[1].split(' ').collect::<Vec<&str>>();

            let winning_card_numbers = card_numbers.iter().filter(|card_number| {
                !card_number.is_empty() && winning_numbers.contains(card_number)
            });

            let winning_card_numbers_count = winning_card_numbers.clone().count();

            if winning_card_numbers_count == 0 {
                0
            } else {
                let base: i32 = 2;

                base.pow((winning_card_numbers_count - 1).try_into().unwrap())
            }
        })
        .sum()
}

fn part2(input: String) -> i32 {
    let mut total_cards_by_number: HashMap<usize, i32> = HashMap::new();
    for (pos, line) in input.split('\n').enumerate() {
        let card_number = pos + 1;
        let numbers = line.split(": ").collect::<Vec<&str>>()[1];
        let split_numbers = numbers.split('|').collect::<Vec<&str>>();
        let winning_numbers = split_numbers[0].split(' ').collect::<Vec<&str>>();
        let card_numbers = split_numbers[1].split(' ').collect::<Vec<&str>>();

        let winning_card_numbers = card_numbers
            .iter()
            .filter(|card_number| !card_number.is_empty() && winning_numbers.contains(card_number));

        let winning_card_numbers_count = winning_card_numbers.clone().count();

        let card_count = total_cards_by_number.get(&card_number).unwrap_or(&0) + 1;

        total_cards_by_number.insert(card_number, card_count);

        for _ in 0..card_count {
            for i in card_number + 1..card_number + 1 + winning_card_numbers_count {
                let current_card_count = total_cards_by_number.get(&i).unwrap_or(&0) + 1;
                total_cards_by_number.insert(i, current_card_count);
            }
        }
    }
    total_cards_by_number.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::input;

    #[test]
    fn part1sample() {
        let answer = part1(input("example", file!()));
        assert_eq!(answer, 13);
    }

    #[test]
    fn part1input() {
        let answer = part1(input("input", file!()));
        assert_eq!(answer, 25004);
    }

    #[test]
    fn part2sample() {
        let answer = part2(input("example", file!()));
        assert_eq!(answer, 30);
    }

    #[test]
    fn part2input() {
        let answer = part2(input("input", file!()));
        assert_eq!(answer, 0);
    }
}
