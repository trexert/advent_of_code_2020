fn main() {
    let passwords: Vec<(usize, usize, char, &str)> = include_str!("input.txt")
        .lines()
        .map(|s| parse_input_line(s))
        .collect();

    let mut result = part1(&passwords);
    println!("part1 result: {}", result);
    result = part2(&passwords);
    println!("part2 result: {}", result);
}

fn parse_input_line(line: &str) -> (usize, usize, char, &str) {
    let split_line: Vec<&str> = line
        .split(" ")
        .map(|s| s.trim().trim_end_matches(':'))
        .collect();
    let counts: Vec<usize> = split_line[0]
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let letter = split_line[1].chars().collect::<Vec<char>>()[0];
    (counts[0], counts[1], letter, split_line[2])
}

fn part1(passwords: &Vec<(usize, usize, char, &str)>) -> i32 {
    let mut valid_count = 0;
    for password in passwords {
        let (min, max, letter, word) = password.clone();
        let letter_count = word.matches(letter).count();
        if letter_count >= min && letter_count <= max {
            valid_count += 1;
        }
    }
    valid_count
}

fn part2(passwords: &Vec<(usize, usize, char, &str)>) -> i32 {
    let mut valid_count = 0;
    for password in passwords {
        let (min, max, letter, word) = password.clone();
        let mut chars = word.chars();
        let first = chars.nth(min - 1).unwrap();
        let second = chars.nth(max - min - 1).unwrap();
        if (first == letter) ^ (second == letter) {
            valid_count += 1;
        }
    }
    valid_count
}
