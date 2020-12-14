use std::cmp::Ordering;

const HISTORY_COUNT: usize = 25;

fn is_valid(number_pool: &[usize], target: usize) -> bool {
    for x in number_pool {
        for y in number_pool {
            if x == y {
                continue;
            }
            if x + y == target {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let numbers: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut index = HISTORY_COUNT;
    while index < numbers.len() && is_valid(&numbers[index - HISTORY_COUNT..index], numbers[index])
    {
        index += 1
    }

    println!("part1: {}", numbers[index]);
    println!("part2: {}", part2(&numbers, numbers[index]))
}

fn part2(numbers: &[usize], target: usize) -> usize {
    let mut lowest = 0;
    let mut highest = 1;
    let mut total: usize = numbers[lowest..highest].iter().sum();
    while highest < numbers.len() {
        match total.cmp(&target) {
            Ordering::Equal => break,
            Ordering::Less => highest += 1,
            Ordering::Greater => lowest += 1,
        }
        if highest < lowest + 2 {
            highest = lowest + 2
        }
        total = numbers[lowest..highest].iter().sum();
    }
    let min = numbers[lowest..highest].iter().min().unwrap();
    let max = numbers[lowest..highest].iter().max().unwrap();
    min + max
}
