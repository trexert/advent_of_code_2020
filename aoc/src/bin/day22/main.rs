#![feature(deque_range)]
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

fn main() {
    let decks: Vec<VecDeque<usize>> = include_str!("input.txt")
        .trim()
        .split("\n\n")
        .map(|deck_str| {
            deck_str
                .lines()
                .filter_map(|line| match line.parse() {
                    Ok(num) => Some(num),
                    Err(_) => None,
                })
                .collect()
        })
        .collect();

    println!("part1: {}", part1(&mut decks.clone()));
    println!("part2: {}", part2(&mut decks.clone()));
}

fn part1(mut decks: &mut Vec<VecDeque<usize>>) -> usize {
    while !decks[0].is_empty() && !decks[1].is_empty() {
        perform_turn(&mut decks);
    }
    calculate_score(&decks)
}

fn part2(mut decks: &mut Vec<VecDeque<usize>>) -> usize {
    play_recursive_game(&mut decks);
    calculate_score(&decks)
}

fn play_recursive_game(mut decks: &mut Vec<VecDeque<usize>>) -> usize {
    let mut visited_states: HashSet<Vec<VecDeque<usize>>> = HashSet::new();
    while !decks[0].is_empty() && !decks[1].is_empty() {
        if visited_states.contains(decks) {
            return 0;
        }
        visited_states.insert(decks.clone());
        perform_recursive_turn(&mut decks);
    }
    match (decks[0].is_empty(), decks[1].is_empty()) {
        (true, false) => 1,
        (false, true) => 0,
        _ => panic!("Unexpected game end: {:#?}", decks),
    }
}

fn perform_recursive_turn(decks: &mut Vec<VecDeque<usize>>) {
    let cards: Vec<usize> = decks
        .iter_mut()
        .map(|deck| deck.pop_front().unwrap())
        .collect();
    if decks[0].len() >= cards[0] && decks[1].len() >= cards[1] {
        let mut mini_decks: Vec<VecDeque<usize>> = decks
            .iter()
            .zip(cards.iter())
            .map(|(deck, card)| deck.range(..card).copied().collect())
            .collect();
        match play_recursive_game(&mut mini_decks) {
            0 => decks[0].extend(cards.iter()),
            1 => decks[1].extend(cards.iter().rev()),
            x => panic!("Unexpected winner: {}", x),
        }
    } else {
        decks
            .iter_mut()
            .zip(cards.into_iter())
            .for_each(|(deck, card)| deck.push_front(card));
        perform_turn(decks)
    }
}

fn perform_turn(decks: &mut Vec<VecDeque<usize>>) {
    let card0 = decks[0].pop_front().unwrap();
    let card1 = decks[1].pop_front().unwrap();
    match card0.cmp(&card1) {
        Ordering::Greater => decks[0].extend(vec![card0, card1].iter()),
        Ordering::Less => decks[1].extend(vec![card1, card0].iter()),
        Ordering::Equal => panic!("Unexpected tie: {}, {}\n{:?}", card0, card1, decks),
    }
}

fn calculate_score(decks: &Vec<VecDeque<usize>>) -> usize {
    let deck = match (decks[0].is_empty(), decks[1].is_empty()) {
        (true, false) => &decks[1],
        (false, true) => &decks[0],
        _ => panic!("Unexpected game end: {:#?}", decks),
    };
    let number_of_cards = deck.len();
    deck.iter()
        .enumerate()
        .map(|(i, card_value)| card_value * (number_of_cards - i))
        .sum()
}
