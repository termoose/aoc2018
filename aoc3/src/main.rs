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

fn line_to_tuple(line: &str) -> (i32, (i32, i32), (i32, i32)) {
    let caps = Regex::new(r"\#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$")
        .unwrap();
    let cap = caps.captures(line).unwrap();

    let list: Vec<i32> = cap.iter()
        .skip(1)
        .map(|c| c.unwrap().as_str())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    (list[0], (list[1], list[2]), (list[3], list[4]))
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let test = "#1 @ 669,271: 17x11";
    let r = Regex::new(r"\#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$")
        .unwrap();

    println!("Matches: {:?}", line_to_tuple(test));
}
