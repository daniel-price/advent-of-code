use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn part1(input: String) -> i128 {
    let mut nodes_by_name = HashMap::new();
    let lines = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let directions = lines[0];
    lines[1..].iter().for_each(|line| {
        let split = line.split(" = ").collect::<Vec<&str>>();
        let name = split[0];
        let nodes = split[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .collect::<Vec<&str>>();

        let left = nodes[0].to_string();
        let right = nodes[1].to_string();

        let node = Node { left, right };

        nodes_by_name.insert(name, node);
    });

    let mut current_node = "AAA";
    let mut number_of_steps = 0;
    loop {
        for c in directions.chars() {
            number_of_steps += 1;
            let node = nodes_by_name.get(current_node).unwrap();
            current_node = match c {
                'L' => node.left.as_str(),
                'R' => node.right.as_str(),
                _ => panic!("Unknown direction"),
            };
        }
        if current_node == "ZZZ" {
            break;
        }
    }

    number_of_steps
}

fn part2(input: String) -> i128 {
    let mut nodes_by_name = HashMap::new();
    let lines = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let directions = lines[0];
    lines[1..].iter().for_each(|line| {
        let split = line.split(" = ").collect::<Vec<&str>>();
        let name = split[0];
        let nodes = split[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .collect::<Vec<&str>>();

        let left = nodes[0].to_string();
        let right = nodes[1].to_string();

        let node = Node { left, right };

        nodes_by_name.insert(name, node);
    });

    let start_nodes = nodes_by_name
        .iter()
        .filter(|(name, _)| name.chars().collect::<Vec<char>>()[2] == 'A')
        .map(|(name, _)| *name)
        .collect::<Vec<&str>>();

    let first_z_steps = start_nodes
        .iter()
        .map(|node| {
            let mut current_node = *node;
            let mut number_of_steps = 0;
            loop {
                for c in directions.chars() {
                    number_of_steps += 1;
                    let node = nodes_by_name.get(current_node).unwrap();
                    current_node = match c {
                        'L' => &node.left.as_str(),
                        'R' => &node.right.as_str(),
                        _ => panic!("Unknown direction"),
                    };
                }
                if current_node.ends_with('Z') {
                    break;
                }
            }
            number_of_steps
        })
        .collect::<Vec<i128>>();

    let mut product: i128 = 1;
    first_z_steps
        .iter()
        .for_each(|steps| product = num_integer::lcm(product, *steps));

    product
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::input;

    #[test]
    fn part1sample() {
        let answer = part1(input("example", file!()));
        assert_eq!(answer, 2);
    }

    #[test]
    fn part1sample2() {
        let answer = part1(input("example-2", file!()));
        assert_eq!(answer, 6);
    }

    #[test]
    fn part1input() {
        let answer = part1(input("input", file!()));
        assert_eq!(answer, 17287);
    }

    #[test]
    fn part2sample() {
        let answer = part2(input("example-3", file!()));
        assert_eq!(answer, 6);
    }

    #[test]
    fn part2input() {
        let answer = part2(input("input", file!()));
        assert_eq!(answer, 18625484023687);
    }
}
