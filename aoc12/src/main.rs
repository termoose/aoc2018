use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::time::{SystemTime};

type Mutation = (u8, bool);

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
    (to_bits(parts[0]) as u8, to_bits(parts[2]) == 0)
}

fn get_bits(input: u128, n: i32) -> u8 {
    (input >> (LENGTH - n - (MASK_SIZE + 1)) & MASK) as u8
}

fn set_bit(input: u128, pos: i32) -> u128 {
    input | (1 << pos)
}

fn set_bit_val(input: u128, pos: i32, bit: bool) -> u128 {
    input & (!(1 << pos)) | ((bit as u128) << pos)
}

fn unset_bit(input: u128, pos: i32) -> u128 {
    input & !(1 << pos)
}

fn get_bit(input: u128, n: i32) -> bool {
    ((input & (1 << n)) >> n) == 1
}

fn count(input: u128, zero_pos: i32, length: i32) -> i32 {
    let mut sum = 0;
    for n in 0..length {
        let bit = get_bit(input, length - n - 1);

        sum += (n - zero_pos) * bit as i32;
    }

    sum
}

fn multi_mutate(input: u128, mutations: &Vec<Mutation>) -> u128 {
    let mut result: u128 = input;

    for i in 0..(LENGTH - MASK_SIZE) {
        let bits = get_bits(input, i);

        for (pattern, c) in mutations {
            if bits == *pattern {
                //result = set_bit_val(result, LENGTH - 1 - i - 3, !*c);
                if *c {
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

fn useful_mutate(m: Mutation) -> bool {
    (m.0 & 0b00100 == 0b00100) == m.1
}

fn main() {
    let input_start = SystemTime::now();

    let f = File::open("input.txt").unwrap();
    let lines = str_list(&f);
    let init_state: u128 = to_init_state(&lines[0]);

    // Pad with some bits on each end
    let init_state_padded = init_state << 28;

    let mutations: Vec<Mutation> = lines[2..].iter()
        .map(|l| mutation(l))
        .filter(|m| useful_mutate(*m))
        .collect();

    let mut result: u128 = init_state_padded;
    
    let input_end = SystemTime::now();
    let input_duration = input_end.duration_since(input_start).unwrap();
    let input_ms = input_duration.subsec_micros() as u64;
    println!("Runtime init: {} µs", input_ms);

    let start = SystemTime::now();

    for _ in 0..20 {
        result = multi_mutate(result, &mutations);
    }

    let sum = count(result, 5, LENGTH);

    let finished = SystemTime::now();
    let duration = finished.duration_since(start).unwrap();
    let ms = duration.subsec_micros() as u64;
    
    println!("{} Runtime P1: {} µs", sum, ms);
}
