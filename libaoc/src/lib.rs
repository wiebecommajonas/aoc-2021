mod daynumber;
mod error;

use criterion::Criterion;
pub use daynumber::DayNumber;
pub use error::{AocError, DayError};

use chrono::{Local, TimeZone, Utc};
use colored::Colorize;
use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
    result,
    str::FromStr,
};

#[derive(Default)]
pub struct Aoc<const Y: usize> {
    days: HashMap<DayNumber, Box<Day<Y>>>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Day<const Y: usize> {
    number: DayNumber,
    task1: Box<dyn Fn(&str) -> Box<dyn Display>>,
    task2: Box<dyn Fn(&str) -> Box<dyn Display>>,
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
        let mut keys = self.days.keys().collect::<Vec<&DayNumber>>();
        keys.sort();
        for &day in keys {
            self.run_day(day)?;
        }
        Ok(())
    }

    pub fn bench_day(&self, day: DayNumber) -> Result<()> {
        if let Some(d) = self.days.get(&day) {
            d.bench()?;
            Ok(())
        } else {
            Err(AocError::DayNotFound(day).into())
        }
    }

    pub fn year(&self) -> usize {
        Y
    }
}

impl<const Y: usize> Day<Y> {
    pub fn new(
        day: DayNumber,
        task1: impl Fn(&str) -> Box<dyn Display> + 'static,
        task2: impl Fn(&str) -> Box<dyn Display> + 'static,
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
        println!("{}", "Part 1".bold());
        let start = std::time::Instant::now();
        let result = self.task1(&input);
        let duration = start.elapsed();
        println!("{}", result);
        println!("{}", format!("Time: {:?}", duration).bold());

        println!("{}", "Part 2".bold());
        let start = std::time::Instant::now();
        let result = self.task2(&input);
        let duration = start.elapsed();
        println!("{}", result);
        println!("{}", format!("Time: {:?}", duration).bold());
        println!();
        Ok(())
    }

    fn bench_task1(&self, c: &mut Criterion) -> Result<()> {
        let input = self.load_input()?;
        c.bench_function(&format!("Day {} part 1", self.number()), |b| {
            b.iter(|| self.task1(&input))
        });
        Ok(())
    }

    fn bench_task2(&self, c: &mut Criterion) -> Result<()> {
        let input = self.load_input()?;
        c.bench_function(&format!("Day {} part 2", self.number()), |b| {
            b.iter(|| self.task2(&input))
        });
        Ok(())
    }

    pub fn bench(&self) -> Result<()> {
        let mut criterion = Criterion::default()
            .with_output_color(true)
            .sample_size(500)
            .plotting_backend(criterion::PlottingBackend::Gnuplot)
            .with_plots();
        self.bench_task1(&mut criterion)?;
        self.bench_task2(&mut criterion)?;
        Ok(())
    }

    pub fn number(&self) -> DayNumber {
        self.number
    }

    fn task1(&self, input: &str) -> impl Display {
        (self.task1)(input)
    }

    fn task2(&self, input: &str) -> impl Display {
        (self.task2)(input)
    }
}
