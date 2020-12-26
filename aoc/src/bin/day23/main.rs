#![feature(linked_list_cursors)]

use std::collections::HashMap;

const CUPS_TO_MOVE: usize = 3;

fn main() {
    let input_string = include_str!("input.txt").trim();
    println!(
        "part1: {}",
        solve(
            &mut Circle::from_str(input_string),
            100,
            input_string.len() - 1
        )
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(""),
    );
    println!(
        "part2: {}",
        solve(
            &mut Circle::from_str_extended(input_string, 1000_000),
            10_000_000,
            2,
        )
        .iter()
        .fold(1, |acc, i| acc * i),
    )
}

fn solve(circle: &mut Circle, move_count: usize, result_count: usize) -> Vec<usize> {
    let mut current_cup = circle.first;
    for i in 0..move_count {
        circle.move_cups(current_cup);
        current_cup = circle.following(current_cup);
        if i % 100_000 == 0 {
            println!("{}", i);
        }
    }

    circle.order_from(1, result_count)
}

#[derive(Clone)]
struct Circle {
    pub first: usize,
    pub size: usize,
    pub cups: HashMap<usize, Option<usize>>,
}

impl Circle {
    pub fn from_str(starting_position: &str) -> Circle {
        let mut cups: HashMap<usize, Option<usize>> = starting_position
            .chars()
            .zip(starting_position.chars().skip(1))
            .map(|(from, to)| {
                (
                    from.to_string().parse().unwrap(),
                    Some(to.to_string().parse().unwrap()),
                )
            })
            .collect();
        let first = starting_position
            .chars()
            .next()
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        let last = starting_position
            .chars()
            .rev()
            .next()
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        cups.insert(last, Some(first));
        assert!(cups.values().all(|cup| cup.is_some()));
        Circle {
            first,
            size: cups.len(),
            cups,
        }
    }

    pub fn from_str_extended(starting_position: &str, total_length: usize) -> Circle {
        let mut circle = Circle::from_str(starting_position);
        let first = starting_position
            .chars()
            .next()
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        let last = starting_position
            .chars()
            .rev()
            .next()
            .unwrap()
            .to_string()
            .parse()
            .unwrap();

        let extension: HashMap<usize, Option<usize>> = (circle.size + 1..=total_length - 1)
            .zip(circle.size + 2..=total_length)
            .map(|(from, to)| (from, Some(to)))
            .collect();
        circle.cups.extend(extension);
        circle.cups.insert(last, Some(circle.size + 1));
        circle.cups.insert(total_length, Some(first));
        assert_eq!(circle.cups.len(), total_length);
        assert!(circle.cups.values().all(|cup| cup.is_some()));
        assert!((1..=total_length).all(|i| circle.cups.contains_key(&i)));
        circle.size = total_length;
        circle
    }

    pub fn move_cups(&mut self, from: usize) {
        let split_section = self.split_out(from, CUPS_TO_MOVE);
        let mut to = self.wrap_sum(from, -1);
        while split_section.contains(&to) {
            to = self.wrap_sum(to, -1);
        }
        self.splice_in(to, split_section);
    }

    pub fn order_from(&self, value: usize, count: usize) -> Vec<usize> {
        let mut next = self.cups[&value].unwrap();
        let mut result = vec![next];
        for _ in 1..count {
            next = self.cups[&next].unwrap();
            result.push(next);
        }
        result
    }

    pub fn following(&self, value: usize) -> usize {
        self.cups[&value].unwrap()
    }

    pub fn wrap_sum(&self, value: usize, other: isize) -> usize {
        match value as isize + other {
            x if x <= 0 => (x + self.size as isize) as usize,
            x if x > self.size as isize => (x - self.size as isize) as usize,
            x => x as usize,
        }
    }

    fn split_out(&mut self, after: usize, count: usize) -> Vec<usize> {
        let mut split = vec![self.following(after)];
        let mut next = split[0];
        for _ in 1..count {
            next = self.cups[&next].unwrap();
            split.push(next);
        }
        self.cups.insert(after, Some(self.following(next)));
        self.cups.insert(next, None);
        split
    }

    fn splice_in(&mut self, after: usize, to_splice: Vec<usize>) {
        let following = self.following(after);
        self.cups.insert(after, Some(to_splice[0]));
        self.cups
            .insert(*to_splice.last().unwrap(), Some(following));
    }
}
