use std::collections::HashSet;

use libaoc::{Day, DayNumber};

enum Axis {
    X,
    Y,
}

pub fn thirteen() -> Day<2021> {
    Day::new(
        DayNumber::Thirteen,
        |input| {
            let (points_strs, fold_instructions) = input.split_once("\n\n").unwrap();

            let mut points: HashSet<(usize, usize)> = points_strs
                .lines()
                .map(|line| {
                    let (x, y) = line.split_once(',').unwrap();
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                })
                .collect();

            let fold: (&str, &str) = fold_instructions
                .split_once('\n')
                .unwrap()
                .0
                .split_whitespace()
                .last()
                .unwrap()
                .split_once('=')
                .unwrap();
            let parsed_axis = match fold.0 {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => unreachable!(),
            };
            let value = fold.1.parse::<usize>().unwrap();

            points = points
                .iter()
                .map(|(x, y)| match parsed_axis {
                    Axis::X => {
                        if *x < value {
                            (*x, *y)
                        } else {
                            (*x - 2 * (*x - value), *y)
                        }
                    }
                    Axis::Y => {
                        if *y < value {
                            (*x, *y)
                        } else {
                            (*x, *y - 2 * (*y - value))
                        }
                    }
                })
                .collect();

            Box::new(points.len())
        },
        |input| {
            let (points_strs, fold_instructions) = input.split_once("\n\n").unwrap();

            let mut points: HashSet<(usize, usize)> = points_strs
                .lines()
                .map(|line| {
                    let (x, y) = line.split_once(',').unwrap();
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                })
                .collect();

            let folds: Vec<(Axis, usize)> = fold_instructions
                .lines()
                .map(|line| {
                    let (axis, value) = line
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .split_once('=')
                        .unwrap();
                    let parsed_axis = match axis {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        _ => unreachable!(),
                    };
                    (parsed_axis, value.parse::<usize>().unwrap())
                })
                .collect();

            for (fold_axis, fold_value) in folds {
                points = points
                    .iter()
                    .map(|(x, y)| match fold_axis {
                        Axis::X => {
                            if *x < fold_value {
                                (*x, *y)
                            } else {
                                (*x - 2 * (*x - fold_value), *y)
                            }
                        }
                        Axis::Y => {
                            if *y < fold_value {
                                (*x, *y)
                            } else {
                                (*x, *y - 2 * (*y - fold_value))
                            }
                        }
                    })
                    .collect();
            }

            let mut result = String::new();
            for i in 0..6 {
                for j in 0..39 {
                    if points.contains(&(j, i)) {
                        result.push('#');
                    } else {
                        result.push(' ');
                    }
                }
                if i != 5 {
                    result.push('\n');
                }
            }

            Box::new(result)
        },
    )
}
