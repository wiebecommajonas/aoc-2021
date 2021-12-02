use libaoc::{Day, DayNumber};

pub fn one() -> Day<2021> {
    Day::new(
        DayNumber::One,
        |input| {
            let split = input.split('\n');
            let numbers = split
                .map(|num| {
                    if num.is_empty() {
                        0
                    } else {
                        num.parse::<u32>().unwrap()
                    }
                })
                .collect::<Vec<u32>>();

            let mut prev = numbers[0];
            let mut counter = 0;
            for &i in numbers.iter() {
                if i > prev {
                    counter += 1;
                }
                prev = i;
            }
            println!("{}", counter);
        },
        |input| {
            let split = input.split('\n');
            let numbers = split
                .map(|num| {
                    if num.is_empty() {
                        0
                    } else {
                        num.parse::<u32>().unwrap()
                    }
                })
                .collect::<Vec<u32>>();

            let mut counter = 0;
            let mut prev = 0;
            for i in 0..(numbers.len() - 2) {
                let sum = numbers[i] + numbers[i + 1] + numbers[i + 2];
                if sum > prev && prev != 0 {
                    counter += 1;
                }
                prev = sum;
            }
            println!("{}", counter);
        },
    )
}
