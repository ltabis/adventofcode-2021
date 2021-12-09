fn main() {
    ()
}

struct Data {
    patterns: Vec<String>,
    digits: Vec<String>,
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
    use crate::{load_input, Data};

    #[test]
    fn test_data_loading() {
        let input = load_input("./src/test.txt").expect("failed to read input");

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
}
