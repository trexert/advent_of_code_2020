use lazy_static::lazy_static;
use regex::Regex;
use std::assert;

struct Field {
    name: String,
    ranges: Vec<(usize, usize)>,
}

impl Field {
    pub fn from_str(field_str: &str) -> Field {
        lazy_static! {
            static ref FIELD_PARSER: Regex = Regex::new(r"(.*?): (\d*-\d*) or (\d*-\d*)").unwrap();
        }
        let caps = FIELD_PARSER.captures(field_str).unwrap();

        let name = caps[1].to_string();
        let ranges = (2..=3)
            .map(|i| {
                let min_max: Vec<_> = caps[i]
                    .split('-')
                    .map(|boundary| boundary.parse().unwrap())
                    .collect();
                (min_max[0], min_max[1])
            })
            .collect();
        Field {
            name: name,
            ranges: ranges,
        }
    }

    pub fn validate(&self, val: usize) -> bool {
        self.ranges
            .iter()
            .any(|range| val > range.0 && val < range.1)
    }
}

fn main() {
    let mut input_iter = include_str!("input.txt").lines();

    let mut fields: Vec<Field> = Vec::new();
    while let Some(line) = input_iter.next() {
        if line == "" {
            break;
        }

        fields.push(Field::from_str(line));
    }

    assert!(input_iter.next().unwrap() == "your ticket:");

    let my_ticket: Vec<usize> = input_iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    assert!(input_iter.next().unwrap() == "");
    assert!(input_iter.next().unwrap() == "nearby tickets:");

    let mut tickets: Vec<Vec<usize>> = Vec::new();
    while let Some(line) = input_iter.next() {
        if line == "" {
            break;
        }

        tickets.push(line.split(',').map(|s| s.parse().unwrap()).collect());
    }

    println!("part1: {}", part1(&fields, &tickets));
}

fn part1(fields: &Vec<Field>, tickets: &Vec<Vec<usize>>) -> usize {
    tickets
        .iter()
        .map(|ticket| {
            ticket.iter().fold(0, |acc, val| {
                acc + if !fields.iter().any(|field| field.validate(*val)) {
                    *val
                } else {
                    0
                }
            })
        })
        .sum()
}
