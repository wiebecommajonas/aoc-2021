use std::collections::HashMap;

use libaoc::{Day, DayNumber};

fn decode(digits: &[&str], output: &[&str]) -> u32 {
    assert_eq!(output.len(), 4);
    assert_eq!(digits.len(), 10);
    let mut mapping: HashMap<u32, &str> = HashMap::new();

    for &i in digits {
        match i.len() {
            2 => {
                mapping.insert(1, i);
            }
            3 => {
                mapping.insert(7, i);
            }
            4 => {
                mapping.insert(4, i);
            }
            7 => {
                mapping.insert(8, i);
            }
            _ => (),
        }
    }

    /* 6 has six segments (also: 9 and 0)
     * 6 does not contain exactly one of the segments of 7 (unique)
     */
    let six = digits
        .iter()
        .filter(|&&digit| digit.len() == 6)
        .filter(|&&digit| {
            for i in mapping.get(&7).unwrap().chars() {
                if !digit.contains(i) {
                    return true;
                }
            }
            false
        })
        .last()
        .unwrap();
    mapping.insert(6, six);

    /* 5 hast 5 segments (also: 2 and 3)
     * 5 has exactly one segment missing from 6 (unique)
     */
    let five = digits
        .iter()
        .filter(|&&digit| digit.len() == 5)
        .filter(|&&digit| {
            let mut sum = 0;
            for i in mapping.get(&6).unwrap().chars() {
                if !digit.contains(i) {
                    sum += 1;
                    if sum > 1 {
                        return false;
                    }
                }
            }
            true
        })
        .last()
        .unwrap();
    mapping.insert(5, five);

    /* 9 has six segments (also: 6 and 0)
     * 9 is not the same sequence as 6 (also: 0)
     * 9 has all segments of 5 (unique)
     */
    let nine = digits
        .iter()
        .filter(|&digit| digit.len() == 6 && digit != mapping.get(&6).unwrap())
        .filter(|&&digit| {
            for i in mapping.get(&5).unwrap().chars() {
                if !digit.contains(i) {
                    return false;
                }
            }
            true
        })
        .last()
        .unwrap();
    mapping.insert(9, nine);

    /* 0 has six segments (also: 6 and 9)
     * 0 is not the same sequence as 9 (also: 6)
     * 0 is not the same sequence as 6 (unique)
     */
    let zero = digits
        .iter()
        .filter(|&digit| {
            digit.len() == 6
                && digit != mapping.get(&9).unwrap()
                && digit != mapping.get(&6).unwrap()
        })
        .last()
        .unwrap();
    mapping.insert(0, zero);

    /* 3 has five segments (also: 5 and 2)
     * 3 is not the same sequence as 5 (also: 2)
     * 3 has all segments of 1 (unique)
     */
    let three = digits
        .iter()
        .filter(|&digit| digit.len() == 5 && digit != mapping.get(&5).unwrap())
        .filter(|&&digit| {
            for i in mapping.get(&1).unwrap().chars() {
                if !digit.contains(i) {
                    return false;
                }
            }
            true
        })
        .last()
        .unwrap();
    mapping.insert(3, three);

    /* 2 is the only sequence which is not mapped yet (unique)
     */
    let two = digits
        .iter()
        .filter(|&&digit| !mapping.values().any(|&x| x == digit))
        .last()
        .unwrap();
    mapping.insert(2, two);

    let mut result = 0;
    for (i, out) in output.iter().enumerate() {
        'outer: for key in mapping.keys() {
            let value = mapping.get(key).unwrap();

            // check if `out` and `value` are the same set of chars
            // if not: continue
            if value.len() != out.len() {
                continue 'outer;
            }
            for c in value.chars() {
                if !out.contains(c) {
                    continue 'outer;
                }
            }

            // add the mapped digit `key` to `result`
            // at the proper place (`* pow10`)
            result += {
                let mut pow10 = 1;
                for _ in 0..(3 - i) {
                    pow10 *= 10;
                }
                pow10
            } * key;
            break;
        }
    }
    result
}

pub fn eight() -> Day<2021> {
    Day::new(
        DayNumber::Eight,
        |input| {
            input
                .lines()
                .map(|line| line.split_once(" | ").unwrap().1)
                .map(|display| display.split_whitespace().collect::<Vec<&str>>())
                .flatten()
                .filter(|&d| d.len() == 2 || d.len() == 3 || d.len() == 4 || d.len() == 7)
                .count()
                .to_string()
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

            let sum = codes
                .iter()
                .map(|(digits, out)| decode(digits, out))
                .sum::<u32>();
            sum.to_string()
        },
    )
}
