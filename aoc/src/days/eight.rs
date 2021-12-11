use std::collections::HashMap;

use libaoc::{Day, DayNumber};

pub fn decode(digits: &[&str], output: &[&str]) -> u32 {
    assert_eq!(output.len(), 4);
    assert_eq!(digits.len(), 10);

    let mut byte_number: HashMap<u8, u8> = HashMap::new();
    let mut number_byte: HashMap<u8, u8> = HashMap::new();
    let mut bitmaps: Vec<u8> = digits
        .iter()
        .map(|&digit| {
            let mut byte = 0u8;
            for c in digit.chars() {
                byte |= 1 << (c as u8 - b'a');
            }
            match digit.len() {
                2 => {
                    byte_number.insert(byte, 1);
                    number_byte.insert(1, byte);
                }
                3 => {
                    byte_number.insert(byte, 7);
                    number_byte.insert(7, byte);
                }
                4 => {
                    byte_number.insert(byte, 4);
                    number_byte.insert(4, byte);
                }
                7 => {
                    byte_number.insert(byte, 8);
                    number_byte.insert(8, byte);
                }
                _ => (),
            }
            byte
        })
        .collect();

    bitmaps.drain_filter(|map| number_byte.values().any(|&v| v == *map));

    let six = bitmaps
        .drain_filter(|map| {
            (*map | (number_byte.get(&7).unwrap() ^ number_byte.get(&8).unwrap())) == *map
        })
        .last()
        .unwrap();
    number_byte.insert(6, six);
    byte_number.insert(six, 6);

    let five = bitmaps
        .drain_filter(|map| (*map & number_byte.get(&6).unwrap()) == *map)
        .last()
        .unwrap();
    number_byte.insert(5, five);
    byte_number.insert(five, 5);

    let nine = bitmaps
        .drain_filter(|map| *map == number_byte.get(&5).unwrap() | number_byte.get(&4).unwrap())
        .last()
        .unwrap();
    number_byte.insert(9, nine);
    byte_number.insert(nine, 9);

    let three = bitmaps
        .drain_filter(|map| *map | number_byte.get(&9).unwrap() == *number_byte.get(&9).unwrap())
        .last()
        .unwrap();
    byte_number.insert(three, 3);

    let zero = bitmaps
        .drain_filter(|map| *map & number_byte.get(&1).unwrap() == *number_byte.get(&1).unwrap())
        .last()
        .unwrap();
    byte_number.insert(zero, 0);

    let two = bitmaps[0];
    byte_number.insert(two, 2);

    let mut result = 0u32;
    for (i, &out) in output.iter().enumerate() {
        let mut out_byte = 0u8;
        for c in out.chars() {
            out_byte |= 1 << (c as u8 - b'a');
        }
        let mut pow10 = 1u32;
        for _ in 0..(3 - i) {
            pow10 *= 10;
        }
        result += pow10 * *byte_number.get(&out_byte).unwrap() as u32
    }

    result
}

pub fn eight() -> Day<2021> {
    Day::new(
        DayNumber::Eight,
        |input| {
            Box::new(
                input
                    .lines()
                    .map(|line| {
                        line.split_once(" | ")
                            .unwrap()
                            .1
                            .split_whitespace()
                            .filter(|&d| [2, 3, 4, 7].contains(&d.len()))
                            .count()
                    })
                    .sum::<usize>(),
            )
        },
        |input| {
            let codes: Vec<(Vec<&str>, Vec<&str>)> = input
                .lines()
                .map(|line| line.split_once(" | ").unwrap())
                .map(|(digits, out)| {
                    (
                        digits.split_whitespace().collect::<Vec<&str>>(),
                        out.split_whitespace().collect::<Vec<&str>>(),
                    )
                })
                .collect();

            Box::new(
                codes
                    .iter()
                    .map(|(digits, out)| decode(digits, out))
                    .sum::<u32>(),
            )
        },
    )
}

#[cfg(test)]
mod tests {
    fn parse_line(input: &str) -> (Vec<&str>, Vec<&str>) {
        let (digits_str, out_str) = input.split_once(" | ").unwrap();
        (
            digits_str.split_whitespace().collect(),
            out_str.split_whitespace().collect(),
        )
    }

    #[test]
    pub fn decode_seven_segments() {
        use crate::{days, parse};
        let inputs = parse!(
        parse_line,
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    );
        let results = vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];
        for ((a, b), &r) in inputs.iter().zip(results.iter()) {
            assert_eq!(days::eight::decode(a, b), r);
        }
    }
}
