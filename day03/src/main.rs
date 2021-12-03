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
    let lcbs: Vec<u64> = mcbs.iter().map(|c| [1, 0][(*c < 0) as usize]).collect();
    let mcbs: Vec<u64> = mcbs.iter().map(|c| [0, 1][(*c < 0) as usize]).collect();

    let γ_rate = mcbs.iter().fold(0, |acc, bit| (acc << 1) ^ bit);
    let ε_rate = lcbs.iter().fold(0, |acc, bit| (acc << 1) ^ bit);

    γ_rate * ε_rate
}

fn ex02(input: &Vec<String>) -> u64 {
    find_oxygen_generator_rating(input) * find_co2_scrubber_rating(input)
}

fn find_oxygen_generator_rating(input: &Vec<String>) -> u64 {
    let mut input_cpy = input.clone();
    let data_len = input[0].len();
    let mut index = 0;

    while index < data_len {
        let mut mcb = 0;
        for data in input_cpy.iter() {
            mcb += [1, -1][(data.chars().nth(index).unwrap() == '0') as usize];
        }

        let mcb = ['0', '1'][(mcb >= 0) as usize];

        input_cpy = input_cpy
            .into_iter()
            .filter(|data| data.chars().nth(index).unwrap() == mcb)
            .collect();

        index += 1;
    }

    println!("{:?}", input_cpy);

    input_cpy.to_vec()[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .fold(0, |acc, bit| (acc << 1) ^ bit)
}

fn find_co2_scrubber_rating(input: &Vec<String>) -> u64 {
    let mut input_cpy = input.clone();
    let data_len = input[0].len();
    let mut index = 0;

    while index < data_len && input_cpy.len() > 1 {
        let mut lcb = 0;
        for data in input_cpy.iter() {
            lcb += [1, -1][(data.chars().nth(index).unwrap() == '1') as usize];
        }

        let lcb = ['0', '1'][(lcb > 0) as usize];

        input_cpy = input_cpy
            .into_iter()
            .filter(|data| data.chars().nth(index).unwrap() == lcb)
            .collect();

        index += 1;
    }

    println!("{:?}", input_cpy);

    input_cpy.to_vec()[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .fold(0, |acc, bit| (acc << 1) ^ bit)
}

#[cfg(test)]
mod test {

    use crate::{ex01, ex02, find_co2_scrubber_rating, find_oxygen_generator_rating, load_input};

    #[test]
    fn test_example() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let oxygen_generator_rating = find_oxygen_generator_rating(&input);
        let co2_scrubber_rating = find_co2_scrubber_rating(&input);

        assert_eq!(ex01(&input), 198);
        assert_eq!(oxygen_generator_rating, 23);
        assert_eq!(co2_scrubber_rating, 10);
        assert_eq!(oxygen_generator_rating * co2_scrubber_rating, 230);
    }

    #[test]
    fn test_ex01() {
        let input = load_input().unwrap();

        println!("exercice input result: {}", ex01(&input));
    }

    #[test]
    fn test_ex02() {
        let input = load_input().unwrap();

        println!("exercice 2 input result: {}", ex02(&input));
    }
}
