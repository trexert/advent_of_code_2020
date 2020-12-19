use lazy_static::lazy_static;
use regex::Regex;
use std::assert;
use std::collections::HashMap;
use std::collections::HashSet;

struct Field {
    pub name: String,
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
            .any(|range| val >= range.0 && val <= range.1)
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
    println!("part2: {}", part2(&fields, &tickets, &my_ticket));
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

fn part2(fields: &Vec<Field>, tickets: &Vec<Vec<usize>>, my_ticket: &Vec<usize>) -> usize {
    let valid_tickets: Vec<_> = tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|val| fields.iter().any(|field| field.validate(*val)))
        })
        .collect();
    let mut field_counts: HashMap<_, _> = fields
        .iter()
        .map(|field| (field.name.clone(), vec![0; fields.len()]))
        .collect();
    valid_tickets.iter().for_each(|ticket| {
        ticket.iter().enumerate().for_each(|(i, val)| {
            fields.iter().for_each(|field| {
                if field.validate(*val) {
                    field_counts.get_mut(&field.name).unwrap()[i] += 1;
                }
            })
        })
    });
    let mut field_poses: Vec<_> = field_counts
        .iter()
        .map(|(name, counts)| {
            let valid_positions: HashSet<_> = counts
                .iter()
                .enumerate()
                .filter_map(|(i, count)| {
                    if *count == valid_tickets.len() {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();
            (name, valid_positions)
        })
        .collect();

    field_poses.sort_by_key(|(_name, poses)| poses.len());

    let mut final_order = vec![""; fields.len()];
    let mut used_poses: HashSet<usize> = HashSet::new();

    for (field_name, posibilities) in field_poses {
        let position = *posibilities.difference(&used_poses).next().unwrap();
        used_poses.insert(position);
        final_order[position] = field_name;
    }

    final_order
        .iter()
        .enumerate()
        .fold(1, |acc, (i, field_name)| {
            if field_name.starts_with("departure") {
                acc * my_ticket[i]
            } else {
                acc
            }
        })
}
