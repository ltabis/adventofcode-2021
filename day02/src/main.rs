use std::{error::Error, fmt::Display, str::FromStr};

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

#[derive(Debug)]
struct CommandError {}
impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse command")
    }
}
impl Error for CommandError {}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_whitespace().collect::<Vec<&str>>();
        match s[..] {
            ["forward", ammount] => Ok(Command::Forward(ammount.parse().unwrap_or(0))),
            ["up", ammount] => Ok(Command::Up(ammount.parse().unwrap_or(0))),
            ["down", ammount] => Ok(Command::Down(ammount.parse().unwrap_or(0))),
            _ => Err(CommandError {}),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./src/input.txt")?
        .lines()
        .map(|s| Command::from_str(s).unwrap())
        .collect::<Vec<Command>>();

    println!("product of depth and horizontal pos: {}", ex01(&input));
    println!(
        "product of depth and horizontal pos using the aim: {}",
        ex02(&input)
    );

    Ok(())
}

fn ex01(input: &Vec<Command>) -> u64 {
    let mut depth = 0;
    let mut horizontal_pos = 0;

    for cmd in input {
        match cmd {
            Command::Forward(ammount) => horizontal_pos += ammount,
            Command::Up(ammount) => depth -= ammount,
            Command::Down(ammount) => depth += ammount,
        }
    }

    depth * horizontal_pos
}

fn ex02(input: &Vec<Command>) -> u64 {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_pos = 0;

    for cmd in input {
        match cmd {
            Command::Forward(ammount) => {
                horizontal_pos += ammount;
                depth += aim * ammount;
            }
            Command::Up(ammount) => aim -= ammount,
            Command::Down(ammount) => aim += ammount,
        }
    }

    depth * horizontal_pos
}

#[cfg(test)]
mod test {
    use crate::{ex01, ex02, Command};

    #[test]
    fn test_ex01() {
        println!(
            "product of depth and horizontal pos: {}",
            ex01(&vec![
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2)
            ])
        );
    }

    #[test]
    fn test_ex02() {
        println!(
            "product of depth and horizontal pos using the aim: {}",
            ex02(&vec![
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2)
            ])
        );
    }
}
