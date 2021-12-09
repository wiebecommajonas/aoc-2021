use libaoc::{Day, DayNumber};

pub fn two() -> Day<2021> {
    Day::new(
        DayNumber::Two,
        |input| {
            let (depth, horizontal) =
                input
                    .lines()
                    .fold((0, 0), |(depth, horizontal), instruction| {
                        if !instruction.is_empty() {
                            let (inst, str_num) = instruction.split_once(' ').unwrap();
                            let number = str_num.parse::<u32>().unwrap();
                            match inst {
                                "forward" => (depth, horizontal + number),
                                "down" => (depth + number, horizontal),
                                "up" => (depth - number, horizontal),
                                _ => unreachable!(),
                            }
                        } else {
                            (depth, horizontal)
                        }
                    });
            Box::new(depth * horizontal)
        },
        |input| {
            let (depth, horizontal, _aim) =
                input
                    .lines()
                    .fold((0, 0, 0), |(depth, horizontal, aim), instruction| {
                        if !instruction.is_empty() {
                            let (inst, str_num) = instruction.split_once(' ').unwrap();
                            let number = str_num.parse::<u32>().unwrap();
                            match inst {
                                "forward" => (depth + aim * number, horizontal + number, aim),
                                "down" => (depth, horizontal, aim + number),
                                "up" => (depth, horizontal, aim - number),
                                _ => unreachable!(),
                            }
                        } else {
                            (depth, horizontal, aim)
                        }
                    });
            Box::new(depth * horizontal)
        },
    )
}
