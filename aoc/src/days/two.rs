use libaoc::{Day, DayNumber};

pub fn two() -> Day<2021> {
    Day::new(
        DayNumber::Two,
        |input| {
            println!("{}", input);
        },
        |input| {},
    )
}
