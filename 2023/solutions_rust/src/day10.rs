#[derive(Debug, PartialEq, PartialOrd)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn pad_input(input: String) -> Vec<String> {
    let mut lines = input.lines().collect::<Vec<&str>>();

    //pad input with dots
    let line_length = lines[0].len();

    let dotted_line = ".".repeat(line_length);
    lines.insert(0, &dotted_line);
    lines.insert(lines.len(), &dotted_line);

    lines
        .iter()
        .map(|l| format!(".{l}."))
        .collect::<Vec<String>>()
}

fn check(
    lines: Vec<String>,
    coords_to_check: Coordinate,
    direction: &Direction,
    mut all_steps: Vec<Coordinate>,
) -> Option<Vec<Coordinate>> {
    let mut current_coords = coords_to_check;
    let mut current_direction = direction;
    loop {
        all_steps.push(current_coords);
        let current_value = lines[current_coords.y]
            .chars()
            .nth(current_coords.x)
            .expect("no current value");

        if current_value == '.' {
            return None;
        }

        if current_value == 'S' {
            return Some(all_steps);
        }

        match current_direction {
            Direction::Up => {
                if current_value == '|' {
                    current_coords = Coordinate {
                        x: current_coords.x,
                        y: current_coords.y - 1,
                    };
                    current_direction = &Direction::Up;
                } else if current_value == '7' {
                    current_coords = Coordinate {
                        x: current_coords.x - 1,
                        y: current_coords.y,
                    };
                    current_direction = &Direction::Left;
                } else if current_value == 'F' {
                    current_coords = Coordinate {
                        x: current_coords.x + 1,
                        y: current_coords.y,
                    };
                    current_direction = &Direction::Right;
                } else {
                    return None;
                }
            }
            Direction::Left => {
                if current_value == '-' {
                    current_coords = Coordinate {
                        x: current_coords.x - 1,
                        y: current_coords.y,
                    };
                    current_direction = &Direction::Left;
                } else if current_value == 'L' {
                    current_coords = Coordinate {
                        x: current_coords.x,
                        y: current_coords.y - 1,
                    };
                    current_direction = &Direction::Up;
                } else if current_value == 'F' {
                    current_coords = Coordinate {
                        x: current_coords.x,
                        y: current_coords.y + 1,
                    };
                    current_direction = &Direction::Down;
                } else {
                    return None;
                }
            }
            Direction::Right => {
                if current_value == '-' {
                    current_coords = Coordinate {
                        x: current_coords.x + 1,
                        y: current_coords.y,
                    };
                    current_direction = &Direction::Right;
                } else if current_value == 'J' {
                    current_coords = Coordinate {
                        x: current_coords.x,
                        y: current_coords.y - 1,
                    };
                    current_direction = &Direction::Up;
                } else if current_value == '7' {
                    current_coords = Coordinate {
                        x: current_coords.x,
                        y: current_coords.y + 1,
                    };
                    current_direction = &Direction::Down;
                } else {
                    return None;
                }
            }
            Direction::Down => {
                if current_value == '|' {
                    current_coords = Coordinate {
                        x: current_coords.x,
                        y: current_coords.y + 1,
                    };
                    current_direction = &Direction::Down;
                } else if current_value == 'J' {
                    current_coords = Coordinate {
                        x: current_coords.x - 1,
                        y: current_coords.y,
                    };
                    current_direction = &Direction::Left;
                } else if current_value == 'L' {
                    current_coords = Coordinate {
                        x: current_coords.x + 1,
                        y: current_coords.y,
                    };
                    current_direction = &Direction::Right;
                } else {
                    return None;
                }
            }
        }
    }
}

fn get_total_steps(padded_lines: &Vec<String>, start_coordinate: Coordinate) -> Vec<Coordinate> {
    let up = check(
        padded_lines.clone(),
        Coordinate {
            x: start_coordinate.x,
            y: start_coordinate.y - 1,
        },
        &Direction::Up,
        vec![start_coordinate],
    );

    match up {
        Some(steps) => return steps,
        None => {}
    }

    let left = check(
        padded_lines.clone(),
        Coordinate {
            x: start_coordinate.x - 1,
            y: start_coordinate.y,
        },
        &Direction::Left,
        vec![start_coordinate],
    );

    match left {
        Some(steps) => return steps,
        None => {}
    }

    let right = check(
        padded_lines.clone(),
        Coordinate {
            x: start_coordinate.x + 1,
            y: start_coordinate.y,
        },
        &Direction::Right,
        vec![start_coordinate],
    );

    match right {
        Some(steps) => return steps,
        None => {}
    }

    let down = check(
        padded_lines.clone(),
        Coordinate {
            x: start_coordinate.x,
            y: start_coordinate.y + 1,
        },
        &Direction::Down,
        vec![start_coordinate],
    );

    match down {
        Some(steps) => return steps,
        None => {}
    }

    panic!("no exit found");
}

fn get_starting_coordinate(padded_lines: &[String]) -> Coordinate {
    for (y, line) in padded_lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                return Coordinate { x, y };
            }
        }
    }

    panic!("no start found");
}

fn part1(input: String) -> i128 {
    let padded_lines = pad_input(input);
    let start_coordinate = get_starting_coordinate(&padded_lines);
    let total_steps = get_total_steps(&padded_lines, start_coordinate);

    return ((total_steps.len()) / 2).try_into().unwrap();
}

fn part2(input: String) -> i128 {
    let padded_lines = pad_input(input);
    let start_coordinate = get_starting_coordinate(&padded_lines);
    let total_steps = get_total_steps(&padded_lines, start_coordinate);

    let all_coords = total_steps;

    println!("all coords: {:?}", all_coords);

    let assigned: Vec<String> = padded_lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            let mut in_loop = all_coords.contains(&Coordinate { x: 0, y });
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if all_coords.contains(&Coordinate { x, y }) {
                        if c == '|' {
                            in_loop = !in_loop;
                        }
                        if c == '7' || c == 'J' {
                            in_loop = false;
                        }
                        if c == 'L' || c == 'F' {
                            in_loop = true;
                        }
                        return c;
                    }
                    if in_loop {
                        return '.';
                    }
                    return '.';
                })
                .collect()
        })
        .collect();

    assigned.iter().for_each(|l| println!("{:?}", l));

    println!(
        "


    "
    );

    let assigned: Vec<String> = padded_lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            let mut in_loop = all_coords.contains(&Coordinate { x: 0, y });
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if all_coords.contains(&Coordinate { x, y }) {
                        if c == '|' {
                            in_loop = !in_loop;
                        }
                        if c == '7' || c == 'J' {
                            in_loop = false;
                        }
                        if c == 'L' || c == 'F' {
                            in_loop = true;
                        }
                        return 'N';
                    }
                    if in_loop {
                        return 'Y';
                    }
                    return 'N';
                })
                .collect()
        })
        .collect();

    assigned.iter().for_each(|l| println!("{:?}", l));

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::input;

    #[test]
    fn part1sample() {
        let answer = part1(input("example", file!()));
        assert_eq!(answer, 4);
    }

    #[test]
    fn part1sample2() {
        let answer = part1(input("example-2", file!()));
        assert_eq!(answer, 8);
    }

    #[test]
    fn part1input() {
        let answer = part1(input("input", file!()));
        assert_eq!(answer, 7012);
    }

    #[test]
    fn part2sample() {
        let answer = part2(input("example-3", file!()));
        assert_eq!(answer, 0);
    }

    // #[test]
    // fn part2input() {
    //     let answer = part2(input("input", file!()));
    //     assert_eq!(answer, 0);
    // }
}
