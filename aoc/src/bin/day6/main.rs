use std::collections::HashSet;

fn main() {
    let answers = include_str!("input.txt").split("\n\n").collect::<Vec<_>>();

    println!("part1: {}", part1(&answers));
    println!("part2: {}", part2(&answers));
}

fn part1(answer_strings: &Vec<&str>) -> usize {
    answer_strings
        .iter()
        .map(|s| {
            s.chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<_>>()
        })
        .fold(0, |acc, answer_set| acc + answer_set.len())
}

fn part2(answer_strings: &Vec<&str>) -> usize {
    answer_strings
        .iter()
        .map(|group_answers| {
            group_answers
                .lines()
                .map(|person_answers| person_answers.chars().collect::<HashSet<_>>())
                .fold(
                    (b'a'..=b'z').map(|c| c as char).collect::<HashSet<_>>(),
                    |acc, answer_set| acc.intersection(&answer_set).cloned().collect(),
                )
        })
        .fold(0, |acc, answer_set| acc + answer_set.len())
}
