fn main() {
    let mut expense_report: Vec<i32> =
        include_str!("input.txt")
            .lines()
            .map(|s| s.parse().unwrap())
            .collect();
    
    expense_report.sort();

    let part1_result = part1(&expense_report);
    println!("part1 result = {}", part1_result.expect("Failed to find result!"));

    let part2_result = part2(&expense_report);
    println!("part2 result = {}", part2_result.expect("Failed to find result!"))
}

fn part1(report: &Vec<i32>) -> Option<i32> {
    let mut i = 0;
    let mut j = report.len() - 1;
    loop {
        if i >= j { break None; }

        let lower = report[i];
        let higher = report[j];
        let sum = lower + higher;

        if sum == 2020 {
            break Some(report[i] * report[j]);
        } else if sum < 2020 {
            i += 1;
        } else if sum > 2020 {
            j -= 1;
        }
    }
}

fn part2(report: &Vec<i32>) -> Option<i32> {
    let mut result: Option<i32> = None;
    for i in report {
        for j in report {
            if i == j { continue; }
            for k in report {
                if i == k || j == k { continue }
                if i + j + k == 2020 {
                    result = Some(i * j * k);
                    break;
                }
            }
        }
    }
    result
}
