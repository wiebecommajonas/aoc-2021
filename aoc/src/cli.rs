use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "Run days")]
    Run {
        #[clap(
            short,
            long,
            value_name = "DAYNUMBER",
            conflicts_with = "all",
            required_unless_present = "all",
            validator_regex(
                regex::Regex::new("^([1-9]|1[0-9]|2[0-5])$").unwrap(),
                "Only days from 1-25 are valid days"
            ),
            help = "Runs day"
        )]
        day: Vec<u32>,

        #[clap(short, help = "Runs all available days in order")]
        all: bool,
    },
    #[clap(about = "Run benchmarks")]
    Bench {
        #[clap(
            short,
            long,
            value_name = "DAYNUMBER",
            required = true,
            validator_regex(
                regex::Regex::new("^([1-9]|1[0-9]|2[0-5])$").unwrap(),
                "Only days from 1-25 are valid days"
            ),
            help = "Runs benchmarks of a day"
        )]
        day: Vec<u32>,
    },
}
