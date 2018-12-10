use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::prelude::*;

fn file_to_string(mut handle: &File) -> String {
    let mut contents = String::new();
    handle.read_to_string(&mut contents).unwrap();

    contents
}

fn toggle(c: char) -> String {
    if c.is_uppercase() {
        return c.to_lowercase().to_string();
    }

    c.to_uppercase().to_string()
}

fn combine(left: char, right: char) -> Option<String> {
    let left_other: String = toggle(left);
    let right_other: String = toggle(right);

    if left_other == right.to_string() || right_other == left.to_string() {
        return None
    }

    Some([left, right].into_iter().collect())
}

fn scan(list: Vec<char>) -> String {
    let mut result: String = String::new();

    for i in 0..(list.len() - 1) {
        let first = list[i];
        let second = list[i+1];

        let combined = combine(first, second);

        match combined {
            Some(x) => {
                result += &x;
            },
            None => {}
        }
    }

    result
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let file = file_to_string(&f);

    println!("Scan: {}", scan(file.chars().collect()));
    println!("Combine: {:?}", combine('a', 'A'));
    println!("Combine: {:?}", combine('a', 'B'));
}
