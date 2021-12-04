use libaoc::{Day, DayNumber};

pub fn three() -> Day<2021> {
    Day::new(
        DayNumber::Three,
        |input| {
            let lines = input.lines().collect::<Vec<&str>>();
            let size = lines.len();
            let bits = lines[0].len();
            let mut gamma = 0;

            for i in 0..bits {
                let ones = lines
                    .iter()
                    .filter(|&a| a.get(i..i + 1).unwrap() == "1")
                    .count();
                gamma = (gamma << 1) | (size <= 2 * ones) as u32;
            }
            println!(
                "{}",
                gamma * (gamma ^ u32::from_str_radix("111111111111", 2).unwrap())
            );
        },
        |input| {
            let lines = input.lines().collect::<Vec<&str>>();
            let bits = lines[0].len();
            let mut oxygen = String::new();
            let mut co2 = String::new();

            let mut oxygen_data = lines.clone();
            for i in 0..bits {
                let ones = oxygen_data
                    .iter()
                    .filter(|&a| a.get(i..i + 1).unwrap() == "1")
                    .count();
                if 2 * ones >= oxygen_data.len() {
                    oxygen_data = oxygen_data
                        .iter()
                        .filter(|&a| a.get(i..i + 1).unwrap() == "1")
                        .copied()
                        .collect::<Vec<&str>>();
                } else {
                    oxygen_data = oxygen_data
                        .iter()
                        .filter(|&a| a.get(i..i + 1).unwrap() == "0")
                        .copied()
                        .collect::<Vec<&str>>();
                }
                if oxygen_data.len() == 1 {
                    oxygen = oxygen_data[0].to_owned();
                    break;
                }
            }

            let mut co2_data = lines.clone();
            for i in 0..bits {
                let ones = co2_data
                    .iter()
                    .filter(|&a| a.get(i..i + 1).unwrap() == "1")
                    .count();
                if co2_data.len() <= 2 * ones {
                    co2_data = co2_data
                        .iter()
                        .filter(|&a| a.get(i..i + 1).unwrap() == "0")
                        .copied()
                        .collect::<Vec<&str>>();
                } else {
                    co2_data = co2_data
                        .iter()
                        .filter(|&a| a.get(i..i + 1).unwrap() == "1")
                        .copied()
                        .collect::<Vec<&str>>();
                }
                if co2_data.len() == 1 {
                    co2 = co2_data[0].to_owned();
                    break;
                }
            }

            println!(
                "{}",
                u32::from_str_radix(&oxygen, 2).unwrap() * u32::from_str_radix(&co2, 2).unwrap()
            );
        },
    )
}
