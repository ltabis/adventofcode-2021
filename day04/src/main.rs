use std::error::Error;

fn main() {
    ()
}

fn load_input() -> Result<Vec<String>, Box<dyn Error>> {
    Ok(std::fs::read_to_string("./src/input.txt")?
        .lines()
        .map(|s| s.to_string())
        .collect())
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
