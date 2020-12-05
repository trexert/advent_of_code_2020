fn main() {
    let mut seats: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|s| {
            let mut seat_id = 0;
            for (i, character) in s.chars().enumerate() {
                match character {
                    'F' => (),
                    'B' => seat_id |= 1 << (9 - i),
                    'L' => (),
                    'R' => seat_id |= 1 << (9 - i),
                    _ => panic!("Unexpected code character"),
                }
            }
            seat_id
        })
        .collect();

    println!("part1: {}", seats.iter().max().unwrap());
    println!("part2: {}", part2(&mut seats));
}

fn part2(seats: &mut Vec<i32>) -> i32 {
    seats.sort_unstable();
    let mut iter = seats.iter();
    let mut prev = iter.next().unwrap();
    return loop {
        let seat_id = iter
            .next()
            .expect("Reached end of list before finding gap!");
        if *seat_id != prev + 1 {
            break seat_id - 1;
        }
        prev = seat_id
    };
}
