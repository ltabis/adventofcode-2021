use std::error::Error;

fn main() {
    ()
}

type Pile = Vec<i64>;
type Board = [[i64; 5]; 5];

fn load_board_line(line: &str) -> [i64; 5] {
    match line
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()[..]
    {
        [a, b, c, d, e] => [a, b, c, d, e],
        _ => panic!("board line isn't 5 number long."),
    }
}

fn load_input(src: &str) -> Result<(Pile, Vec<Board>), Box<dyn Error>> {
    let input = std::fs::read_to_string(src)?;
    let mut input = input.lines();

    let pile = input
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut boards = Vec::new();

    loop {
        // whitespace
        if let None = input.next() {
            break;
        }

        let row1 = load_board_line(input.next().unwrap());
        let row2 = load_board_line(input.next().unwrap());
        let row3 = load_board_line(input.next().unwrap());
        let row4 = load_board_line(input.next().unwrap());
        let row5 = load_board_line(input.next().unwrap());

        boards.push([row1, row2, row3, row4, row5]);
    }

    Ok((pile, boards))
}

fn check_for_win<'a>(pile: &Pile, boards: &'a mut Vec<Board>) -> Option<(usize, &'a Board)> {
    for (index, board) in boards.iter().enumerate() {
        for row in board.iter() {
            if row.iter().all(|cell| *cell == -1) {
                // horizontal bingo!
                return Some((index, board));
            }
        }

        for column in 0..4 {
            if board.iter().all(|row| row[column] == -1) {
                // vertical bingo!
                return Some((index, board));
            }
        }
    }

    None
}

fn get_bingo(pile: Pile, mut boards: Vec<Board>) -> Option<(Board, i64)> {
    for number in &pile {
        for board in boards.iter_mut() {
            for row in board {
                row.iter_mut().for_each(|cell| {
                    if cell == number {
                        *cell = -1;
                    }
                });
            }
        }

        if let Some((_, board)) = check_for_win(&pile, &mut boards) {
            return Some((*board, *number));
        }
    }

    None
}

fn get_last_bingo(pile: Pile, mut boards: Vec<Board>) -> (Board, i64) {
    for number in &pile {
        for board in boards.iter_mut() {
            for row in board {
                row.iter_mut().for_each(|cell| {
                    if cell == number {
                        *cell = -1;
                    }
                });
            }
        }

        while let Some((index, _)) = check_for_win(&pile, &mut boards) {
            if boards.len() == 1 {
                return (boards[0], *number);
            }
            boards.remove(index);
        }
    }

    return (boards[0], *pile.last().unwrap());
}

fn ex01(pile: Pile, boards: Vec<Board>) {
    if let Some((bingo, winning_number)) = get_bingo(pile, boards) {
        let total = bingo.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .fold(0, |acc, n| if *n != -1 { acc + *n } else { acc })
        });

        println!("result ex01: {}", total * winning_number);
    }
}

fn ex02(pile: Pile, boards: Vec<Board>) {
    let (bingo, winning_number) = get_last_bingo(pile, boards);

    let total = bingo.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, n| if *n != -1 { acc + *n } else { acc })
    });

    println!("result ex02: {}", total * winning_number);
}

#[cfg(test)]
mod test {

    use crate::{ex01, ex02, get_bingo, get_last_bingo, load_input};

    #[test]
    fn test_example() {
        let (pile, boards) = load_input("./src/test.txt").unwrap();

        if let Some((bingo, winning_number)) = get_bingo(pile.clone(), boards.clone()) {
            let total = bingo.iter().fold(0, |acc, row| {
                acc + row
                    .iter()
                    .fold(0, |acc, n| if *n != -1 { acc + *n } else { acc })
            });

            assert_eq!(winning_number, 24);
            assert_eq!(total, 188);
        } else {
            assert!(false)
        }

        let (bingo, winning_number) = get_last_bingo(pile, boards);
        let total = bingo.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .fold(0, |acc, n| if *n != -1 { acc + *n } else { acc })
        });

        assert_eq!(winning_number, 13);
        assert_eq!(total, 148);
    }

    #[test]
    fn test_ex01() {
        let (pile, boards) = load_input("./src/input.txt").unwrap();

        ex01(pile, boards);
    }

    #[test]
    fn test_ex02() {
        let (pile, boards) = load_input("./src/input.txt").unwrap();

        ex02(pile, boards);
    }
}
