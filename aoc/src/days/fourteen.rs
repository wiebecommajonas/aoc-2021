use itertools::Itertools;
use libaoc::{Day, DayNumber};
use std::collections::HashMap;

fn solve<const INSERTION_CYCLES: usize>(input: &str) -> usize {
    let (template_line, insert_lines) = input.split_once("\n\n").unwrap();
    let mut template: HashMap<(char, char), usize> = HashMap::new();
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    template_line
        .chars()
        .tuple_windows::<(_, _)>()
        .for_each(|pair| {
            if let Some(count) = template.get_mut(&pair) {
                *count += 1;
            } else {
                template.insert(pair, 1);
            }
        });
    template_line.chars().for_each(|ch| {
        if let Some(count) = char_counts.get_mut(&ch) {
            *count += 1;
        } else {
            char_counts.insert(ch, 1);
        }
    });

    let mut insertions = HashMap::<(char, char), char>::new();
    insert_lines.lines().for_each(|line| {
        let (pair_str, insert_str) = line.split_once(" -> ").unwrap();
        let pair: Vec<char> = pair_str.chars().collect();
        let insert: char = insert_str.chars().next().unwrap();
        insertions.insert((pair[0], pair[1]), insert);
    });

    for _ in 0..INSERTION_CYCLES {
        let prev_template = template.clone();
        let mut new_template = HashMap::new();
        for (chars, a) in &prev_template {
            if let Some(insert) = insertions.get(chars) {
                if let Some(count) = new_template.get_mut(&(chars.0, *insert)) {
                    *count += *a;
                } else {
                    new_template.insert((chars.0, *insert), *a);
                }

                if let Some(count) = new_template.get_mut(&(*insert, chars.1)) {
                    *count += *a;
                } else {
                    new_template.insert((*insert, chars.1), *a);
                }

                if let Some(count) = char_counts.get_mut(insert) {
                    *count += *a;
                } else {
                    char_counts.insert(*insert, *a);
                }
            }
        }
        template = new_template;
    }

    let mut values: Vec<&usize> = char_counts.values().collect();
    values.sort_unstable();

    values[values.len() - 1] - values[0]
}

pub fn fourteen() -> Day<2021> {
    Day::new(
        DayNumber::Fourteen,
        |input| Box::new(solve::<10>(input)),
        |input| Box::new(solve::<40>(input)),
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
        assert_eq!(solve::<40>(input), 2188189693529);
    }
}
