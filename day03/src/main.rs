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

fn ex01(input: &Vec<String>) -> u64 {
    let mut mcbs = vec![0; input[0].len()];

    for bits in input {
        for (index, bit) in bits.chars().enumerate() {
            mcbs[index] += [1, -1][(bit == '0') as usize];
        }
    }

    // FIXME: to optimise.
    let mcbs_regular: Vec<u64> = mcbs.iter().map(|c| [0, 1][(*c < 0) as usize]).collect();
    let mcbs_reversed: Vec<u64> = mcbs.iter().map(|c| [1, 0][(*c < 0) as usize]).collect();

    let γ_rate = mcbs_regular.iter().fold(0, |acc, bit| (acc << 1) ^ bit);
    let ε_rate = mcbs_reversed.iter().fold(0, |acc, bit| (acc << 1) ^ bit);

    γ_rate * ε_rate
}

fn ex02() {}

#[cfg(test)]
mod test {

    use crate::{ex01, load_input};

    #[test]
    fn test_ex01() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(ex01(&input), 198);
    }

    #[test]
    fn test_ex02() {}

    #[test]
    fn input() {
        let input = load_input().unwrap();

        println!("exercice input result: {}", ex01(&input));
    }
}
