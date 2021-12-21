use std::{error::Error, fmt::Display};

fn main() {
    let (simultaneous, flashes) = load_input("./src/input.txt").unwrap().count_flashes(1000);
    println!("ex01 result's: {}", flashes);
    println!("ex02 result's: {:?}", simultaneous);
}

const DIR_SIZE: usize = 8;
const DIRECTIONS: [[i64; 2]; DIR_SIZE] = [
    // x, y
    [-1, -1],
    [0, -1],
    [1, -1],
    [1, 0],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

#[derive(Debug)]
struct ParserError(String);
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for ParserError {}

#[derive(Debug)]
struct Octopus {
    energy: i8,
    flashed: bool,
}

struct Map {
    inner: Vec<Vec<Octopus>>,
    max_x: i64,
    max_y: i64,
}

impl Map {
    pub fn new(input: &str) -> Result<Self, Box<dyn Error>> {
        let inner = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        Ok(Octopus {
                            energy: c.to_digit(10).ok_or(ParserError {
                                0: "couldn't parse char to u32".to_string(),
                            })? as i8,
                            flashed: false,
                        })
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<Octopus>>, ParserError>>()?;

        let max_x = inner.get(0).unwrap_or(&vec![]).len();
        let max_y = inner.len();

        inner.iter().for_each(|row| {
            row.iter().for_each(|o| {
                if o.energy != -1 {
                    print!("{}", o.energy)
                } else {
                    print!("*")
                }
            });
            println!()
        });
        println!();

        Ok(Self {
            inner,
            // berk
            max_x: max_x as i64,
            max_y: max_y as i64,
        })
    }

    fn debug(&self) {
        // debug
        self.inner.iter().for_each(|row| {
            row.iter().for_each(|o| {
                if o.energy > 9 {
                    print!("^")
                } else if o.energy != 0 {
                    print!("{}", o.energy)
                } else {
                    print!("*")
                }
            });
            println!()
        });
        println!()
    }

    fn chain_reaction(&mut self, x: i64, y: i64) -> usize {
        let mut flashes = 0;

        for dir in DIRECTIONS {
            let adj_x = x + dir[0];
            let adj_y = y + dir[1];
            if adj_x >= 0 && adj_y >= 0 && adj_x < self.max_x && adj_y < self.max_y {
                let octopus = &mut self.inner[adj_y as usize][adj_x as usize];
                octopus.energy += 1;

                if octopus.flashed == false && octopus.energy > 9 {
                    octopus.flashed = true;
                    flashes += self.chain_reaction(adj_x, adj_y) + 1;
                }
            }
        }

        flashes
    }

    pub fn count_flashes(&mut self, steps: usize) -> (Option<usize>, usize) {
        let mut flashes = 0;

        for step in 1..steps + 1 {
            // increase energy by 1.
            self.inner
                .iter_mut()
                .flatten()
                .for_each(|octopus| octopus.energy += 1);

            // loop => flash if > 9 -> adjacent + 1 | octopus flash once per step.
            for y in 0..self.max_y {
                for x in 0..self.max_x {
                    let octopus = &mut self.inner[y as usize][x as usize];

                    if octopus.flashed == false && octopus.energy > 9 {
                        octopus.flashed = true;
                        flashes += self.chain_reaction(x, y) + 1;
                    }
                }
            }

            if self.inner.iter().flatten().all(|octopus| octopus.flashed) {
                return (Some(step), 0);
            }

            // energy level set to 0 for all octopuses that flashed.
            self.inner
                .iter_mut()
                .flatten()
                .filter(|octopus| octopus.flashed)
                .for_each(|octopus| {
                    octopus.energy = 0;
                    octopus.flashed = false;
                });
        }

        (None, flashes)
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
    fn test_load_example_input() {
        let input = load_input("./src/example1.txt").expect("example1.txt couldn't be parsed");

        assert_eq!(input.inner[0][0].energy, 1);
        assert_eq!(input.inner[1][1].energy, 9);
        assert_eq!(input.inner[4][4].energy, 1);
        assert_eq!(input.max_x, 5);
        assert_eq!(input.max_y, 5);
    }

    #[test]
    fn test_example_1() {
        let mut input = load_input("./src/example1.txt").expect("example1.txt couldn't be parsed");

        let (_, flashes) = input.count_flashes(2);
        assert_eq!(flashes, 9);
    }

    #[test]
    fn test_example_2() {
        let mut input = load_input("./src/example2.txt").expect("example2.txt couldn't be parsed");

        let (_, flashes) = input.count_flashes(100);
        assert_eq!(flashes, 1656);
    }
}
