use std::collections::VecDeque;

use libaoc::{Day, DayNumber};

pub fn ten() -> Day<2021> {
    Day::new(
        DayNumber::Ten,
        |input| {
            let lines = input.lines();
            let mut syntax_error_score = 0usize;

            lines.for_each(|line| {
                let mut opened_chunks = VecDeque::new();
                line.chars().for_each(|brace| match brace {
                    open @ ('(' | '{' | '[' | '<') => {
                        opened_chunks.push_front(open as u8);
                    }
                    close @ (')' | '}' | ']' | '>') => {
                        let top = opened_chunks.pop_front().unwrap();
                        // close == top + 2 is for `<>`,`[]`,`{}`; close == top + 1 for `()`
                        if !(close as u8 == top + 2 || close as u8 == top + 1) {
                            syntax_error_score += match close {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => unreachable!(),
                            };
                        }
                    }
                    _ => unreachable!(),
                });
            });

            Box::new(syntax_error_score)
        },
        |input| {
            let lines = input.lines();

            let mut autocomplete_scores: Vec<usize> = lines
                .filter(|line| {
                    let mut opened_chunks = VecDeque::new();
                    for brace in line.chars() {
                        match brace {
                            open @ ('(' | '{' | '[' | '<') => {
                                opened_chunks.push_front(open as u8);
                            }
                            close @ (')' | '}' | ']' | '>') => {
                                let top = opened_chunks.pop_front().unwrap();
                                // close == top + 2 is for `<>`,`[]`,`{}`; close == top + 1 for `()`
                                if !(close as u8 == top + 2 || close as u8 == top + 1) {
                                    return false;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    true
                })
                .map(|line| {
                    let mut opened_chunks = VecDeque::new();
                    line.chars().for_each(|char| match char {
                        open @ ('(' | '{' | '[' | '<') => opened_chunks.push_front(open),
                        ')' | '}' | ']' | '>' => {
                            opened_chunks.pop_front();
                        }
                        _ => unreachable!(),
                    });
                    opened_chunks
                        .iter()
                        .fold(0usize, |autocomplete_score, next_char| {
                            autocomplete_score * 5
                                + match next_char {
                                    '(' => 1,
                                    '[' => 2,
                                    '{' => 3,
                                    '<' => 4,
                                    _ => unreachable!(),
                                }
                        })
                })
                .collect();

            autocomplete_scores.sort_unstable();

            Box::new(autocomplete_scores[autocomplete_scores.len() / 2])
        },
    )
}
