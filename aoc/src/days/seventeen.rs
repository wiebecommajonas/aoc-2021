use libaoc::{Day, DayNumber};

fn gauss_sum(number: i32) -> i32 {
    (number * (number + 1)) / 2
}

pub fn seventeen() -> Day<2021> {
    Day::new(
        DayNumber::Seventeen,
        |input| {
            let target_y_min = input
                .lines()
                .next()
                .unwrap()
                .split_once(", ")
                .unwrap()
                .1
                .split_once('=')
                .unwrap()
                .1
                .split_once("..")
                .unwrap()
                .0
                .parse::<i32>()
                .unwrap();

            Box::new(gauss_sum(-target_y_min - 1))
        },
        |input| {
            let line = input.lines().next().unwrap();
            let (target_x, target_y) = line.split_once(", ").unwrap();
            let target_x = target_x.split_once('=').unwrap().1;
            let target_y = target_y.split_once('=').unwrap().1;
            let (target_y_min, target_y_max): (i64, i64) = (
                target_y.split_once("..").unwrap().0.parse().unwrap(),
                target_y.split_once("..").unwrap().1.parse().unwrap(),
            );
            let (target_x_min, target_x_max): (i64, i64) = (
                target_x.split_once("..").unwrap().0.parse().unwrap(),
                target_x.split_once("..").unwrap().1.parse().unwrap(),
            );

            let mut valid_y = Vec::new();
            let mut valid_x = Vec::new();

            let mut v_init = (0, target_y_min);
            let pos_init = (0, 0);

            'outer_1: for x in 0..=target_x_max {
                v_init.0 = x;
                let mut v = v_init;
                let mut pos = pos_init;
                while !(pos.0 >= target_x_min && pos.0 <= target_x_max) {
                    pos.0 += v.0;
                    if v.0 > 0 {
                        v.0 -= 1;
                    } else if v.0 < 0 {
                        v.0 += 1;
                    } else if pos.0 >= target_x_min && pos.0 <= target_x_max {
                        valid_x.push(x);
                        continue 'outer_1;
                    } else {
                        continue 'outer_1;
                    }
                    if pos.0 > target_x_max {
                        continue 'outer_1;
                    }
                }
                valid_x.push(x);
            }

            'outer_2: for y in target_y_min..=(-target_y_min - 1) {
                v_init.1 = y;
                let mut v = v_init;
                let mut pos = pos_init;
                while !(pos.1 >= target_y_min && pos.1 <= target_y_max) {
                    pos.1 += v.1;
                    v.1 -= 1;
                    if pos.1 < target_y_min {
                        continue 'outer_2;
                    }
                }
                valid_y.push(y);
            }

            let mut valid = Vec::new();

            for x in &valid_x {
                'outer_3: for y in &valid_y {
                    v_init = (*x, *y);
                    let mut v = v_init;
                    let mut pos = pos_init;
                    while !(pos.1 >= target_y_min
                        && pos.1 <= target_y_max
                        && pos.0 >= target_x_min
                        && pos.0 <= target_x_max)
                    {
                        pos.0 += v.0;
                        pos.1 += v.1;
                        v.1 -= 1;
                        if v.0 > 0 {
                            v.0 -= 1;
                        } else if v.0 < 0 {
                            v.0 += 1;
                        } else if !(pos.0 >= target_x_min && pos.0 <= target_x_max) {
                            continue 'outer_3;
                        }

                        if pos.0 > target_x_max || pos.1 < target_y_min {
                            continue 'outer_3;
                        }
                    }
                    valid.push((*x, *y))
                }
            }

            Box::new(valid.len())
        },
    )
}
