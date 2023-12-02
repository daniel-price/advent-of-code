fn part1(input: String) -> i32 {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let lines: Vec<&str> = input.split('\n').collect();
    let ids = lines.iter().map(|line| {
        let split_line = line.split(": ").collect::<Vec<&str>>();
        let game_number = split_line[0].split(' ').collect::<Vec<&str>>()[1];
        let sets = split_line[1].split("; ").collect::<Vec<&str>>();

        for set in sets {
            let number_and_colours = set.split(", ").collect::<Vec<&str>>();
            for number_and_colour in number_and_colours {
                let split_number_and_colour = number_and_colour.split(' ').collect::<Vec<&str>>();
                let number = split_number_and_colour[0].parse::<i32>().unwrap();
                let colour = split_number_and_colour[1];
                if (colour == "red" && number > red_limit)
                    || (colour == "green" && number > green_limit)
                    || (colour == "blue" && number > blue_limit)
                {
                    return 0;
                }
            }
        }

        game_number.parse::<i32>().unwrap()
    });

    ids.sum()
}

fn part2(input: String) -> i32 {
    let lines: Vec<&str> = input.split('\n').collect();
    let powers = lines.iter().map(|line| {
        let split_line = line.split(": ").collect::<Vec<&str>>();
        let sets = split_line[1].split("; ").collect::<Vec<&str>>();

        let mut red_limit = 0;
        let mut green_limit = 0;
        let mut blue_limit = 0;
        sets.iter().for_each(|set| {
            let number_and_colours = set.split(", ").collect::<Vec<&str>>();
            for number_and_colour in number_and_colours {
                let split_number_and_colour = number_and_colour.split(' ').collect::<Vec<&str>>();
                let number = split_number_and_colour[0].parse::<i32>().unwrap();
                let colour = split_number_and_colour[1];

                if colour == "red" {
                    red_limit = red_limit.max(number);
                } else if colour == "green" {
                    green_limit = green_limit.max(number);
                } else if colour == "blue" {
                    blue_limit = blue_limit.max(number);
                }
            }
        });

        red_limit * green_limit * blue_limit
    });

    powers.sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::input;

    #[test]
    fn part1sample() {
        let answer: i32 = part1(input("example", file!()));
        assert_eq!(answer, 8);
    }

    #[test]
    fn part1input() {
        let answer: i32 = part1(input("input", file!()));
        assert_eq!(answer, 3059);
    }

    #[test]
    fn part2sample() {
        let answer = part2(input("example", file!()));
        assert_eq!(answer, 2286);
    }

    #[test]
    fn part2input() {
        let answer = part2(input("input", file!()));
        assert_eq!(answer, 65371);
    }
}
