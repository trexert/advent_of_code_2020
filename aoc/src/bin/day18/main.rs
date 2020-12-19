use std::str::Chars;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    PLUS,
    TIMES,
}

impl Operator {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Operator::PLUS => left + right,
            Operator::TIMES => left * right,
        }
    }
}

fn eval(mut input: &mut Chars, low_prec: Option<Operator>) -> u64 {
    let mut value = 0;
    let mut op = Operator::PLUS;

    let mut delayed_terms: Vec<u64> = Vec::new();

    while let Some(c) = input.next() {
        match c {
            ' ' => {}
            '+' => op = Operator::PLUS,
            '*' => op = Operator::TIMES,
            '(' => {
                value = match low_prec {
                    Some(low_prec_op) if low_prec_op == op => {
                        delayed_terms.push(value);
                        eval(&mut input, low_prec)
                    }
                    _ => op.apply(value, eval(&mut input, low_prec)),
                }
            }
            ')' => break,
            digit => {
                value = match low_prec {
                    Some(low_prec_op) if low_prec_op == op => {
                        delayed_terms.push(value);
                        digit.to_digit(10).unwrap() as u64
                    }
                    _ => op.apply(value, digit.to_digit(10).unwrap() as u64),
                }
            }
        }
    }

    if let Some(low_prec_op) = low_prec {
        delayed_terms
            .iter()
            .fold(value, |acc, term| low_prec_op.apply(acc, *term))
    } else {
        value
    }
}

fn main() {
    let expressions: Vec<_> = include_str!("input.txt").lines().collect();

    println!("part1: {}", part1(&expressions));
    println!("part2: {}", part2(&expressions));
}

fn part1(expressions: &Vec<&str>) -> u64 {
    expressions
        .iter()
        .map(|expression| eval(&mut expression.chars(), None))
        .sum()
}

fn part2(expressions: &Vec<&str>) -> u64 {
    expressions
        .iter()
        .map(|expression| eval(&mut expression.chars(), Some(Operator::TIMES)))
        .sum()
}
