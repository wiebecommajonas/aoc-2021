use libaoc::{Day, DayNumber};

fn solve<F>(input: &str, calc_fuel: F) -> Box<usize>
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
    let mut _alignment_value = 0;

    for i in 0..*numbers.iter().max().unwrap() {
        let new = numbers.iter().map(|&number| calc_fuel(number, i)).sum();
        if new < least_fuel {
            least_fuel = new;
            _alignment_value = i;
        }
    }

    Box::new(least_fuel)
}

pub fn sub_abs(a: usize, b: usize) -> usize {
    let a = a as isize;
    let b = b as isize;
    (a - b).abs() as usize
}

pub fn seven() -> Day<2021> {
    Day::new(
        DayNumber::Seven,
        |input| solve(input, sub_abs),
        |input| {
            solve(input, |number, alignment| {
                sub_abs(number, alignment) * (sub_abs(number, alignment) + 1) / 2
            })
        },
    )
}
