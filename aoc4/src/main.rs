extern crate regex;
extern crate chrono;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;
use chrono::{NaiveDate, NaiveDateTime};

fn str_list(handle: &File) -> Vec<String> {
    let file = BufReader::new(handle);

    file.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

// [1518-11-01 00:05] -> NaiveDateTime
fn to_date(line: &str) -> (NaiveDateTime, String)
{
    let caps = Regex::new(r"\[([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)\] (.+)").unwrap();
    let cap = caps.captures(line).unwrap();

    let list: Vec<_> = cap.iter()
        .skip(1)
        .map(|c| c.unwrap().as_str())
//        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    
    (NaiveDate::from_ymd(list[0].parse::<i32>().unwrap(),
                         list[1].parse::<u32>().unwrap(),
                         list[2].parse::<u32>().unwrap())
     .and_hms(
         list[3].parse::<u32>().unwrap(),
         list[4].parse::<u32>().unwrap(), 0),
     list[5].to_string())
}

fn main() {
    let f = File::open("input.txt").unwrap();

    let lines = str_list(&f);

    for line in lines {
        let (date, msg) = to_date(&line);
        println!("Date: {:?} Msg: {}", date, msg);
    }
}
