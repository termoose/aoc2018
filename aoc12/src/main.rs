extern crate regex;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;

type Mutation = (u128, u128);

const MASK: u128 = 0b11111;
const MASK_SIZE: i32 = 5;
const LENGTH: i32 = 128;

fn str_list(handle: &File) -> Vec<String> {
    let file = BufReader::new(handle);

    file.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

fn to_bits(line: &str) -> u128 {
    let bitstring = line.replace("#", "1").replace(".", "0");
    u128::from_str_radix(&bitstring, 2).unwrap()
}

fn to_init_state(line: &str) -> u128 {
    let parts: Vec<&str> = line.split(' ').collect();

    to_bits(parts[2])
}

fn mutation(line: &str) -> Mutation {
    let parts: Vec<&str> = line.split(' ').collect();
    (to_bits(parts[0]), to_bits(parts[2]))
}

fn get_bits(input: u128, n: i32) -> u128 {
    input >> (LENGTH - n - (MASK_SIZE + 1)) & MASK
}

fn set_bit(input: u128, pos: i32) -> u128 {
    input | (1 << pos)
}

fn unset_bit(input: u128, pos: i32) -> u128 {
    input & !(1 << pos)
}

fn get_bit(input: u128, n: i32) -> u128 {
    (input & (1 << n)) >> n
}

fn count(input: u128, zero_pos: i32, length: i32) -> i32 {
    let mut sum = 0;
    for n in 0..length {
        let bit = get_bit(input, length - n - 1);
        let position = n - zero_pos;

        if bit == 1 {
            sum += position;
        }
    }

    sum
}

fn multi_mutate(input: u128, mutations: &Vec<Mutation>) -> u128 {
    let mut result: u128 = input;

    for i in 0..(LENGTH - MASK_SIZE) {
        let bits = get_bits(input, i);

        for (pattern, c) in mutations {
            if bits == *pattern {
                if c == &0 {
                    result = unset_bit(result, LENGTH - 1 - i - 3);
                }
                else {
                    result = set_bit(result, LENGTH - 1 - i - 3);
                }
            }
        }
    }

    result
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = str_list(&f);
    let init_state: u128 = to_init_state(&lines[0]);

    // Pad with 16 bits on each end
    let init_state_padded = init_state << 28;

    let mutations: Vec<Mutation> = lines[2..].iter()
        .map(|l| mutation(l))
        .collect();

    println!("    {:0128b}", init_state_padded);

    let mut result: u128 = init_state_padded;
    for i in 0..20 {
        result = multi_mutate(result, &mutations);

        println!("{:2}: {:0128b}", i, result);
    }

    println!("\n{:0128b}", result);

    let sum = count(result, 5, LENGTH);
    println!("Sum: {}", sum);
}
