extern crate regex;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;

// #14 @ 662,511: 24x10
type Patch = (usize, (usize, usize), (usize, usize));

fn str_list(handle: &File) -> Vec<String> {
    let file = BufReader::new(handle);

    file.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

fn line_to_tuple(line: &str) -> Patch {
    let caps = Regex::new(r"\#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$")
        .unwrap();
    let cap = caps.captures(line).unwrap();

    let list: Vec<usize> = cap.iter()
        .skip(1)
        .map(|c| c.unwrap().as_str())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    (list[0], (list[1], list[2]), (list[3], list[4]))
}

fn attach(suit: &mut Vec<Vec<u8>>, patch: Patch) {
    let size = patch.2;
    let pos = patch.1;
    for x in 1..(size.0+1) {
        for y in 1..(size.1+1) {
            suit[pos.0 + x][pos.1 + y] += 1;
        }
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut suit = vec![vec![0 as u8; 1000]; 1000];

    // Stitch suit
    for line in str_list(&f) {
        attach(&mut suit, line_to_tuple(&line));
    }

    let reduced: Vec<usize> = suit.iter()
        .map(|v| {
            v.iter().filter(|&elem| *elem > 1).count()
        }).collect();

    println!("Length: {}", reduced.iter().fold(0, |acc, e| acc + e));
}
