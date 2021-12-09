use std::collections::HashMap;

use libaoc::{Day, DayNumber};

pub fn five() -> Day<2021> {
    Day::new(
        DayNumber::Five,
        |input| {
            let mut points = HashMap::<(usize, usize), usize>::new();
            let lines = input.lines();

            lines.for_each(|line| {
                let points_str = line.split_once(" -> ").unwrap();
                let point_str_a = points_str.0.split_once(',').unwrap();
                let point_str_b = points_str.1.split_once(',').unwrap();
                if point_str_a.0 == point_str_b.0 || point_str_a.1 == point_str_b.1 {
                    let point_a = (
                        point_str_a.0.parse::<usize>().unwrap(),
                        point_str_a.1.parse::<usize>().unwrap(),
                    );
                    let point_b = (
                        point_str_b.0.parse::<usize>().unwrap(),
                        point_str_b.1.parse::<usize>().unwrap(),
                    );
                    let end: usize;
                    let start: usize;
                    if point_a.0 == point_b.0 {
                        if point_a.1 > point_b.1 {
                            end = point_a.1;
                            start = point_b.1;
                        } else {
                            end = point_b.1;
                            start = point_a.1;
                        };
                        for i in start..=end {
                            let point = (point_a.0, i);
                            if let Some(entry) = points.get_mut(&point) {
                                *entry += 1;
                            } else {
                                points.insert(point, 1);
                            }
                        }
                    } else if point_a.1 == point_b.1 {
                        if point_a.0 > point_b.0 {
                            end = point_a.0;
                            start = point_b.0;
                        } else {
                            end = point_b.0;
                            start = point_a.0;
                        };
                        for i in start..=end {
                            let point = (i, point_a.1);
                            if let Some(entry) = points.get_mut(&point) {
                                *entry += 1;
                            } else {
                                points.insert(point, 1);
                            }
                        }
                    }
                }
            });
            let at_least_2_overlap = points
                .values()
                .filter(|&&intersecting| intersecting >= 2)
                .count();

            Box::new(at_least_2_overlap)
        },
        |input| {
            let mut points = HashMap::<(usize, usize), usize>::new();
            let lines = input.lines();

            lines.for_each(|line| {
                let points_str = line.split_once(" -> ").unwrap();
                let point_str_a = points_str.0.split_once(',').unwrap();
                let point_str_b = points_str.1.split_once(',').unwrap();
                let point_a = (
                    point_str_a.0.parse::<usize>().unwrap(),
                    point_str_a.1.parse::<usize>().unwrap(),
                );
                let point_b = (
                    point_str_b.0.parse::<usize>().unwrap(),
                    point_str_b.1.parse::<usize>().unwrap(),
                );
                let end: usize;
                let start: usize;
                if point_a.0 == point_b.0 {
                    if point_a.1 > point_b.1 {
                        end = point_a.1;
                        start = point_b.1;
                    } else {
                        end = point_b.1;
                        start = point_a.1;
                    };
                    for i in start..=end {
                        let point = (point_a.0, i);
                        if let Some(entry) = points.get_mut(&point) {
                            *entry += 1;
                        } else {
                            points.insert(point, 1);
                        }
                    }
                } else if point_a.1 == point_b.1 {
                    if point_a.0 > point_b.0 {
                        end = point_a.0;
                        start = point_b.0;
                    } else {
                        end = point_b.0;
                        start = point_a.0;
                    };
                    for i in start..=end {
                        let point = (i, point_a.1);
                        if let Some(entry) = points.get_mut(&point) {
                            *entry += 1;
                        } else {
                            points.insert(point, 1);
                        }
                    }
                } else {
                    let mut y: usize;
                    let check = (point_a.0 > point_b.0 && point_a.1 < point_b.1)
                        || (point_a.0 < point_b.0 && point_a.1 > point_b.1);
                    let xs = if point_a.0 > point_b.0 {
                        y = point_b.1;
                        point_b.0..=point_a.0
                    } else {
                        y = point_a.1;
                        point_a.0..=point_b.0
                    };

                    for x in xs {
                        let point = (x, y);
                        if let Some(entry) = points.get_mut(&point) {
                            *entry += 1;
                        } else {
                            points.insert(point, 1);
                        }
                        if check {
                            y -= 1;
                        } else {
                            y += 1;
                        }
                    }
                }
            });

            let at_least_2_overlap = points
                .values()
                .filter(|&&intersecting| intersecting >= 2)
                .count();

            Box::new(at_least_2_overlap)
        },
    )
}
