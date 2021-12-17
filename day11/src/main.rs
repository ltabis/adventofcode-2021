use std::{error::Error, fmt::Display};

fn main() {
    ()
}

// during 1 step
//   increase 1
//   loop => flash > 9 -> adjacent + 1 | flash only once
//   enery level set to 0

struct Octopus(u8);

#[derive(Debug)]
struct ParserError(String);

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParserError {}

struct Map {
    inner: Vec<Vec<Octopus>>,
}

impl Map {
    fn new(input: &str) -> Result<Self, Box<dyn Error>> {
        let inner = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        Ok(Octopus {
                            0: c.to_digit(10).ok_or(ParserError {
                                0: "couldn't parse char to u32".to_string(),
                            })? as u8,
                        })
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<Octopus>>, ParserError>>()?;

        Ok(Self { inner })
    }
}

fn load_input(src: &str) -> Result<Map, Box<dyn Error>> {
    let input = std::fs::read_to_string(src)?;

    Map::new(&input)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_load_input() {
        let input = load_input("./src/example.txt").expect("example.txt couldn't be parsed");

        assert_eq!(input.inner[0][0].0, 5);
        assert_eq!(input.inner[9][9].0, 6);
    }
}
