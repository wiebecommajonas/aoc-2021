use libaoc::{Day, DayNumber};

fn solve<const TICKS: usize>(input: &str) {
    let fish: Vec<usize> = input
        .split_once('\n')
        .unwrap()
        .0
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let mut counts = [0; 9];
    for &i in &fish {
        counts[i] += 1;
    }

    for _ in 0..TICKS {
        let mut new_counts = [0; 9];
        new_counts[..9 - 1].copy_from_slice(&counts[1..9]);
        new_counts[6] += &counts[0];
        new_counts[8] += &counts[0];

        counts = new_counts;
    }

    println!("{}", counts.iter().sum::<usize>());
}

pub fn six() -> Day<2021> {
    Day::new(
        DayNumber::Six,
        |input| {
            solve::<80>(input);
        },
        |input| {
            solve::<256>(input);
        },
    )
}
