use std::io::BufReader;
extern crate itertools;
use std::io::BufRead;
use std::fs::File;
use itertools::Itertools;

fn str_list(handle: &File) -> Vec<String> {
    let file = BufReader::new(handle);

    file.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

fn count(line: &str) -> Vec<(char, usize)> {
    let mut char_list: Vec<char> = line.chars()
        .collect();

    char_list.sort();
    
    char_list.into_iter().group_by(|e| *e).into_iter().map(|(key, g)| -> (char, usize) {
	(key, g.count())
    }).collect()
}

fn combine(left: &str, right: &str) -> String {
    left.chars().zip(right.chars()).filter(|(l, r)| {
        l == r
    }).map(|(l, _)| l).into_iter().collect()
}

fn value(line: Vec<(char, usize)>) -> (i32, i32) {
    let list: Vec<usize> = line.iter().map(|(_, s)| *s)
        .dedup().collect();

    let twos = list.iter().filter(|&n| *n == 2).count();
    let threes = list.iter().filter(|&n| *n == 3).count();

    (twos as i32, threes as i32)
}

fn hamming(left: &str, right: &str) -> i32 {
    left.chars().zip(right.chars()).map(|(l, r)| (l != r) as i32).sum()
}

fn main() {
    let f = File::open("input.txt").unwrap();

    let list: Vec<String> = str_list(&f);

    let letter_count: (i32, i32) = list.iter()
        .map(|s| {
            let mut curr: Vec<(char, usize)> = count(&s); // (char, count)
            curr.retain(|(_, s)| s == &2 || s == &3); // Remove <2 and >3 counts
            value(curr) // Remove duplicates, count twos and threes
        })
        .fold((0,0), |acc, (twos, threes)| (acc.0 + twos, acc.1 + threes)); // Sums all pairs

    println!("Twos: {} Threes: {}", letter_count.0, letter_count.1);
    println!("Product: {}", letter_count.0 * letter_count.1);

    let mut matching_pair: (String, String) = ("".to_string(), "".to_string());
    for s1 in list.iter() {
        for s2 in list.iter() {
            if hamming(&s1, &s2) == 1 {
                matching_pair = (s1.to_string(), s2.to_string());
            }
        }
    }

    println!("Proto fabric: {:?}", combine(&matching_pair.0, &matching_pair.1));
}
