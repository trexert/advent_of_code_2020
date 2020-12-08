use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct MappingElt {
    pub from: HashMap<String, usize>,
    pub to: HashMap<String, usize>,
}

fn parse_line(line: &str) -> (String, HashMap<String, usize>) {
    let bag_pattern = Regex::new(r"(?P<count>\d*)? ?(?P<color>.*?) bags?\.?").unwrap();
    let from_to = line.split("contain").map(|s| s.trim()).collect::<Vec<_>>();
    let containing = from_to[0];
    let contained = from_to[1];
    let containing_color = bag_pattern.captures(containing).unwrap()["color"].to_string();
    let line_iter = contained.split(",").map(|s| s.trim());
    let mut contained_colors = HashMap::with_capacity(line_iter.size_hint().0);
    if contained != "no other bags." {
        for contained_color in line_iter {
            let caps = bag_pattern.captures(contained_color).unwrap();
            contained_colors.insert(caps["color"].to_string(), caps["count"].parse().unwrap());
        }
    }

    (containing_color, contained_colors)
}

fn main() {
    let input_iter = include_str!("input.txt").lines();
    let mut mapping: HashMap<String, MappingElt> = HashMap::with_capacity(input_iter.size_hint().0);
    for line in input_iter {
        let (containing_color, contained_colors) = parse_line(line);
        let mut new_from = HashMap::new();
        for (color, elt) in &mapping {
            if elt.to.contains_key(&containing_color) {
                new_from.insert(color.clone(), elt.to[&containing_color]);
            }
        }
        for (color, count) in &contained_colors {
            if let Some(elt_to_update) = mapping.get_mut(color) {
                elt_to_update.from.insert(containing_color.clone(), *count);
            }
        }

        mapping.insert(
            containing_color,
            MappingElt {
                from: new_from,
                to: contained_colors,
            },
        );
    }

    println!("part1: {}", part1(&mapping));
    println!("part2: {}", part2(&mapping));
}

fn part1(mapping: &HashMap<String, MappingElt>) -> usize {
    let mut queue = VecDeque::with_capacity(mapping.len());
    let mut found_colors = HashSet::with_capacity(mapping.len());
    queue.push_back("shiny gold");
    while let Some(next) = queue.pop_front() {
        for color in mapping[next].from.keys() {
            if !found_colors.contains(color) {
                found_colors.insert(color);
                queue.push_back(color);
            }
        }
    }
    found_colors.len()
}

fn part2(mapping: &HashMap<String, MappingElt>) -> usize {
    let mut queue = VecDeque::with_capacity(mapping.len());
    let mut total_count = 0;
    queue.push_back(("shiny gold", 1));
    while let Some((color, count)) = queue.pop_front() {
        total_count += count;
        if !mapping[color].to.is_empty() {
            for (color, bag_count) in &mapping[color].to {
                queue.push_back((color, bag_count * count));
            }
        }
    }
    total_count - 1
}
