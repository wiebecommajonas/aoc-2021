use libaoc::{Day, DayNumber};

fn paths_to_end<const VISIT_SMALL_N: usize>(
    start: &str,
    input: &[(&str, &str)],
    visited_small_caves: &[(&str, usize)],
) -> usize {
    if start == "end" {
        return 1;
    }
    let available_paths: Vec<(&str, &str)> = input
        .iter()
        .filter(|(a, b)| {
            (*a == start || *b == start)
                && !visited_small_caves.iter().any(|(cave_name, visited)| {
                    (*cave_name == {
                        if *a == start {
                            *b
                        } else {
                            *a
                        }
                    }) && *visited >= VISIT_SMALL_N
                })
        })
        .copied()
        .collect();
    if available_paths.is_empty() {
        return 0;
    }

    let mut new_visited_small_caves = Vec::new();
    new_visited_small_caves.extend_from_slice(visited_small_caves);
    if start.chars().any(|char| char.is_lowercase()) {
        if start == "start" {
            new_visited_small_caves.push((start, VISIT_SMALL_N));
        } else if !visited_small_caves
            .iter()
            .any(|(cave_name, _)| *cave_name == start)
        {
            new_visited_small_caves.push((start, 1));
        } else if visited_small_caves
            .iter()
            .any(|(cave_name, visited)| *cave_name == start && *visited < VISIT_SMALL_N)
        {
            new_visited_small_caves
                .iter_mut()
                .for_each(|(cave_name, visited)| {
                    if *cave_name == start {
                        *visited += 1;
                    }
                });
            if VISIT_SMALL_N != 1
                && new_visited_small_caves
                    .iter()
                    .filter(|(_, visited)| *visited == VISIT_SMALL_N)
                    .count()
                    > 2
            {
                return 0;
            }
        } else {
            unreachable!()
        }
    }

    available_paths
        .iter()
        .map(|(a, b)| {
            if *a == start {
                paths_to_end::<VISIT_SMALL_N>(b, input, &new_visited_small_caves)
            } else {
                paths_to_end::<VISIT_SMALL_N>(a, input, &new_visited_small_caves)
            }
        })
        .sum::<usize>()
}

pub fn twelve() -> Day<2021> {
    Day::new(
        DayNumber::Twelve,
        |input| {
            let input: Vec<(&str, &str)> = input
                .lines()
                .map(|line| line.split_once('-').unwrap())
                .collect();

            let a = paths_to_end::<1>("start", &input, &[]);

            Box::new(a)
        },
        |input| {
            let input: Vec<(&str, &str)> = input
                .lines()
                .map(|line| line.split_once('-').unwrap())
                .collect();

            let a = paths_to_end::<2>("start", &input, &[]);

            Box::new(a)
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::days::twelve::paths_to_end;

    #[test]
    fn slightly_larger() {
        let input: Vec<(&str, &str)> = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            .lines()
            .map(|line| line.split_once('-').unwrap())
            .collect();

        assert_eq!(paths_to_end::<1>("start", &input, &[]), 19);
        assert_eq!(paths_to_end::<2>("start", &input, &[]), 103);
    }

    #[test]
    fn even_larger() {
        let input: Vec<(&str, &str)> = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
            .lines()
            .map(|line| line.split_once('-').unwrap())
            .collect();

        assert_eq!(paths_to_end::<1>("start", &input, &[]), 226);
        assert_eq!(paths_to_end::<2>("start", &input, &[]), 3509);
    }
}
