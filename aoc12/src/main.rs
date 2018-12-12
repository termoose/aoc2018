extern crate regex;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;

fn str_list(handle: &File) -> Vec<String> {
    let file = BufReader::new(handle);

    file.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

fn init_state(line: &str) -> u128 {
    let parts: Vec<&str> = line.split(' ').collect();
    let bitstring = parts[2].replace("#", "1").replace(".", "0");

    u128::from_str_radix(&bitstring, 2).unwrap()
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = str_list(&f);

    let init_state: u128 = init_state(&lines[0]);
    println!("Init state: {}", init_state);
}
