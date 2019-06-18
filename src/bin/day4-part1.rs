extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Eq)]
#[derive(Debug)]
struct Record {
    idx: usize,
    ord: usize,
    id: Option<i32>,
    timestamp: String,
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
    hour: Option<i32>,
    minute: Option<i32>,
    wakes: bool,
    sleeps: bool,
}

#[derive(Debug)]
struct GuardSleep {
    id: i32,
    mins_slept: i32,
    last_min_slept: Option<i32>,
    sleep_mins_count: Vec<i32>
}


impl Ord for Record {
    fn cmp(&self, other: &Record) -> Ordering {
        self.timestamp.cmp(&other.timestamp).then(self.ord.cmp(&other.ord))
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Record) -> bool {
        self.idx == other.idx
    }
}

fn main()-> std::io::Result<()> {
    let mut file = File::open("../../day-4-input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut entries = BTreeSet::new();
    let mut guard_sleep: HashMap<i32, GuardSleep> = HashMap::new();

    println!("lines: {}", lines.len());

    for (idx, line) in lines.iter().enumerate() {
        let re = Regex::new(r"\[(?P<dt>(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2}))\] (?P<entry>Guard #(?P<id>\d+?) begins shift|wakes up|falls asleep)").unwrap();
        let capture = re.captures(line).unwrap();
        let year = match capture.name("year") {None => None, Some(x) => Some(x.as_str().parse::<i32>().unwrap())};
        let month = match capture.name("month") {None => None, Some(x) => Some(x.as_str().parse::<i32>().unwrap())};
        let day = match capture.name("day") {None => None, Some(x) => Some(x.as_str().parse::<i32>().unwrap())};
        let hour = match capture.name("hour") {None => None, Some(x) => Some(x.as_str().parse::<i32>().unwrap())};
        let minute = match capture.name("minute") {None => None, Some(x) => Some(x.as_str().parse::<i32>().unwrap())};
        let id = match capture.name("id") {None => None, Some(x) => Some(x.as_str().parse::<i32>().unwrap())};

        entries.insert(
            Record {
                idx: idx,
                ord: match capture.name("entry").unwrap().as_str() {"wakes up" => 3, "falls asleep" => 2, _ => 1},
                id: id,
                timestamp: capture.name("dt").unwrap().as_str().to_string(),
                year: year,
                month: month,
                day: day,
                hour: hour,
                minute: minute,
                wakes: match capture.name("entry") {None => false, Some(x) => x.as_str() == "wakes up"},
                sleeps: match capture.name("entry") {None => false, Some(x) => x.as_str() == "falls asleep"},
            }
        );
    }

    let mut latest = 0;
    let mut most_mins_slept = 0;
    let mut most_mins_slept_id = 0;
    for entry in entries.iter() {
        match entry.id {None => {}, Some(x) => {latest = x; if !guard_sleep.contains_key(&x) {guard_sleep.insert(x, GuardSleep{id:x, mins_slept:0, last_min_slept:None, sleep_mins_count: vec![0; 60]});}}}

        if entry.sleeps == true {
            guard_sleep.entry(latest)
                .and_modify
                (|last_sleep|
                 {
                     *last_sleep = GuardSleep{id: latest, mins_slept: last_sleep.mins_slept, last_min_slept: entry.minute, sleep_mins_count: last_sleep.sleep_mins_count.clone()};
                 });
        }

        if entry.wakes == true {
            guard_sleep.entry(latest)
                .and_modify
                (|last_sleep|
                 {
                     let mins_slept = last_sleep.mins_slept + (entry.minute.unwrap() - last_sleep.last_min_slept.unwrap());
                     let mut sleep_mins_count = last_sleep.sleep_mins_count.clone();

                     for min in last_sleep.last_min_slept.unwrap()..entry.minute.unwrap() {
                         sleep_mins_count[min as usize] = sleep_mins_count[min as usize] + 1;
                     }

                     *last_sleep = GuardSleep{id: latest, mins_slept: mins_slept, last_min_slept: None, sleep_mins_count: sleep_mins_count};
                 });

            let sleep = guard_sleep.get(&latest).unwrap();
            if most_mins_slept < sleep.mins_slept {
                most_mins_slept = sleep.mins_slept;
                most_mins_slept_id = sleep.id;
            }
        }
    }

    println!("most slept: {:?}", guard_sleep.get(&most_mins_slept_id));
    Ok(())
}
