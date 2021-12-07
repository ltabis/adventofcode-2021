fn main() {
    println!("Hello, world!");
}

fn load_data(src: &str) -> std::io::Result<Vec<usize>> {
    Ok(std::fs::read_to_string(src)?
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect())
}

#[cfg(test)]
mod test {
    use crate::load_data;

    #[test]
    fn test_example() {
        let data = load_data("./src/example.txt").unwrap();

        println!("data: {:?}", data);
    }
}
