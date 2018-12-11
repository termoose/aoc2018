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

fn combine(left: char, right: char) -> bool {
    let left_other: String = toggle(left);
    let right_other: String = toggle(right);
    if left_other == right.to_string() || right_other == left.to_string() {
        return false
    }

    true
}

fn remove(list: Vec<char>, letter: char) -> String {
    list.iter()
        .filter(|c| **c != letter && (**c).to_string() != toggle(letter)).collect()
}

fn scan(list: Vec<char>) -> String {
    let mut result: String = String::new();

    let mut i = 0;
    while i < list.len() {
        if i == list.len() - 1 {
            result += &list[i].to_string();
            break;
        }
        let first = list[i];
        let second = list[i+1];

        let combined = combine(first, second);

        match combined {
            true => {
                result += &first.to_string();
                i += 1;
            },
            false => {
                i += 2;
            }
        }
    }

    result
}

fn solve(list: String) -> usize {
    let mut curr_string: String = String::new();
    let mut prev_string: String = list.trim().to_string();

    loop {
        curr_string = scan(prev_string.trim().chars().collect());

        if(curr_string.len() == prev_string.len()) {
            return curr_string.len();
        }

        prev_string = curr_string;
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let file = file_to_string(&f);

    // let mut curr_string: String = String::new();
    // let mut prev_string: String = file.trim().to_string();

    for n in b'a'..(b'z'+1) {
        //println!("Char: {}", n as char);
        println!("{}: {}", n as char,
                 solve(remove(file.chars().collect(), n as char)));
    }

    //println!("Remove: {}", remove(vec!['a', 'a', 'b'], 'B'));

    // loop {
    //     curr_string = scan(prev_string.trim().chars().collect());
    //     if curr_string.len() == prev_string.len() {
    //         println!("Solution length {}", curr_string.len());
    //         break;
    //     }

    //     prev_string = curr_string;
    // }
}
