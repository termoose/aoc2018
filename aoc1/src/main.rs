use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

fn number_list(handle: &File) -> Vec<i32> {
    let file = BufReader::new(handle);
    
    file.lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect()
}

fn sum(handle: &File) -> i32 {
    number_list(&handle).iter().sum()
}

fn first_freq(handle: &File) -> Option<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    let numbers = number_list(&handle);
    let cycle = numbers.iter().cycle();

    let mut acc = 0;
    for number in cycle {
        acc += number;
        if !set.insert(acc) {
            return Some(acc);
        }
    }

    None
}

fn main() {
    let f = File::open("input.txt").unwrap();
    
    //println!("Sum: {}", sum(&f));
    println!("Loop: {}", first_freq(&f).unwrap());
}
