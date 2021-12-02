use libaoc::{Day, DayNumber};

pub fn two() -> Day<2021> {
    Day::new(
        DayNumber::Two,
        |input| {
            let (depth, horizontal) =
                input
                    .split('\n')
                    .fold((0, 0), |(depth, horizontal), instruction| {
                        if !instruction.is_empty() {
                            let split = instruction.split(' ').collect::<Vec<&str>>();
                            let number = split[1].parse::<u32>().unwrap();
                            match split[0] {
                                "forward" => (depth, horizontal + number),
                                "down" => (depth + number, horizontal),
                                "up" => (depth - number, horizontal),
                                _ => unreachable!(),
                            }
                        } else {
                            (depth, horizontal)
                        }
                    });
            println!("{}", depth * horizontal);
        },
        |input| {
            let (depth, horizontal, _aim) =
                input
                    .split('\n')
                    .fold((0, 0, 0), |(depth, horizontal, aim), instruction| {
                        if !instruction.is_empty() {
                            let split = instruction.split(' ').collect::<Vec<&str>>();
                            let number = split[1].parse::<u32>().unwrap();
                            match split[0] {
                                "forward" => (depth + aim * number, horizontal + number, aim),
                                "down" => (depth, horizontal, aim + number),
                                "up" => (depth, horizontal, aim - number),
                                _ => unreachable!(),
                            }
                        } else {
                            (depth, horizontal, aim)
                        }
                    });
            println!("{}", depth * horizontal);
        },
    )
}
