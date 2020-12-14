use std::cmp::max;

fn main() {
    let mut adapters: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    adapters.sort_unstable();

    println!("part1: {}", part1(&adapters));
    println!("part2: {}", part2(&adapters));
}

fn part1(sorted_adapters: &Vec<usize>) -> usize {
    let mut gap_counts = [0; 4];
    let mut current_joltage = 0;
    for adapter in sorted_adapters {
        gap_counts[adapter - current_joltage] += 1;
        current_joltage = *adapter;
    }
    gap_counts[1] * (gap_counts[3] + 1)
}

fn part2(sorted_adapters: &Vec<usize>) -> usize {
    let mut counts = Vec::with_capacity(sorted_adapters.len());
    let mut index = 0;
    while sorted_adapters[index] <= 3 {
        counts.push(1 << index);
        index += 1;
    }
    while index < sorted_adapters.len() {
        counts.push(check_possibilities(
            sorted_adapters[index],
            &sorted_adapters[max(0, index - 3)..index],
            &counts[max(0, index - 3)..index],
        ));
        index += 1;
    }
    counts[counts.len() - 1]
}

fn check_possibilities(
    current_adapter: usize,
    possible_adapters: &[usize],
    counts: &[usize],
) -> usize {
    possible_adapters
        .iter()
        .zip(counts.iter())
        .fold(0, |acc, (adapter, count)| {
            if current_adapter - adapter <= 3 {
                acc + count
            } else {
                acc
            }
        })
}
