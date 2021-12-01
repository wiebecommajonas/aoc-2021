mod daynumber;
mod error;

pub use daynumber::DayNumber;
pub use error::{AocError, DayError};

use chrono::{Local, TimeZone, Utc};
use colored::Colorize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Default)]
pub struct Aoc<const Y: usize> {
    days: HashMap<DayNumber, Box<Day<Y>>>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Day<const Y: usize> {
    number: DayNumber,
    task1: Box<dyn Fn(&str)>,
    task2: Box<dyn Fn(&str)>,
    input_url: String,
}

impl<const Y: usize> Aoc<Y> {
    pub fn new() -> Self {
        Self {
            days: HashMap::new(),
        }
    }

    pub fn add_day(&mut self, day: Day<Y>) {
        self.days.insert(day.number(), Box::new(day));
    }

    pub fn run_day(&self, day: DayNumber) -> Result<()> {
        if let Some(d) = self.days.get(&day) {
            d.run()?;
            Ok(())
        } else {
            Err(AocError::DayNotFound(day).into())
        }
    }

    pub fn run_days(&self, days: &[DayNumber]) -> Result<()> {
        for &day in days {
            self.run_day(day)?;
        }
        Ok(())
    }

    pub fn run_days_all(&self) -> Result<()> {
        for &day in self.days.keys() {
            self.run_day(day)?;
        }
        Ok(())
    }

    pub fn year(&self) -> usize {
        Y
    }
}

impl<const Y: usize> Day<Y> {
    pub fn new(
        day: DayNumber,
        task1: impl Fn(&str) + 'static,
        task2: impl Fn(&str) + 'static,
    ) -> Self {
        Self {
            number: day,
            task1: Box::new(task1),
            task2: Box::new(task2),
            input_url: format!("https://www.adventofcode.com/{}/day/{}/input", Y, day),
        }
    }

    fn load_input(&self) -> Result<String> {
        if Local::now().with_timezone(&Utc)
            < Utc.ymd(Y as i32, 12, self.number().into()).and_hms(5, 0, 0)
        {
            return Err(DayError::NotAvailable(self.number()).into());
        }

        let store_string = format!("./inputs/{}/{}", Y, self.number());
        let mut store_path = PathBuf::from_str(&store_string).unwrap();
        std::fs::create_dir_all(Path::new(&store_path)).expect("Failed to create directories");
        store_path.push("input.txt");

        if !store_path.exists() {
            let sessionid = match std::env::var("CURL_AOC_SESSION") {
                Ok(id) => id,
                Err(_) => return Err(DayError::SessionNotSet.into()),
            };
            let mut curl = curl::easy::Easy::new();
            curl.url(&self.input_url).unwrap();
            curl.follow_location(true).unwrap();
            curl.cookie(&format!("session={}", sessionid)).unwrap();
            let mut transfer = curl.transfer();
            transfer
                .write_function(|input| {
                    std::fs::write(Path::new(&store_path), input).expect("Failed to write input");
                    Ok(input.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }
        Ok(std::fs::read_to_string(Path::new(&store_path)).expect("Unable to read file"))
    }

    pub fn run(&self) -> Result<()> {
        let input = self.load_input()?;
        println!("{}", format!("Running Day {}", self.number()).bold());
        println!("Task 1");
        let start = std::time::Instant::now();
        self.task1(&input);
        let duration = start.elapsed();
        println!("{}", format!("Time: {:?}", duration).bold());

        println!("Task 2");
        let start = std::time::Instant::now();
        self.task2(&input);
        let duration = start.elapsed();
        println!("{}", format!("Time: {:?}", duration).bold());
        Ok(())
    }

    pub fn number(&self) -> DayNumber {
        self.number
    }

    fn task1(&self, input: &str) {
        (self.task1)(input);
    }

    fn task2(&self, input: &str) {
        (self.task2)(input);
    }
}
