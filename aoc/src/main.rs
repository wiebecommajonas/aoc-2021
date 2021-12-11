#![feature(drain_filter)]

mod days;
mod utils;

use clap::{load_yaml, App};
use dotenv::dotenv;
use libaoc::{Aoc, DayNumber};

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from(yaml).get_matches();

    dotenv().ok();

    let mut aoc = Aoc::<2021>::new();

    aoc.add_day(days::one());
    aoc.add_day(days::two());
    aoc.add_day(days::three());
    aoc.add_day(days::four());
    aoc.add_day(days::five());
    aoc.add_day(days::six());
    aoc.add_day(days::seven());
    aoc.add_day(days::eight());
    aoc.add_day(days::nine());
    aoc.add_day(days::ten());

    if let Some(("bench", ms)) = matches.subcommand() {
        if let Some(values) = ms.values_of("bench_day") {
            for i in values {
                if let Err(e) = aoc.bench_day(DayNumber::from(i)) {
                    eprintln!("{}", e);
                }
            }
        }
        return;
    }

    if let Some(("run", ms)) = matches.subcommand() {
        if let Some(values) = ms.values_of("run_day") {
            for i in values {
                if let Err(e) = aoc.run_day(DayNumber::from(i)) {
                    eprintln!("{}", e);
                }
            }
        }

        if ms.is_present("run_all") {
            if let Err(e) = aoc.run_days_all() {
                println!("{}", e)
            }
        }

        return;
    }
}
