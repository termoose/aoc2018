extern crate regex;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;
use std::collections::HashSet;

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

fn attach(suit: &mut Vec<Vec<(usize, usize, Vec<usize>)>>, patch: Patch) {
    let size = patch.2;
    let pos = patch.1;
    let id = patch.0;

    for x in 1..(size.0+1) {
        for y in 1..(size.1+1) {
            let claim_count: &mut (usize, usize, Vec<usize>) = &mut suit[pos.0 + x][pos.1 + y];

            claim_count.2.push(id);
            claim_count.1 += 1;
            claim_count.0 = id;
        }
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = str_list(&f);
    let mut suit = vec![vec![(0 as usize, 0 as usize, vec![]); 1000]; 1000];

    // Stitch suit
    for line in &lines {
        attach(&mut suit, line_to_tuple(&line));
    }

    let reduced: Vec<usize> = suit.iter()
        .map(|v| {
            v.iter().filter(|&elem| elem.1 > 1).count()
        }).collect();

    let mut overlaps: HashSet<usize> = HashSet::new();

    for patch in suit {
        for (_id, count, ids) in patch {
            if count > 1 {
                for i in ids {
                    overlaps.insert(i);
                }
            }
        }
    }

    let mut all: HashSet<usize> = HashSet::new();
    for line in &lines {
        let l: Patch = line_to_tuple(&line);
        all.insert(l.0);
    }

    let diff: HashSet<&usize> = all.difference(&overlaps).collect();
    println!("Overlaps: {:?}", diff);
    println!("Length: {}", reduced.iter().fold(0, |acc, e| acc + e));
}
