use libaoc::{Day, DayNumber};

fn pow(base: u32, exp: u32) -> u32 {
    let mut result = 1;
    for i in 0..exp {
        result *= base;
    }
    result
}

fn bin_to_dec(bin: &str) -> u32 {
    let mut result = 0;
    for i in 0..bin.len() {
        if bin.get(i..i + 1).unwrap() == "1" {
            result += pow(2, (bin.len() - 1 - i) as u32)
        }
    }
    result
}

pub fn three() -> Day<2021> {
    Day::new(
        DayNumber::Three,
        |input| {
            let lines = input.lines().collect::<Vec<&str>>();
            let size = lines.len();
            let bits = lines[0].len();
            let mut gamma = String::new();
            let mut epsilon = String::new();

            for i in 0..bits {
                let ones = lines
                    .iter()
                    .filter(|a| a.get(i..i + 1).unwrap() == "1")
                    .count();
                if size > 2 * ones {
                    gamma.push('0');
                    epsilon.push('1');
                } else {
                    gamma.push('1');
                    epsilon.push('0');
                }
            }
            println!("{}", bin_to_dec(&gamma) * bin_to_dec(&epsilon));
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

            println!("{}", bin_to_dec(&oxygen) * bin_to_dec(&co2));
        },
    )
}
