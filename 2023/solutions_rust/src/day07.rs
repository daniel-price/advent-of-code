use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct TypeAndBit {
    hand: String,
    hand_type: HandType,
    bid: i64,
}

fn card_char_to_int(c: char, j_is_joker: bool) -> i64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if j_is_joker {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => c.to_digit(10).unwrap() as i64,
    }
}

fn compare_same_type_hands(a: &str, b: &str, j_is_joker: bool) -> Ordering {
    let a_chars = a.chars().collect::<Vec<char>>();
    let b_chars = b.chars().collect::<Vec<char>>();

    for i in 0..a_chars.len() {
        let a_char = a_chars[i];
        let b_char = b_chars[i];
        if a_char == b_char {
            continue;
        }

        let a_char_int = card_char_to_int(a_char, j_is_joker);
        let b_char_int = card_char_to_int(b_char, j_is_joker);
        return a_char_int.cmp(&b_char_int);
    }

    Ordering::Equal
}

fn get_hand_type(hand: &str, j_is_joker: bool) -> HandType {
    if j_is_joker {
        return get_hand_type_joker(hand);
    }

    let mut card_counts: HashMap<char, i32> = HashMap::new();

    let char_vec: Vec<char> = hand.chars().collect();
    for c in char_vec {
        *card_counts.entry(c).or_insert(0) += 1;
    }

    let max = card_counts.values().max().unwrap();
    if max == &5 {
        return HandType::FiveOfAKind;
    }
    if max == &4 {
        return HandType::FourOfAKind;
    }
    if max == &3 {
        if card_counts.values().filter(|x| x == &&2).count() == 1 {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    if max == &2 {
        if card_counts.values().filter(|x| x == &&2).count() == 2 {
            return HandType::TwoPair;
        }
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn get_hand_type_joker(hand: &str) -> HandType {
    let hand_without_jokers = hand.replace('J', "");
    let number_of_jokers = hand.len() - hand_without_jokers.len();

    if number_of_jokers > 3 {
        return HandType::FiveOfAKind;
    }

    let hand_type_without_jokers = get_hand_type(&hand_without_jokers, false);

    if number_of_jokers == 0 {
        return hand_type_without_jokers;
    }

    if number_of_jokers == 1 {
        return match hand_type_without_jokers {
            HandType::FiveOfAKind => HandType::FiveOfAKind,
            HandType::FourOfAKind => HandType::FiveOfAKind,
            HandType::FullHouse => HandType::FourOfAKind,
            HandType::ThreeOfAKind => HandType::FourOfAKind,
            HandType::TwoPair => HandType::FullHouse,
            HandType::OnePair => HandType::ThreeOfAKind,
            HandType::HighCard => HandType::OnePair,
        };
    }

    if number_of_jokers == 2 {
        return match hand_type_without_jokers {
            HandType::FiveOfAKind => HandType::FiveOfAKind,
            HandType::FourOfAKind => HandType::FiveOfAKind,
            HandType::FullHouse => HandType::FiveOfAKind,
            HandType::ThreeOfAKind => HandType::FiveOfAKind,
            HandType::TwoPair => HandType::FourOfAKind,
            HandType::OnePair => HandType::FourOfAKind,
            HandType::HighCard => HandType::ThreeOfAKind,
        };
    }

    if number_of_jokers == 3 {
        return match hand_type_without_jokers {
            HandType::FiveOfAKind => HandType::FiveOfAKind,
            HandType::FourOfAKind => HandType::FiveOfAKind,
            HandType::FullHouse => HandType::FiveOfAKind,
            HandType::ThreeOfAKind => HandType::FiveOfAKind,
            HandType::TwoPair => HandType::FiveOfAKind,
            HandType::OnePair => HandType::FiveOfAKind,
            HandType::HighCard => HandType::FourOfAKind,
        };
    }

    HandType::FiveOfAKind
}

fn part1(input: String) -> i64 {
    let j_is_joker = false;
    let mut handtype_and_bids = input
        .lines()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            let hand = split[0];
            let bid = split[1].parse::<i64>().unwrap();

            let hand_type = get_hand_type(hand, j_is_joker);

            TypeAndBit {
                hand: hand.to_string(),
                hand_type,
                bid,
            }
        })
        .collect::<Vec<TypeAndBit>>();

    handtype_and_bids.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            return compare_same_type_hands(&a.hand, &b.hand, j_is_joker);
        }

        if a.hand_type > b.hand_type {
            return Ordering::Less;
        }

        Ordering::Greater
    });

    handtype_and_bids
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let rank = (i as i64) + 1;
            rank * x.bid
        })
        .sum::<i64>()
}

fn part2(input: String) -> i64 {
    let j_is_joker = true;
    let mut handtype_and_bids = input
        .lines()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            let hand = split[0];
            let bid = split[1].parse::<i64>().unwrap();

            let hand_type = get_hand_type(hand, j_is_joker);

            TypeAndBit {
                hand: hand.to_string(),
                hand_type,
                bid,
            }
        })
        .collect::<Vec<TypeAndBit>>();

    handtype_and_bids.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            return compare_same_type_hands(&a.hand, &b.hand, j_is_joker);
        }

        if a.hand_type > b.hand_type {
            return Ordering::Less;
        }

        Ordering::Greater
    });

    handtype_and_bids
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let rank = (i as i64) + 1;
            rank * x.bid
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::input;

    #[test]
    fn part1sample() {
        let answer = part1(input("example", file!()));
        assert_eq!(answer, 6440);
    }

    #[test]
    fn part1input() {
        let answer = part1(input("input", file!()));
        assert_eq!(answer, 246912307);
    }

    #[test]
    fn part2sample() {
        let answer = part2(input("example", file!()));
        assert_eq!(answer, 5905);
    }

    #[test]
    fn part2input() {
        let answer = part2(input("input", file!()));
        assert_eq!(answer, 246894760);
    }
}

