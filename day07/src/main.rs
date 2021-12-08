fn main() {
    ()
}

fn load_input(src: &str) -> std::io::Result<Vec<i64>> {
    Ok(std::fs::read_to_string(src)?
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect())
}

/// align crabs horizontaly and returns the total fuel costs.
fn align_from_number(crabs: &Vec<i64>, position: i64) -> usize {
    crabs.iter().fold(0, |acc, crab_pos| {
        acc + (position - crab_pos).abs() as usize
    })
}

#[cfg(test)]
mod test {
    use crate::{align_from_number, load_input};

    #[test]
    fn test_example() {
        let input = load_input("./src/test.txt").unwrap();

        assert_eq!(align_from_number(&input, 2), 37);
        assert_eq!(align_from_number(&input, 1), 41);
        assert_eq!(align_from_number(&input, 3), 39);
        assert_eq!(align_from_number(&input, 10), 71);
    }
}
