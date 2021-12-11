use libaoc::{Day, DayNumber};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn mutate(octopuses: &mut Vec<u8>) -> usize {
    // bool indicates if octopus has flashed
    let mut mapped_octs: Vec<(u8, bool)> = octopuses
        .iter()
        .map(|&energy| (energy + 1, false))
        .collect();
    let mut flashed = 0;

    loop {
        // filter tens that havent flashed yet with their coordinates
        // if none, break
        // else increase all neighbors of these tens

        let mut tens_not_flashed = Vec::new();
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let (oct_energy, oct_flashed) = mapped_octs.get_mut(i * WIDTH + j).unwrap();
                if *oct_energy >= 10 && !*oct_flashed {
                    tens_not_flashed.push((i, j));
                    *oct_flashed = true;
                }
            }
        }

        if tens_not_flashed.is_empty() {
            break;
        }

        flashed += tens_not_flashed.len();

        for (i, j) in tens_not_flashed {
            if j < WIDTH - 1 {
                if let Some((energy, _)) = mapped_octs.get_mut(i * WIDTH + j + 1) {
                    if *energy <= 10 {
                        *energy += 1;
                    }
                }
            }
            if j > 0 {
                if let Some((energy, _)) = mapped_octs.get_mut(i * WIDTH + j - 1) {
                    if *energy <= 10 {
                        *energy += 1;
                    }
                }
            }
            if i < HEIGHT {
                if let Some((energy, _)) = mapped_octs.get_mut((i + 1) * WIDTH + j) {
                    if *energy <= 10 {
                        *energy += 1;
                    }
                }
            }
            if i > 0 {
                if let Some((energy, _)) = mapped_octs.get_mut((i - 1) * WIDTH + j) {
                    if *energy <= 10 {
                        *energy += 1;
                    }
                }
            }
            if i < HEIGHT && j > 0 {
                if let Some((energy, _)) = mapped_octs.get_mut((i + 1) * WIDTH + j - 1) {
                    if *energy <= 10 {
                        *energy += 1;
                    }
                }
            }
            if i > 0 && j > 0 {
                if let Some((energy, _)) = mapped_octs.get_mut((i - 1) * WIDTH + j - 1) {
                    if *energy <= 10 {
                        *energy += 1;
                    }
                }
            }
            if i < HEIGHT && j < WIDTH - 1 {
                if let Some((energy, _)) = mapped_octs.get_mut((i + 1) * WIDTH + j + 1) {
                    if *energy <= 10 {
                        *energy += 1;
                    }
                }
            }
            if i > 0 && j < WIDTH - 1 {
                if let Some((energy, _)) = mapped_octs.get_mut((i - 1) * WIDTH + j + 1) {
                    if *energy <= 10 {
                        *energy += 1;
                    }
                }
            }
        }
    }

    *octopuses = mapped_octs
        .iter()
        .map(|(energy, _)| if *energy >= 10 { 0 } else { *energy })
        .collect();

    flashed
}

pub fn eleven() -> Day<2021> {
    Day::new(
        DayNumber::Eleven,
        |input| {
            let mut octopuses: Vec<u8> = input
                .lines()
                .flat_map(|line| line.bytes().map(|byte| byte - b'0').collect::<Vec<u8>>())
                .collect();

            let mut flashes = 0;
            for _ in 0..100 {
                flashes += mutate(&mut octopuses);
            }

            Box::new(flashes)
        },
        |input| {
            let mut octopuses: Vec<u8> = input
                .lines()
                .flat_map(|line| line.bytes().map(|byte| byte - b'0').collect::<Vec<u8>>())
                .collect();

            let mut step = 1;
            loop {
                if mutate(&mut octopuses) == 100 {
                    break;
                }
                step += 1;
            }

            Box::new(step)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::mutate;

    #[test]
    fn mutate_octopuses() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut octopuses: Vec<u8> = input
            .lines()
            .flat_map(|line| line.bytes().map(|byte| byte - b'0').collect::<Vec<u8>>())
            .collect();

        let mut flashes = 0;
        for _ in 0..10 {
            flashes += mutate(&mut octopuses);
        }

        assert_eq!(flashes, 204);
    }
}
