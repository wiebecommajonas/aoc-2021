use libaoc::{Day, DayNumber};

fn solve<F>(input: &str, calc_fuel: F)
where
    F: Fn(usize, usize) -> usize,
{
    let numbers: Vec<usize> = input
        .split_once('\n')
        .unwrap()
        .0
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();
    let mut least_fuel = usize::MAX;
    let mut alignment_value = 0;

    for i in 0..*numbers.iter().max().unwrap() {
        let new = numbers.iter().map(|&number| calc_fuel(number, i)).sum();
        if new < least_fuel {
            least_fuel = new;
            alignment_value = i;
        }
    }

    println!("{}: {}", alignment_value, least_fuel);
}

pub fn seven() -> Day<2021> {
    Day::new(
        DayNumber::Seven,
        |input| {
            solve(input, |number, alignment| {
                if number >= alignment {
                    number - alignment
                } else {
                    alignment - number
                }
            })
        },
        |input| {
            solve(input, |number, alignment| {
                if number >= alignment {
                    (number - alignment) * (number - alignment + 1) / 2
                } else {
                    (alignment - number) * (alignment - number + 1) / 2
                }
            })
        },
    )
}
