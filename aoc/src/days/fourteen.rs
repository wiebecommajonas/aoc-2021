use std::collections::HashMap;

use itertools::Itertools;
use libaoc::{Day, DayNumber};

fn solve<const INSERTION_CYCLES: usize>(input: &str) -> usize {
    let (template_line, insert_lines) = input.split_once("\n\n").unwrap();
    let mut template: Vec<char> = template_line.chars().collect();

    let mut insertions = HashMap::<(char, char), char>::new();
    insert_lines.lines().for_each(|line| {
        let (pair_str, insert_str) = line.split_once(" -> ").unwrap();
        let pair: Vec<char> = pair_str.chars().collect();
        let insert: char = insert_str.chars().next().unwrap();
        insertions.insert((pair[0], pair[1]), insert);
    });

    let mut prev_template = template.clone();
    let mut insertion_count = 0;
    for _ in 0..INSERTION_CYCLES {
        prev_template
            .iter()
            .tuple_windows::<(_, _)>()
            .enumerate()
            .for_each(|(idx, (char1, char2))| {
                if let Some(char) = insertions.get(&(*char1, *char2)) {
                    template.insert(idx + 1 + insertion_count, *char);
                    insertion_count += 1;
                }
            });
        prev_template = template.clone();
        insertion_count = 0;
    }

    let mut char_counts = HashMap::<char, usize>::new();

    template.iter().for_each(|char| {
        if let Some(count) = char_counts.get_mut(char) {
            *count += 1;
        } else {
            char_counts.insert(*char, 1);
        }
    });

    let mut values: Vec<&usize> = char_counts.values().collect();
    values.sort_unstable();

    values[values.len() - 1] - values[0]
}

pub fn fourteen() -> Day<2021> {
    Day::new(
        DayNumber::Fourteen,
        |input| Box::new(solve::<10>(input)),
        |input| Box::new("nope"),
    )
}

#[cfg(test)]
mod tests {
    use crate::days::fourteen::solve;

    #[test]
    fn example() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(solve::<10>(input), 1588);
        // assert_eq!(solve::<40>(input), 2188189693529);
    }
}
