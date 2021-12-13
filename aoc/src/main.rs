#![feature(drain_filter)]

mod cli;
mod days;
mod utils;

use crate::cli::Commands;
use clap::StructOpt;
use dotenv::dotenv;
use libaoc::{Aoc, DayNumber};

fn main() {
    let cli = cli::Cli::parse();

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
    aoc.add_day(days::eleven());
    aoc.add_day(days::twelve());
    aoc.add_day(days::thirteen());

    match cli.command {
        Commands::Run {
            day: days,
            all: false,
        } => {
            for i in days {
                if let Err(e) = aoc.run_day(DayNumber::from(i)) {
                    eprintln!("{}", e);
                }
            }
        }
        Commands::Run { day: _, all: true } => {
            if let Err(e) = aoc.run_days_all() {
                println!("{}", e)
            }
        }
        Commands::Bench { day: days } => {
            for i in days {
                if let Err(e) = aoc.bench_day(DayNumber::from(i)) {
                    eprintln!("{}", e);
                }
            }
        }
    }
}
