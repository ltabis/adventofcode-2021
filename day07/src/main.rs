fn main() {
    let input = load_input("./src/input.txt").unwrap();

    println!("ex01: {}", find_lowest_fuel_consumption(&input));
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

fn find_average(crabs: &Vec<i64>) -> usize {
    crabs.iter().sum::<i64>() as usize / crabs.len()
}

fn find_lowest_fuel_consumption(crabs: &Vec<i64>) -> usize {
    let average = find_average(crabs);
    let mut lowest_fuel_consumption = align_from_number(crabs, average as i64);

    for i in (0..average).rev() {
        let concumption = align_from_number(crabs, i as i64);
        if concumption < lowest_fuel_consumption {
            lowest_fuel_consumption = concumption;
        }
    }

    let mut index = average;

    loop {
        let concumption = align_from_number(crabs, index as i64);
        if concumption < lowest_fuel_consumption {
            lowest_fuel_consumption = concumption;
        } else {
            break;
        }
        index += 1;
    }

    lowest_fuel_consumption
}

#[cfg(test)]
mod test {
    use crate::{align_from_number, find_average, find_lowest_fuel_consumption, load_input};

    #[test]
    fn test_example() {
        let input = load_input("./src/test.txt").unwrap();

        assert_eq!(find_average(&input), 4);
        assert_eq!(align_from_number(&input, 2), 37);
        assert_eq!(align_from_number(&input, 1), 41);
        assert_eq!(align_from_number(&input, 3), 39);
        assert_eq!(align_from_number(&input, 10), 71);

        assert_eq!(find_lowest_fuel_consumption(&input), 37);
    }
}
