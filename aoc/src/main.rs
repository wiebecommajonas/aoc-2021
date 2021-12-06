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

    if let Some(("run", ms)) = matches.subcommand() {
        if let Some(values) = ms.values_of("day") {
            for i in values {
                if let Err(e) = aoc.run_day(DayNumber::from(i)) {
                    eprintln!("{}", e);
                }
            }
        }

        if ms.is_present("all") {
            if let Err(e) = aoc.run_days_all() {
                println!("{}", e)
            }
        }
    }
}
