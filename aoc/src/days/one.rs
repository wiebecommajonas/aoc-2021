use libaoc::{Day, DayNumber};

fn solve<const WINDOWS: usize>(input: &str) {
    let result = input
        .lines()
        .map(|num| {
            if num.is_empty() {
                0
            } else {
                num.parse::<u16>().unwrap()
            }
        })
        .collect::<Vec<u16>>()
        .windows(WINDOWS)
        .filter(|w| w[0] < w[WINDOWS - 1])
        .count();
    println!("{}", result);
}

pub fn one() -> Day<2021> {
    Day::new(
        DayNumber::One,
        |input| {
            solve::<2>(input);
        },
        |input| {
            solve::<4>(input);
        },
    )
}
