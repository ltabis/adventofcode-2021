use core::panic;
use nom::{
    bytes::complete::{tag, take_while},
    IResult,
};
use std::error::Error;

fn main() {
    ()
}

const MAP_SIZE: usize = 1000;
type Map = Vec<[u8; MAP_SIZE]>;

// parser
fn take_coord(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(nom::character::is_digit)(input)
}

fn comma(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(",")(input)
}

fn to(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(" -> ")(input)
}

fn parse_line(map: &mut Map, line: &str) -> (usize, usize, usize, usize) {
    let (line, x1) = take_coord(line.as_bytes()).unwrap();
    let (line, _) = comma(line).unwrap();
    let (line, y1) = take_coord(line).unwrap();
    let (line, _) = to(line).unwrap();
    let (line, x2) = take_coord(line).unwrap();
    let (line, _) = comma(line).unwrap();
    let (_, y2) = take_coord(line).unwrap();

    let x1: usize = std::str::from_utf8(x1).unwrap().parse().unwrap();
    let x2: usize = std::str::from_utf8(x2).unwrap().parse().unwrap();
    let y1: usize = std::str::from_utf8(y1).unwrap().parse().unwrap();
    let y2: usize = std::str::from_utf8(y2).unwrap().parse().unwrap();

    (x1, y1, x2, y2)
}

fn load_input(src: &str) -> Result<Map, Box<dyn Error>> {
    let input = std::fs::read_to_string(src)?;
    let mut map = vec![[0; MAP_SIZE]; MAP_SIZE];

    input.lines().for_each(|line| {
        let (x1, y1, x2, y2) = parse_line(&mut map, line);

        if x1 == x2 {
            for y in if y1 < y2 { y1..=y2 } else { y2..=y1 } {
                map[y][x1] += 1;
            }
        } else if y1 == y2 {
            for x in if x1 < x2 { x1..=x2 } else { x2..=x1 } {
                map[y1][x] += 1;
            }
        }
    });
    Ok(map)
}

fn load_input_with_diagonals(src: &str) -> Result<Map, Box<dyn Error>> {
    let input = std::fs::read_to_string(src)?;
    let mut map = vec![[0; MAP_SIZE]; MAP_SIZE];

    input.lines().for_each(|line| {
        let (x1, y1, x2, y2) = parse_line(&mut map, line);

        if x1 == x2 {
            for y in if y1 < y2 { y1..=y2 } else { y2..=y1 } {
                map[y][x1] += 1;
            }
        } else if y1 == y2 {
            for x in if x1 < x2 { x1..=x2 } else { x2..=x1 } {
                map[y1][x] += 1;
            }
        } else if (x1 as i32 - x2 as i32).abs() == (y1 as i32 - y2 as i32).abs() {
            let mut range_x = x1;
            let mut range_y = y1;

            while range_x != x2 && range_y != y2 {
                map[range_y][range_x] += 1;

                range_x = if x1 < x2 { range_x + 1 } else { range_x - 1 };
                range_y = if y1 < y2 { range_y + 1 } else { range_y - 1 };
            }

            map[range_y][range_x] += 1;
        }
    });
    Ok(map)
}

fn count_dangerous_areas(map: &Map) -> usize {
    map.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, col| if *col >= 2 { acc + 1 } else { acc })
    })
}

#[cfg(test)]
mod test {

    use crate::{count_dangerous_areas, load_input, load_input_with_diagonals, MAP_SIZE};

    #[test]
    fn test_example() {
        let vec = load_input("./src/test.txt").unwrap();

        println!("normal:");
        for i in 0..MAP_SIZE {
            println!("{:?}", vec[i]);
        }

        assert_eq!(count_dangerous_areas(&vec), 5);

        let vec = load_input_with_diagonals("./src/test.txt").unwrap();

        println!("diagonals:");
        for i in 0..MAP_SIZE {
            println!("{:?}", vec[i]);
        }

        assert_eq!(count_dangerous_areas(&vec), 12);
    }

    #[test]
    fn test_ex01() {
        let vec = load_input("./src/input.txt").unwrap();

        println!("ex01 result: {}", count_dangerous_areas(&vec));
    }

    #[test]
    fn test_ex02() {
        let vec = load_input_with_diagonals("./src/input.txt").unwrap();

        println!("ex02 result: {}", count_dangerous_areas(&vec));
    }
}
