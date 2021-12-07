use libaoc::{Day, DayNumber};

pub fn eight() -> Day<2021> {
    Day::new(
        DayNumber::Eight,
        |input| input.to_owned(),
        |input| "".to_owned(),
    )
}
