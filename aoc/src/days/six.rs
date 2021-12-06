use libaoc::{Day, DayNumber};

fn adds_fish(time: usize, remaining_time: usize) -> usize {
    // println!("remaining: {}, time: {}", remaining_time, time);
    if remaining_time <= time {
        0
    } else {
        1 + adds_fish(6, remaining_time - time - 1) + adds_fish(8, remaining_time - time - 1)
    }
}

pub fn six() -> Day<2021> {
    Day::new(
        DayNumber::Six,
        |input| {
            let fish: Vec<usize> = input
                .split_once('\n')
                .unwrap()
                .0
                .split(',')
                .map(|number| number.parse().unwrap())
                .collect();

            let mut fish_count = fish.len();
            fish.iter().for_each(|&time| {
                let added_fish = adds_fish(time, 80);
                fish_count += added_fish;
            });

            println!("{}", fish_count);
        },
        |input| {},
    )
}
