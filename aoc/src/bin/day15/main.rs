use std::collections::HashMap;

enum Part {
    One,
    Two,
}

fn main() {
    let starting_numbers = include_str!("input.txt")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    println!("part1: {}", solve(&starting_numbers, Part::One));
    println!("part2: {}", solve(&starting_numbers, Part::Two));
}

fn solve(starting_numbers: &Vec<usize>, part: Part) -> usize {
    let mut history: HashMap<_, _> = starting_numbers[..starting_numbers.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, num)| (*num, i))
        .collect();

    let mut last_said = starting_numbers[starting_numbers.len() - 1];

    let stopping_point = match part {
        Part::One => 2020,
        Part::Two => 30000000,
    };

    for i in starting_numbers.len() - 1..stopping_point - 1 {
        let next = if let Some(val) = history.get(&last_said) {
            i - *val
        } else {
            0
        };
        history.insert(last_said, i);
        last_said = next;
    }
    last_said
}
