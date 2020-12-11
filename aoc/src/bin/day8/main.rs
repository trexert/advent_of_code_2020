use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Operation {
    ACC,
    JMP,
    NOP,
}

#[derive(Clone, Debug)]
struct Instruction {
    pub called: bool,
    pub op: Operation,
    pub arg: i32,
}

impl Instruction {
    pub fn from_string(s: &str) -> Instruction {
        let (operator, argument) = s.split(" ").next_tuple().unwrap();
        Instruction {
            called: false,
            op: match operator {
                "acc" => Operation::ACC,
                "jmp" => Operation::JMP,
                "nop" => Operation::NOP,
                _ => panic!("Unexpected operator"),
            },
            arg: argument.parse().unwrap(),
        }
    }

    fn perform(self: &mut Instruction, mut acc: i32, mut ip: usize) -> (i32, usize) {
        self.called = true;
        match self.op {
            Operation::ACC => {
                acc += self.arg;
                ip += 1
            }
            Operation::JMP => ip = (ip as i32 + self.arg) as usize,
            Operation::NOP => ip += 1,
        }
        (acc, ip)
    }
}

fn main() {
    let instructions: Vec<_> = include_str!("input.txt")
        .lines()
        .map(Instruction::from_string)
        .collect();

    println!("part1: {}", part1(instructions.clone()));
    println!("part2: {}", part2(instructions.clone()));
}

fn part1(mut instructions: Vec<Instruction>) -> i32 {
    let (mut acc, mut ip) = (0, 0);

    while !instructions[ip].called {
        let acc_ip = instructions[ip].perform(acc, ip);
        acc = acc_ip.0;
        ip = acc_ip.1;
    }

    acc
}

fn part2(mut instructions: Vec<Instruction>) -> i32 {
    let mut variants: Vec<(i32, usize, Vec<Instruction>)> = Vec::new();
    let mut have_modified = false;
    let (mut acc, mut ip) = (0, 0);

    while ip != instructions.len() {
        if ip >= instructions.len() || instructions[ip].called {
            let snapshot = variants.pop().unwrap();
            acc = snapshot.0;
            ip = snapshot.1;
            instructions = snapshot.2;
            have_modified = true;
        }

        if !have_modified {
            match instructions[ip].op {
                Operation::JMP => {
                    let mut snapshot = instructions.clone();
                    snapshot[ip].op = Operation::NOP;
                    variants.push((acc, ip, snapshot));
                }
                Operation::NOP => {
                    let mut snapshot = instructions.clone();
                    snapshot[ip].op = Operation::JMP;
                    variants.push((acc, ip, snapshot));
                }
                _ => (),
            }
        }

        let acc_ip = instructions[ip].perform(acc, ip);
        acc = acc_ip.0;
        ip = acc_ip.1;
    }

    acc
}
