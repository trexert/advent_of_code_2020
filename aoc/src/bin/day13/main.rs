#![feature(iterator_fold_self)]

use modinverse::modinverse;
use std::collections::HashMap;

fn main() {
    let mut input = include_str!("input.txt").lines();
    let earliest_time = input.next().unwrap().parse().unwrap();
    let ids = input
        .next()
        .unwrap()
        .split(',')
        .map(|s| match s {
            "x" => None,
            _ => Some(s.parse().unwrap()),
        })
        .collect();

    println!("part1: {}", part1(earliest_time, &ids));
    println!("part2: {}", part2(&ids));
}

fn part1(earliest_time: i64, ids: &Vec<Option<i64>>) -> i64 {
    ids.iter()
        .filter_map(|x| x.as_ref())
        .map(|x| {
            let waiting_time = x - (earliest_time % x);
            (waiting_time, waiting_time * x)
        })
        .min()
        .unwrap()
        .1
}

fn part2(ids: &Vec<Option<i64>>) -> i64 {
    let bases: Vec<_> = ids.iter().filter_map(|x| *x).collect();
    let modulus = bases.iter().fold(1, |acc, base| acc * base);
    let first = *bases.first().unwrap();
    let mults: HashMap<i64, i64> = bases
        .iter()
        .map(|base| {
            (
                *base,
                bases.iter().fold(1, |acc, sub_base| {
                    if sub_base == base {
                        acc
                    } else {
                        (acc * sub_base % modulus) * modinverse(*sub_base, *base).unwrap() % modulus
                    }
                }),
            )
        })
        .collect();

    ids.iter()
        .enumerate()
        .filter_map(|(i, x)| match x {
            Some(val) if *val != first => Some((i as i64, *val)),
            _ => None,
        })
        .map(|(i, x)| {
            let inverse = modinverse(first, x).unwrap();
            (inverse * -i).rem_euclid(x) * mults[&x]
        })
        .fold_first(|acc, a| acc + a)
        .unwrap()
        * first
        % modulus
}
