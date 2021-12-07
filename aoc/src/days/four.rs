use libaoc::{Day, DayNumber};

const BOARDWIDTH: usize = 5;
const BOARDHEIGHT: usize = 5;
const BOARDSIZE: usize = BOARDHEIGHT * BOARDWIDTH;
fn check_board(boards: &[(usize, bool)], board_number: usize) -> Option<usize> {
    let board = &boards[board_number * BOARDSIZE..board_number * BOARDSIZE + BOARDSIZE];

    // lines
    for i in 0..BOARDHEIGHT {
        let check = board[i * BOARDWIDTH..i * BOARDWIDTH + BOARDWIDTH]
            .iter()
            .filter(|(_, b)| *b)
            .count();
        if check == 5 {
            return Some(
                board
                    .iter()
                    .filter(|(_, b)| !*b)
                    .fold(0, |sum, (a, _)| sum + *a),
            );
        }
    }

    // cols
    'c: for i in 0..BOARDWIDTH {
        for j in 0..BOARDHEIGHT {
            if !board[j * BOARDWIDTH + i].1 {
                continue 'c;
            }
        }
        return Some(
            board
                .iter()
                .filter(|(_, b)| !*b)
                .fold(0, |sum, (a, _)| sum + *a),
        );
    }

    None
}

pub fn four() -> Day<2021> {
    Day::new(
        DayNumber::Four,
        |input| {
            let numbers = input
                .split_once('\n')
                .unwrap()
                .0
                .split(',')
                .map(|a| a.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut boards = input
                .lines()
                .skip(1)
                .filter(|&line| !line.is_empty())
                .map(|line| {
                    line.split_whitespace()
                        .map(|number| (number.parse::<usize>().unwrap(), false))
                        .collect::<Vec<(usize, bool)>>()
                })
                .flatten()
                .collect::<Vec<(usize, bool)>>();

            for &number in numbers.iter() {
                boards.iter_mut().for_each(|(n, marked)| {
                    if *n == number {
                        *marked = true
                    }
                });
                for board in 0..(boards.len() / BOARDSIZE) {
                    if let Some(result) = check_board(&boards, board) {
                        return (result * number).to_string();
                    }
                }
            }
            unreachable!();
        },
        |input| {
            let numbers = input
                .split_once('\n')
                .unwrap()
                .0
                .split(',')
                .map(|a| a.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut boards = input
                .lines()
                .skip(1)
                .filter(|&line| !line.is_empty())
                .map(|line| {
                    line.split_whitespace()
                        .map(|number| (number.parse::<usize>().unwrap(), false))
                        .collect::<Vec<(usize, bool)>>()
                })
                .flatten()
                .collect::<Vec<(usize, bool)>>();

            let mut all_boards = (0..(boards.len() / BOARDSIZE)).collect::<Vec<usize>>();

            for &number in numbers.iter() {
                boards.iter_mut().for_each(|(n, marked)| {
                    if *n == number {
                        *marked = true
                    }
                });
                for board in 0..(boards.len() / BOARDSIZE) {
                    if !all_boards.contains(&board) {
                        continue;
                    }
                    if let Some(result) = check_board(&boards, board) {
                        if all_boards.len() == 1 && all_boards[0] == board {
                            return (result * number).to_string();
                        }
                        all_boards = all_boards
                            .iter()
                            .filter(|&b| *b != board)
                            .copied()
                            .collect::<Vec<usize>>();
                    }
                }
            }
            unreachable!();
        },
    )
}
