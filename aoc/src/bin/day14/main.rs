use regex::Regex;
use std::collections::HashMap;

trait MaskLike {
    fn from_string(from: &str) -> Self
    where
        Self: Sized;
    fn apply_to(&self, subject: u64) -> Vec<u64>;
}

struct Mask {
    _str_mask: String,
    ones_mask: u64,
    zeros_mask: u64,
}

impl MaskLike for Mask {
    fn from_string(from: &str) -> Mask {
        let mut ones = 0;
        let mut zeros = (1 << 36) - 1;
        from.chars().enumerate().for_each(|(i, c)| match c {
            '0' => zeros = !(!zeros | 1 << (from.len() - i - 1)),
            '1' => ones |= 1 << (from.len() - i - 1),
            'X' => {}
            _ => panic!("Unexpected character in mask"),
        });
        Mask {
            _str_mask: from.to_string(),
            ones_mask: ones,
            zeros_mask: zeros,
        }
    }

    fn apply_to(&self, subject: u64) -> Vec<u64> {
        let mut result = subject;
        result |= self.ones_mask;
        result &= self.zeros_mask;
        vec![result]
    }
}

struct MaskV2 {
    str_mask: String,
}

impl MaskLike for MaskV2 {
    fn from_string(from: &str) -> MaskV2 {
        MaskV2 {
            str_mask: from.to_string(),
        }
    }

    fn apply_to(&self, subject: u64) -> Vec<u64> {
        let mut results = vec![0];
        self.str_mask
            .chars()
            .rev()
            .enumerate()
            .for_each(|(i, c)| match c {
                '0' => {
                    let new_bit = (1 << i) & subject;
                    if new_bit > 0 {
                        results.iter_mut().for_each(|val| *val |= new_bit);
                    }
                }
                '1' => {
                    let new_bit = 1 << i;
                    results.iter_mut().for_each(|val| *val |= new_bit);
                }
                'X' => {
                    let new_bit = 1 << i;
                    let mut new_results = results.clone();
                    new_results.iter_mut().for_each(|val| *val |= new_bit);
                    results.extend(new_results);
                }
                _ => panic!("Unexpected character in mask"),
            });
        results
    }
}

struct Port {
    mask: Box<dyn MaskLike>,
    mem: HashMap<u64, u64>,
    mask_regex: Regex,
    mem_regex: Regex,
    version: u8,
}

impl Port {
    pub fn new() -> Port {
        Port {
            mask: Box::new(Mask::from_string("")),
            mem: HashMap::new(),
            mask_regex: Regex::new(r"mask = ([01X]{36})").unwrap(),
            mem_regex: Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap(),
            version: 0,
        }
    }

    pub fn new_v2() -> Port {
        let mut port = Port::new();
        port.mask = Box::new(MaskV2::from_string(""));
        port.version = 1;
        port
    }

    pub fn perform(&mut self, command: &str) {
        if let Some(mask_caps) = self.mask_regex.captures(command) {
            self.mask = match self.version {
                0 => Box::new(Mask::from_string(&mask_caps[1])),
                1 => Box::new(MaskV2::from_string(&mask_caps[1])),
                _ => panic!("Unexpected version"),
            }
        } else if let Some(mem_caps) = self.mem_regex.captures(command) {
            match self.version {
                0 => {
                    self.mem.insert(
                        mem_caps[1].parse().unwrap(),
                        self.mask.apply_to(mem_caps[2].parse().unwrap())[0],
                    );
                }
                1 => {
                    let value = mem_caps[2].parse().unwrap();
                    self.mask
                        .apply_to(mem_caps[1].parse().unwrap())
                        .iter()
                        .for_each(|addr| {
                            self.mem.insert(*addr, value);
                        });
                }
                _ => panic!("Unexpected version"),
            }
        } else {
            panic!("Unexpeted command format: {}", command);
        }
    }

    pub fn sum_mem(&self) -> u64 {
        self.mem.values().fold(0, |acc, val| acc + val)
    }
}

fn main() {
    let commands: Vec<_> = include_str!("input.txt").lines().collect();
    println!("part1: {}", part1(&commands));
    println!("part2: {}", part2(&commands));
}

fn part1(commands: &Vec<&str>) -> u64 {
    let mut port = Port::new();
    commands.iter().for_each(|command| port.perform(command));
    port.sum_mem()
}

fn part2(commands: &Vec<&str>) -> u64 {
    let mut port = Port::new_v2();
    commands.iter().for_each(|command| port.perform(command));
    port.sum_mem()
}
