use nom::{
    bytes::complete::{tag, take},
    number::complete::be_u8,
    sequence::tuple,
    IResult,
};
use std::error::Error;

fn main() {
    ()
}

type Map = [[u8; 10]; 10];

// parser
fn take1(input: &str) -> IResult<&str, &str> {
    take(1u8)(input)
}

fn comma(input: &str) -> IResult<&str, &str> {
    tag(",")(input)
}

fn to(input: &str) -> IResult<&str, &str> {
    tag(" -> ")(input)
}

fn parse_line(map: &mut Map, line: &str) {
    let (line, x1) = take1(line).unwrap();
    let (line, _) = comma(line).unwrap();
    let (line, y1) = take1(line).unwrap();
    let (line, _) = to(line).unwrap();
    let (line, x2) = take1(line).unwrap();
    let (line, _) = comma(line).unwrap();
    let (_, y2) = take1(line).unwrap();

    let x1: u8 = x1.parse().unwrap();
    let x2: u8 = x2.parse().unwrap();
    let y1: u8 = y1.parse().unwrap();
    let y2: u8 = y2.parse().unwrap();

    if x1 == x2 {
        for y in if y1 < y2 { y1..=y2 } else { y2..=y1 } {
            map[y as usize][x1 as usize] += 1;
        }
    } else if y1 == y2 {
        for x in if x1 < x2 { x1..=x2 } else { x2..=x1 } {
            map[y1 as usize][x as usize] += 1;
        }
    }
}

fn load_input(src: &str) -> Result<Map, Box<dyn Error>> {
    let input = std::fs::read_to_string(src)?;
    let mut map = [[0; 10]; 10];

    input.lines().for_each(|line| parse_line(&mut map, line));
    Ok(map)
}

fn ex01() {}

fn ex02() {}

#[cfg(test)]
mod test {

    use crate::{ex01, ex02, load_input};

    #[test]
    fn test_example() {
        let vec = load_input("./src/test.txt").unwrap();

        for i in 0..10 {
            println!("{:?}", vec[i]);
        }
    }

    #[test]
    fn test_ex01() {
        let vec = load_input("./src/input.txt").unwrap();
    }

    #[test]
    fn test_ex02() {
        let vec = load_input("./src/input.txt").unwrap();
    }
}
