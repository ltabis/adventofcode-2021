fn main() {
    let input = load_input("./src/input.txt").expect("failed to read input");

    println!("ex01: {}", get_simple_numbers_occurence(&input));
}

fn get_simple_numbers_occurence(data: &Vec<Data>) -> usize {
    data.iter().fold(0, |acc, data| {
        acc + data.digits.iter().fold(0, |acc, digit| {
            // acc + get_digit(digit.len()).is_some() as usize
            acc + if get_digit(digit.len()).is_some() {
                1
            } else {
                0
            }
        })
    })
}

struct Data {
    patterns: Vec<String>,
    digits: Vec<String>,
}

fn get_digit(size: usize) -> Option<u8> {
    match size {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

fn get_patterns(input: &str) -> Vec<String> {
    input
        .split(char::is_whitespace)
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn load_input(path: &str) -> std::io::Result<Vec<Data>> {
    let raw = std::fs::read_to_string(path)?;

    Ok(raw
        .lines()
        .flat_map(|line| {
            let mut line = line.split("|");

            match (line.next(), line.next()) {
                (Some(patterns), Some(digits)) => Some(Data {
                    patterns: get_patterns(patterns),
                    digits: get_patterns(digits),
                }),
                _ => None,
            }
        })
        .collect())
}

#[cfg(test)]
mod test {
    use crate::{get_simple_numbers_occurence, load_input, Data};

    #[test]
    fn test_data_loading() {
        let input = load_input("./src/test1.txt").expect("failed to read input");

        assert!(match &input[..] {
            [Data { .. }] => true,
            _ => false,
        });

        let data = &input[0];

        assert!(match &data
            .patterns
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()[..]
        {
            ["acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"] =>
                true,
            _ => {
                eprintln!("{:?}", data.patterns);
                false
            }
        });

        assert!(match &data
            .digits
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()[..]
        {
            ["cdfeb", "fcadb", "cdfeb", "cdbaf"] => true,
            _ => false,
        });
    }

    #[test]
    fn test_simple_numbers_occurence() {
        let input = load_input("./src/test2.txt").expect("failed to read input");

        assert_eq!(input.iter().len(), 10);
        assert_eq!(get_simple_numbers_occurence(&input), 26);
    }
}
