use std::error::Error;

fn main() {
    ()
}

type Pile = Vec<u64>;
type Board = [[u64; 5]; 5];

fn load_board_line(line: &str) -> [u64; 5] {
    match line
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()[..]
    {
        [a, b, c, d, e] => [a, b, c, d, e],
        _ => panic!("board line isn't 5 number long."),
    }
}

fn load_input() -> Result<(Pile, Vec<Board>), Box<dyn Error>> {
    let input = std::fs::read_to_string("./src/input.txt")?;
    let mut input = input.lines();

    let pile = input
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();

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

fn ex01(input: &Vec<String>) {}

fn ex02(input: &Vec<String>) {}

#[cfg(test)]
mod test {

    use crate::{ex01, ex02, load_input};

    #[test]
    fn test_example() {
        // let input = vec![
        //     "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
        //     "11001", "00010", "01010",
        // ]
        // .iter()
        // .map(|s| s.to_string())
        // .collect();
    }

    #[test]
    fn test_ex01() {
        let input = load_input().unwrap();

        // println!("exercice input result: {}", ex01(&input));
    }

    #[test]
    fn test_ex02() {
        let input = load_input().unwrap();

        // println!("exercice 2 input result: {}", ex02(&input));
    }
}
