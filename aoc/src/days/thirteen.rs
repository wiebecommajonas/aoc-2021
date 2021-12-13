use libaoc::{Day, DayNumber};

pub fn thirteen() -> Day<2021> {
    Day::new(
        DayNumber::Thirteen,
        |input| {
            println!("{}", input);
            Box::new(0)
        },
        |input| Box::new(0),
    )
}
