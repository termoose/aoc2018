extern crate regex;
extern crate chrono;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
enum GuardAction {
    FallAsleep,
    WakeUp,
    StartDuty(usize)        
}

fn str_list(handle: &File) -> Vec<String> {
    let file = BufReader::new(handle);

    file.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

fn to_action(msg: &str) -> GuardAction {
    let caps_wakeup = Regex::new(r"Guard \#([0-9]+).+").unwrap();

    if caps_wakeup.is_match(&msg) {
        let cap = caps_wakeup.captures(msg).unwrap();
        let id: usize = cap[1].parse::<usize>().unwrap();
        return GuardAction::StartDuty(id);
    }
    else if msg == "wakes up" {
        return GuardAction::WakeUp;
    }

    return GuardAction::FallAsleep;
}

// [1518-11-01 00:05] -> NaiveDateTime
fn to_date(line: &str) -> (NaiveDateTime, String)
{
    let caps = Regex::new(r"\[([0-9]+)-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)\] (.+)").unwrap();
    let cap = caps.captures(line).unwrap();

    let list: Vec<_> = cap.iter()
        .skip(1)
        .map(|c| c.unwrap().as_str())
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

    let mut lines: Vec<(NaiveDateTime, String)> = str_list(&f).iter()
        .map(|l| to_date(&l))
        .collect();

    // Sort by the date
    lines.sort_by(|(d1, _), (d2, _)| d1.cmp(d2));

    for (date, msg) in lines {
        let action = to_action(&msg);
        println!("Date: {:?} Action: {:?}", date, action);
    }
}
