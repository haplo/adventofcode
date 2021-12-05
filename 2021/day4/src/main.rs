use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Board {
    rows: Vec<HashSet<u8>>,
    columns: Vec<HashSet<u8>>,
}

fn parse_input(input: String) -> (Vec<u8>, Vec<Board>) {
    let mut lines = input.lines();

    // first line is all drawn numbers
    let drawn_numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .split(",")
        .into_iter()
        .map(|x| x.parse::<u8>().expect("Expected an integer number"))
        .collect();
    let mut lines = lines.skip(1);
    let mut boards = Vec::new();
    // boards are 5 lines followed by an empty line
    loop {
        let board_lines: Vec<&str> = lines.by_ref().take_while(|l| l.trim() != "").collect();
        if board_lines.is_empty() {
            break;
        }
        boards.push(parse_board(board_lines));
    }
    (drawn_numbers, boards)
}

fn parse_board(lines: Vec<&str>) -> Board {
    let mut rows = vec![HashSet::<u8>::new(); 5];
    let mut columns = vec![HashSet::<u8>::new(); 5];
    for (rown, line) in lines.into_iter().enumerate() {
        let numbers: Vec<u8> = line
            .split(" ")
            .filter(|n| *n != "")
            .map(|n| {
                n.trim()
                    .parse::<u8>()
                    .expect("Expected an integer number in board definition")
            })
            .collect();
        for (coln, number) in numbers.iter().enumerate() {
            columns[coln].insert(*number);
        }
        rows[rown].extend(numbers);
    }
    Board {
        rows: rows,
        columns: columns,
    }
}

// true if given Board is a winner with the given drawn numbers
fn is_winner(board: &Board, numbers: &HashSet<u8>) -> bool {
    board.rows.iter().any(|r| r.is_subset(&numbers))
        || board.columns.iter().any(|r| r.is_subset(&numbers))
}

fn calculate_score(board: &Board, numbers: &HashSet<u8>, last_draw: u8) -> u32 {
    let mut score: u32 = 0;
    for row in &board.rows {
        score += row
            .iter()
            .filter(|n| !numbers.contains(n))
            .map(|n| *n as u32)
            .sum::<u32>();
    }
    score * last_draw as u32
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (all_numbers, boards) = parse_input(input);
    let mut draw = HashSet::new();
    for last_draw in all_numbers {
        draw.insert(last_draw);
        for board in &boards {
            if is_winner(&board, &draw) {
                let score = calculate_score(&board, &draw, last_draw);
                println!("Winner board score: {}", score);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_board() {
        assert_eq!(
            parse_board(vec![
                "22 13 17 11  0",
                "8  2 23  4 24",
                "21  9 14 16  7",
                "6 10  3 18  5",
                "1 12 20 15 19",
            ]),
            Board {
                rows: vec![
                    HashSet::from([22, 13, 17, 11, 0]),
                    HashSet::from([8, 2, 23, 4, 24]),
                    HashSet::from([21, 9, 14, 16, 7]),
                    HashSet::from([6, 10, 3, 18, 5]),
                    HashSet::from([1, 12, 20, 15, 19,])
                ],
                columns: vec![
                    HashSet::from([22, 8, 21, 6, 1]),
                    HashSet::from([13, 2, 9, 10, 12]),
                    HashSet::from([17, 23, 14, 3, 20]),
                    HashSet::from([11, 4, 16, 18, 15]),
                    HashSet::from([0, 24, 7, 5, 19])
                ],
            }
        )
    }

    #[test]
    fn test_is_winner() {
        let board1 = Board {
            rows: vec![
                HashSet::from([14, 21, 17, 24, 4]),
                HashSet::from([10, 16, 15, 9, 19]),
                HashSet::from([18, 8, 23, 26, 20]),
                HashSet::from([22, 11, 13, 6, 5]),
                HashSet::from([2, 0, 12, 3, 7]),
            ],
            columns: vec![
                HashSet::from([14, 10, 18, 22, 2]),
                HashSet::from([21, 16, 8, 13, 12]),
                HashSet::from([17, 15, 23, 13, 12]),
                HashSet::from([24, 9, 26, 6, 3]),
                HashSet::from([4, 19, 20, 5, 7]),
            ],
        };
        let board2 = Board {
            rows: vec![
                HashSet::from([3, 15, 0, 2, 22]),
                HashSet::from([9, 18, 13, 17, 5]),
                HashSet::from([19, 8, 7, 25, 23]),
                HashSet::from([20, 11, 10, 24, 4]),
                HashSet::from([14, 21, 16, 12, 6]),
            ],
            columns: vec![
                HashSet::from([3, 9, 19, 20, 14]),
                HashSet::from([15, 18, 8, 11, 21]),
                HashSet::from([0, 13, 7, 10, 16]),
                HashSet::from([2, 17, 25, 24, 12]),
                HashSet::from([22, 5, 23, 4, 6]),
            ],
        };
        let draw = HashSet::from([17, 23, 4, 0, 14, 21, 24]);
        assert_eq!(is_winner(&board1, &draw), true);
        assert_eq!(is_winner(&board2, &draw), false);
    }
}
