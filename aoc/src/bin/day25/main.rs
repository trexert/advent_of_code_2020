use num::traits::{One, Zero};
use num::{Bounded, Num};
use std::collections::HashSet;
use std::ops::ShrAssign;

const MODULUS: u64 = 20201227;
const SUBJECT: u64 = 7;

fn main() {
    let keys: Vec<u64> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("part1: {}", get_final_key(&keys));
}

fn get_final_key(keys: &Vec<u64>) -> u64 {
    let mut to_find: HashSet<u64> = keys.iter().cloned().collect();
    let mut key_to_be = 1u64;
    let mut loop_sizes = HashSet::with_capacity(to_find.len());
    let mut loops = 0u64;

    while !to_find.is_empty() {
        if to_find.contains(&key_to_be) {
            loop_sizes.insert(loops);
            to_find.remove(&key_to_be);
        }
        loops += 1;
        key_to_be = (key_to_be * SUBJECT) % MODULUS;
    }
    let total_power = loop_sizes.iter().fold(1u64, |acc, &size| acc * size);
    SUBJECT.mod_power(total_power, MODULUS)
}

trait ModPower: Sized + Num + Copy + Bounded + ShrAssign + PartialOrd {
    fn mod_power(&self, exponent: Self, modulus: Self) -> Self {
        let zero = Zero::zero();
        let one = One::one();
        let two = one + one;
        let mut base = *self % modulus;
        let mut result = one;
        let mut exp = exponent;
        while exp > zero {
            if exp % two == one {
                result = (result * base) % modulus;
            }

            exp >>= one;
            base = (base * base) % modulus;
        }
        result
    }
}

impl ModPower for u64 {}
