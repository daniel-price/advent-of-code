fn part1(input: String) -> i128 {
    let lines = input.lines().collect::<Vec<&str>>();
    println!("{:?}", lines);

    let time_line = lines[0];
    let distance_line = lines[1];

    let times = time_line.split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let distances = distance_line.split(": ").collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let mut product = 1;

    for i in 0..times.len() {
        let time = times[i];
        let record_distance = distances[i];

        let mut ways_to_beat = 0;
        for j in 0..time {
            let distance = (time - j) * j;
            if distance > record_distance {
                ways_to_beat += 1;
            }
        }

        product *= ways_to_beat;
    }

    product
}

fn part2(input: String) -> i128 {
    let lines = input.lines().collect::<Vec<&str>>();
    println!("{:?}", lines);

    let time_line = lines[0];
    let distance_line = lines[1];

    let time = time_line.split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i128>()
        .unwrap();

    let record_distance = distance_line.split(": ").collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i128>()
        .unwrap();

    let mut product = 1;

    let mut ways_to_beat = 0;
    for j in 0..time {
        let distance = (time - j) * j;
        if distance > record_distance {
            ways_to_beat += 1;
        }
    }

    product *= ways_to_beat;

    product
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::input;

    #[test]
    fn part1sample() {
        let answer = part1(input("example", file!()));
        assert_eq!(answer, 288);
    }

    #[test]
    fn part1input() {
        let answer = part1(input("input", file!()));
        assert_eq!(answer, 3316275);
    }

    #[test]
    fn part2sample() {
        let answer = part2(input("example", file!()));
        assert_eq!(answer, 71503);
    }

    #[test]
    fn part2input() {
        let answer = part2(input("input", file!()));
        assert_eq!(answer, 27102791);
    }
}
