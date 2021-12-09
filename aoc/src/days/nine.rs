use std::collections::VecDeque;

use colored::Colorize;
use libaoc::{Day, DayNumber};

fn find_adjacent(
    all_basin_points: &mut VecDeque<(usize, usize)>,
    basin_points: &Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    for i in 0..all_basin_points.len() {
        for &point in basin_points {
            let all_point = all_basin_points[i];
            if point.0 + 1 == all_point.0 && point.1 == all_point.1
                || point.0 - 1 == all_point.0 && point.1 == all_point.1
                || point.1 + 1 == all_point.1 && point.0 == all_point.0
                || point.1 - 1 == all_point.1 && point.0 == all_point.0
            {
                return Some(all_basin_points.remove(i).unwrap());
            }
        }
    }
    None
}

pub fn nine() -> Day<2021> {
    Day::new(
        DayNumber::Nine,
        |input| {
            let lines: Vec<&str> = input.lines().collect();
            let height = lines.len();
            let width = lines[0].len();

            let heightmap: Vec<u8> = lines
                .iter()
                .map(|&line| line.bytes().map(|byte| byte - b'0').collect::<Vec<u8>>())
                .flatten()
                .collect();

            let mut sum_of_risk_levels = 0_usize;
            for i in 0..height {
                for j in 0..width {
                    let current_height = heightmap[i * width + j];
                    let left = if j == 0 {
                        u8::MAX
                    } else {
                        heightmap[i * width + j - 1]
                    };
                    let right = if j == width - 1 {
                        u8::MAX
                    } else {
                        heightmap[i * width + j + 1]
                    };
                    let above = if i == 0 {
                        u8::MAX
                    } else {
                        heightmap[(i - 1) * width + j]
                    };
                    let below = if i == height - 1 {
                        u8::MAX
                    } else {
                        heightmap[(i + 1) * width + j]
                    };

                    if current_height < left
                        && current_height < right
                        && current_height < above
                        && current_height < below
                    {
                        // print!("{}", format!("{}", current_height).red());
                        sum_of_risk_levels += current_height as usize + 1;
                    } else {
                        // print!("{}", current_height)
                    }
                }
                // println!();
            }

            Box::new(sum_of_risk_levels)
        },
        |input| {
            let lines: Vec<&str> = input.lines().collect();
            let height = lines.len();
            let width = lines[0].len();

            let heightmap: Vec<u8> = lines
                .iter()
                .map(|&line| line.bytes().map(|byte| byte - b'0').collect::<Vec<u8>>())
                .flatten()
                .collect();

            let mut basin_points: VecDeque<(usize, usize)> = VecDeque::new();
            let mut basins: Vec<Vec<(usize, usize)>> = Vec::new();

            for i in 0..height {
                for j in 0..width {
                    if heightmap[i * width + j] != 9 {
                        basin_points.push_back((i, j));
                        // print!("{}", format!("{}", heightmap[i * width + j]).red());
                    } else {
                        // print!("{}", heightmap[i * width + j]);
                    }
                }
                // println!();
            }

            while !basin_points.is_empty() {
                let origin = basin_points.pop_front().unwrap();
                let mut basin = vec![origin];
                while let Some(adjacent) = find_adjacent(&mut basin_points, &basin) {
                    basin.push(adjacent)
                }
                basins.push(basin);
            }

            let mut basin_sizes: Vec<usize> = basins.iter().map(|basin| basin.len()).collect();
            let mut max_basin_sizes: Vec<usize> = vec![0; 3];

            for i in 0..3 {
                let mut max_index = 0;
                for j in 0..basin_sizes.len() {
                    if max_basin_sizes[i] < basin_sizes[j] {
                        max_basin_sizes[i] = basin_sizes[j];
                        max_index = j;
                    }
                }
                basin_sizes.remove(max_index);
            }

            Box::new(max_basin_sizes.iter().product::<usize>())
        },
    )
}
