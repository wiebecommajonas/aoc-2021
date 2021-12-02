use libaoc::{Day, DayNumber};

pub fn one() -> Day<2021> {
    Day::new(
        DayNumber::One,
        |input| {
            let result = input
                .lines()
                .map(|num| {
                    if num.is_empty() {
                        0
                    } else {
                        num.parse::<u32>().unwrap()
                    }
                })
                .collect::<Vec<u32>>()
                .windows(2)
                .filter(|w| w[0] < w[1])
                .count();
            println!("{}", result);
        },
        |input| {
            let result = input
                .lines()
                .map(|num| {
                    if num.is_empty() {
                        0
                    } else {
                        num.parse::<u32>().unwrap()
                    }
                })
                .collect::<Vec<u32>>()
                .windows(4)
                .filter(|w| w[0] < w[3])
                .count();
            println!("{}", result);
        },
    )
}
