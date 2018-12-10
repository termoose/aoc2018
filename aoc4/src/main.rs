extern crate itertools;
extern crate regex;
extern crate chrono;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;
use chrono::{NaiveDate, NaiveDateTime};
use std::collections::HashMap;
use chrono::Timelike;
use itertools::Itertools;

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
    // Guard -> (duration, [minute])
    let mut guards: HashMap<usize, (i64, Vec<u32>)> = HashMap::new();
    let f = File::open("input.txt").unwrap();

    let mut lines: Vec<(NaiveDateTime, String)> = str_list(&f).iter()
        .map(|l| to_date(&l))
        .collect();

    // Sort by the date
    lines.sort_by(|(d1, _), (d2, _)| d1.cmp(d2));

    let mut current_guard = 0;
    let mut sleep_start: NaiveDateTime = NaiveDate::from_ymd(200, 1, 1)
        .and_hms(0, 0, 0);
    for (date, msg) in lines {
        let action = to_action(&msg);

        match action {
            GuardAction::StartDuty(id) => {
                current_guard = id;
                //println!("Date: {:?} Id: {:?}", date, id);
            }
            GuardAction::FallAsleep => {
                // He slept for 0 minutes so far
                guards.entry(current_guard).or_insert((0, vec![]));

                // Assign sleep start
                sleep_start = date;
            }
            GuardAction::WakeUp => {
                let dur = date.signed_duration_since(sleep_start).num_minutes();
                let (m, mins) = guards.entry(current_guard).or_insert((0, vec![]));

                // Add total duration
                *m += dur;

                let start_min = sleep_start.time().minute();
                let stop_min = date.time().minute();

                for min in start_min..stop_min {
                    mins.push(min);
                }
            }
        }
    }

    let (sleepy_guard, (_dur, hours)) = guards.iter()
        .max_by(|(_, (dur1, _)), (_, (dur2, _))| {
            dur1.cmp(dur2)
        }).unwrap();

    for (guard, (_, hours)) in &guards {
        let mut sorted_hours = hours.clone();
        sorted_hours.sort();

        let grp: Vec<(&u32, usize)> = sorted_hours.iter()
            .group_by(|e| *e)
            .into_iter()
            .map(|(_, elem)| elem.collect())
            .map(|list: Vec<&u32>| (list[0], list.len()))
            .collect();

        let max_min = grp.iter().max_by(|(_h1, c1), (_h2, c2)| {
            c1.cmp(c2)
        });

        println!("Guard {} Max min: {:?}", guard, max_min);
    }

    println!("Most sleepy: {:?}", sleepy_guard);

    let mut sleep_hours: Vec<u32> = hours.clone();

    sleep_hours.sort();

    let grouped: Vec<(&u32, usize)> = sleep_hours.iter()
        .group_by(|e| *e)
        .into_iter()
        .map(|(_, elem)| elem.collect())
        .map(|list: Vec<&u32>| (list[0], list.len()))
        .collect();

    let max_hour = grouped.iter().max_by(|(_h1, c1), (_h2, c2)| {
        c1.cmp(c2)
    });
    println!("Sorted: {:?}", grouped);
    println!("Max hour: {:?}", max_hour);

    // for (guard_id, (duration, mins)) in guards {
    //     println!("Guard {} sleep {} mins: {:?}", guard_id, duration, mins);
    // }
}
