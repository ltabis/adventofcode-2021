use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./src/input.txt")?
        .lines()
        .collect::<Vec<&str>>();

    Ok(())
}

fn ex01() {}

fn ex02() {}

#[cfg(test)]
mod test {
    #[test]
    fn test_ex01() {}

    #[test]
    fn test_ex02() {}
}
