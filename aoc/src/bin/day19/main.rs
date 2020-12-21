use alphanumeric_sort;
use regex::Regex;

enum RulePart {
    BAR,
    REF(usize),
    LETTER(char),
}

impl RulePart {
    pub fn from_str(from: &str) -> RulePart {
        match (from, from.parse()) {
            (_, Ok(x)) => RulePart::REF(x),
            ("|", _) => RulePart::BAR,
            (letter, _) => RulePart::LETTER(letter.trim_matches('"').chars().next().unwrap()),
        }
    }
}

fn parse_rule(rule: &str) -> (usize, Vec<RulePart>) {
    let mut labelling = rule.split(":");
    let rule_number = labelling.next().unwrap().parse().unwrap();
    let sub_rules = labelling
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| RulePart::from_str(s))
        .collect();
    (rule_number, sub_rules)
}

fn regex_from_rules(rules: &Vec<Vec<RulePart>>, is_part2: bool) -> Regex {
    Regex::new(&format!(
        "^(?:{})$",
        regex_str_from_rule(0, rules, is_part2)
    ))
    .unwrap()
}

fn regex_str_from_rule(rule: usize, rules: &Vec<Vec<RulePart>>, is_part2: bool) -> String {
    match (is_part2, rule) {
        (true, 8) => format!("(?:{})+", regex_str_from_rule(42, rules, is_part2)),
        (true, 11) => {
            let left = regex_str_from_rule(42, rules, is_part2);
            let right = regex_str_from_rule(31, rules, is_part2);
            format!(
                "(?:{})",
                (1..=5)
                    .map(|i| format!("(?:{}){{{}}}(?:{}){{{}}}", left, i, right, i))
                    .collect::<Vec<_>>()
                    .join("|")
            )
        }
        _ => {
            let mut result = "".to_owned();
            let mut should_bracket_end = false;
            for part in &rules[rule] {
                match part {
                    RulePart::LETTER(letter) => result.push(*letter),
                    RulePart::REF(sub_rule) => {
                        result.push_str(&regex_str_from_rule(*sub_rule, rules, is_part2))
                    }
                    RulePart::BAR => {
                        result = format!("(?:{}|", result);
                        should_bracket_end = true;
                    }
                }
            }
            if should_bracket_end {
                result.push_str(")");
            }
            result
        }
    }
}

fn main() {
    let mut rule_lines = Vec::new();
    let mut input_iter = include_str!("input.txt").lines();
    while let Some(line) = input_iter.next() {
        if line == "" {
            break;
        }

        rule_lines.push(line);
    }
    alphanumeric_sort::sort_str_slice(&mut rule_lines);

    let rules: Vec<Vec<RulePart>> = rule_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let parsed_rule = parse_rule(line);
            assert_eq!(i, parsed_rule.0);
            parsed_rule.1
        })
        .collect();

    let patterns: Vec<&str> = input_iter.collect();

    println!(
        "part1: {}",
        solve(&patterns, &regex_from_rules(&rules, false))
    );
    println!(
        "part2: {}",
        solve(&patterns, &regex_from_rules(&rules, true))
    )
}

fn solve(patterns: &Vec<&str>, checker: &Regex) -> usize {
    println!("{:?}", checker);
    patterns
        .iter()
        .filter(|pattern| checker.is_match(pattern))
        .count()
}
