use libaoc::{Day, DayNumber};

pub fn three() -> Day<2021> {
    Day::new(
        DayNumber::Three,
        |input| {
            let lines = input
                .lines()
                .map(|line| u16::from_str_radix(line, 2).unwrap())
                .collect::<Vec<u16>>();
            let size = lines.len();
            let bits = 13;
            let mut gamma = 0;

            for i in (0..bits).rev() {
                let ones = lines.iter().filter(|&a| ((*a >> i) & 1) != 0).count();
                gamma = (gamma << 1) | (size <= 2 * ones) as u16;
            }
            println!(
                "{}",
                gamma * (gamma ^ u16::from_str_radix("111111111111", 2).unwrap())
            );
        },
        |input| {
            let lines = input
                .lines()
                .map(|line| u16::from_str_radix(line, 2).unwrap())
                .collect::<Vec<u16>>();
            let bits = 12;

            let mut oxygen_data = lines.clone();
            for i in (0..bits).rev() {
                let ones = oxygen_data.iter().filter(|&a| ((*a >> i) & 1) != 0).count();
                if 2 * ones >= oxygen_data.len() {
                    oxygen_data = oxygen_data
                        .iter()
                        .filter(|&a| ((*a >> i) & 1) != 0)
                        .copied()
                        .collect::<Vec<u16>>();
                } else {
                    oxygen_data = oxygen_data
                        .iter()
                        .filter(|&a| ((*a >> i) & 1) == 0)
                        .copied()
                        .collect::<Vec<u16>>();
                }
                if oxygen_data.len() == 1 {
                    break;
                }
            }

            let mut co2_data = lines;
            for i in (0..bits).rev() {
                let ones = co2_data.iter().filter(|&a| ((*a >> i) & 1) != 0).count();
                if co2_data.len() <= 2 * ones {
                    co2_data = co2_data
                        .iter()
                        .filter(|&a| ((*a >> i) & 1) == 0)
                        .copied()
                        .collect::<Vec<u16>>();
                } else {
                    co2_data = co2_data
                        .iter()
                        .filter(|&a| ((*a >> i) & 1) != 0)
                        .copied()
                        .collect::<Vec<u16>>();
                }
                if co2_data.len() == 1 {
                    break;
                }
            }

            println!("{}", oxygen_data[0] * co2_data[0]);
        },
    )
}
