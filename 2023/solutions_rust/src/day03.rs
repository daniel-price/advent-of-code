fn part1(input: String) -> i32 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    lines
        .iter()
        .enumerate()
        .map(|(line_index, line)| {
            let mut first_number_index_option = None;
            let line_before_option = if line_index > 0 {
                lines.get(line_index - 1)
            } else {
                None
            };
            let line_after_option = lines.get(line_index + 1);
            line.chars()
                .enumerate()
                .map(|(char_index, char)| {
                    let mut sum = 0;
                    if char.is_ascii_digit() && first_number_index_option.is_none() {
                        first_number_index_option = Some(char_index);
                    }

                    let is_last_char = char_index == line.len() - 1;
                    if !char.is_ascii_digit() || is_last_char {
                        if let Some(first_number_index) = first_number_index_option {
                            let mut is_adjacent_symbol = false;
                            if first_number_index > 0 {
                                let char_before = line.chars().nth(first_number_index - 1).unwrap();
                                if !char_before.is_ascii_digit() && char_before != '.' {
                                    is_adjacent_symbol = true;
                                }
                            }

                            if !char.is_ascii_digit() && char != '.' {
                                is_adjacent_symbol = true;
                            }

                            let first_number_above_below = if first_number_index > 0 {
                                first_number_index - 1
                            } else {
                                first_number_index
                            };
                            for i in first_number_above_below..char_index + 1 {
                                if let Some(line_before) = line_before_option {
                                    let char_line_before = line_before.chars().nth(i).unwrap();
                                    if !char_line_before.is_ascii_digit() && char_line_before != '.'
                                    {
                                        is_adjacent_symbol = true;
                                    }
                                }
                                if let Some(line_after) = line_after_option {
                                    let char_line_after = line_after.chars().nth(i).unwrap();
                                    if !char_line_after.is_ascii_digit() && char_line_after != '.' {
                                        is_adjacent_symbol = true;
                                    }
                                }
                            }

                            let last_number_index = if char.is_ascii_digit() {
                                char_index + 1
                            } else {
                                char_index
                            };

                            let number = line
                                [first_number_index_option.unwrap()..last_number_index]
                                .parse::<i32>()
                                .unwrap();

                            if is_adjacent_symbol {
                                sum += number;
                            }
                        }
                        first_number_index_option = None;
                    }
                    sum
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn part2(input: String) -> i128 {
    let lines = input.split('\n').collect::<Vec<&str>>();
    lines
        .iter()
        .enumerate()
        .map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .map(|(char_index, char)| {
                    if char == '*' {
                        let line_above_option = lines.get(line_index - 1);
                        let line_below_option = lines.get(line_index + 1);

                        let mut surrounding_numbers = Vec::new();

                        if let Some(line_above) = line_above_option {
                            let mut line_above_numbers = line_above[0..char_index]
                                .chars()
                                .rev()
                                .take_while(|c| c.is_ascii_digit())
                                .collect::<String>()
                                .chars()
                                .rev()
                                .collect::<String>();

                            let line_above_additional = line_above[char_index + 1..]
                                .chars()
                                .take_while(|c| c.is_ascii_digit())
                                .collect::<String>();

                            let char_above = line_above.chars().nth(char_index).unwrap();
                            line_above_numbers.push_str(&char_above.to_string());

                            line_above_numbers.push_str(line_above_additional.as_str());

                            let line_above_numbers_split = line_above_numbers
                                .split('.')
                                .filter(|e| !e.is_empty())
                                .collect::<Vec<&str>>();
                            if line_above_numbers_split.len() > 1 {
                                println!(
                                    "line_above_numbers_split: {:?}",
                                    line_above_numbers_split
                                );
                                return line_above_numbers_split[0].parse::<i128>().unwrap()
                                    * line_above_numbers_split[1].parse::<i128>().unwrap();
                            }

                            if line_above_numbers_split.len() == 1
                                && !line_above_numbers_split[0].is_empty()
                            {
                                let number_above =
                                    line_above_numbers_split[0].parse::<i128>().unwrap();
                                surrounding_numbers.push(number_above);
                            }
                        };

                        let numbers_before = line[0..char_index]
                            .chars()
                            .rev()
                            .take_while(|c| c.is_ascii_digit())
                            .collect::<String>()
                            .chars()
                            .rev()
                            .collect::<String>();

                        if !numbers_before.is_empty() {
                            let parsed = numbers_before.parse::<i128>().unwrap();
                            surrounding_numbers.push(parsed);
                        }

                        let numbers_after = line[char_index + 1..]
                            .chars()
                            .take_while(|c| c.is_ascii_digit())
                            .collect::<String>();

                        if !numbers_after.is_empty() {
                            let parsed = numbers_after.parse::<i128>().unwrap();
                            surrounding_numbers.push(parsed);
                        }

                        if let Some(line_below) = line_below_option {
                            let mut line_below_numbers = line_below[0..char_index]
                                .chars()
                                .rev()
                                .take_while(|c| c.is_ascii_digit())
                                .collect::<String>()
                                .chars()
                                .rev()
                                .collect::<String>();
                            let line_below_additional = line_below[char_index + 1..]
                                .chars()
                                .take_while(|c| c.is_ascii_digit())
                                .collect::<String>();

                            let char_below = line_below.chars().nth(char_index).unwrap();
                            line_below_numbers.push_str(&char_below.to_string());

                            line_below_numbers.push_str(line_below_additional.as_str());

                            let line_below_numbers_split = line_below_numbers
                                .split('.')
                                .filter(|e| !e.is_empty())
                                .collect::<Vec<&str>>();
                            if line_below_numbers_split.len() > 1 {
                                return line_below_numbers_split[0].parse::<i128>().unwrap()
                                    * line_below_numbers_split[1].parse::<i128>().unwrap();
                            }

                            if line_below_numbers_split.len() == 1
                                && !line_below_numbers_split[0].is_empty()
                            {
                                let number_below =
                                    line_below_numbers_split[0].parse::<i128>().unwrap();
                                surrounding_numbers.push(number_below);
                            }
                        };

                        if surrounding_numbers.len() > 2 {
                            panic!("surrounding_numbers.len() > 2 {:?}", surrounding_numbers);
                        }

                        if surrounding_numbers.len() < 2 {
                            return 0;
                        }

                        let product = surrounding_numbers
                            .iter()
                            .copied()
                            .reduce(|a, b| a * b)
                            .unwrap();

                        println!("surrounding_numbers: {:?} ({product})", surrounding_numbers);

                        product
                    } else {
                        0
                    }
                })
                .sum::<i128>()
        })
        .sum::<i128>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::input;

    #[test]
    fn part1sample() {
        let answer = part1(input("example", file!()));
        assert_eq!(answer, 4361);
    }

    #[test]
    fn part1input() {
        let answer = part1(input("input", file!()));
        assert_eq!(answer, 528799);
    }

    #[test]
    fn part2sample() {
        let answer = part2(input("example", file!()));
        assert_eq!(answer, 467835);
    }

    #[test]
    fn part2input() {
        let answer = part2(input("input", file!()));
        assert_eq!(answer, 0);
    }
}
