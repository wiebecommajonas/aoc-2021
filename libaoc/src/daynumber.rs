use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DayNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

macro_rules! impl_display_from {
    ( $( ($n:path, $s:expr) ),* $(,)? ) => {
        impl Display for DayNumber {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $n => write!(f, $s)?,
                    )*
                };
                Ok(())
            }
        }

        impl From<&str> for DayNumber {
            fn from(other: &str) -> Self {
                match other {
                    $(
                        $s => $n,
                    )*
                    _ => panic!(""),
                }
            }
        }

        impl From<DayNumber> for u32 {
            fn from(other: DayNumber) -> Self {
                match other {
                    $(
                        $n => $s.parse::<u32>().unwrap(),
                    )*
                }
            }
        }
    };
}

impl_display_from!(
    (DayNumber::One, "1"),
    (DayNumber::Two, "2"),
    (DayNumber::Three, "3"),
    (DayNumber::Four, "4"),
    (DayNumber::Five, "5"),
    (DayNumber::Six, "6"),
    (DayNumber::Seven, "7"),
    (DayNumber::Eight, "8"),
    (DayNumber::Nine, "9"),
    (DayNumber::Ten, "10"),
    (DayNumber::Eleven, "11"),
    (DayNumber::Twelve, "12"),
    (DayNumber::Thirteen, "13"),
    (DayNumber::Fourteen, "14"),
    (DayNumber::Fifteen, "15"),
    (DayNumber::Sixteen, "16"),
    (DayNumber::Seventeen, "17"),
    (DayNumber::Eighteen, "18"),
    (DayNumber::Nineteen, "19"),
    (DayNumber::Twenty, "20"),
    (DayNumber::TwentyOne, "21"),
    (DayNumber::TwentyTwo, "22"),
    (DayNumber::TwentyThree, "23"),
    (DayNumber::TwentyFour, "24"),
    (DayNumber::TwentyFive, "25"),
);
