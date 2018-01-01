pub mod day_1;
pub mod day_2;

use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use std::cmp::Ordering;

pub fn read_input(day: &str) -> Vec<String> {
    let path = format!("./inputs/{}.txt", day);
    if !Path::new(&path).exists() {
        eprintln!("{} does not exist", path);
        exit(1);
    }
    let f = File::open(path).unwrap();
    let buf = BufReader::new(f);
    buf.lines().map( |l| l.unwrap()).collect()
}

pub enum Stars {
    Zero,
    One,
}

pub struct Progress {
    pub day: usize,
    pub stars: Stars,
}

impl Progress {

    fn parse_progress(s: String) -> Progress {
        for (i, val) in s.lines().enumerate() {
            let stars = val.chars().filter(|c| *c == '*').count();
            if stars < 2 {
                let s = match stars {
                    0 => Stars::Zero,
                    1 => Stars::One,
                    _ => panic!("SOMANYSTARS")
                };
                return Progress { day: i + 1, stars: s }
            }
        }

        return Progress { day: 25, stars: Stars::One }
    }

    pub fn status(&self) -> String {
        let stars = match self.stars {
            Stars::Zero => "first",
            Stars::One => "second"
        };
        format!("Time to get your {} star on day {}", stars, self.day)
    }

    pub fn read() -> Progress {
        match File::open("./progress.txt") {
            Ok(mut f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents)
                    .expect("something went wrong reading the file");
                Progress::parse_progress(contents)
            }
            Err(_) => Progress { day: 1, stars: Stars::Zero }
        }
    }

    pub fn increment(&mut self) {
        match self.stars {
            Stars::Zero => self.stars = Stars::One,
            Stars::One => {
                self.stars = Stars::Zero;
                self.day += 1;
            }
        }

        self.write();
    }

    pub fn write(&self) {
        let mut f = File::create("./progress.txt").unwrap();
        let stars = match self.stars {
            Stars::Zero => "",
            Stars::One => "*",
        };
        for day in 1..26 {
            match day.cmp(&self.day) {
                Ordering::Less => writeln!(f, "{} {}", day, "**").unwrap(),
                Ordering::Equal => writeln!(f, "{} {}", day, stars).unwrap(),
                Ordering::Greater => writeln!(f, "{} {}", day, "").unwrap(),
            }
        }
    }
}
