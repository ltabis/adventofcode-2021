fn main() {
    let input = load_input("./src/input.txt").unwrap();

    println!(
        "ex01: {}",
        find_lowest_fuel_consumption::<SimpleCost>(&input)
    );
    println!(
        "ex02: {}",
        find_lowest_fuel_consumption::<AdvancedCost>(&input)
    );
}

fn load_input(src: &str) -> std::io::Result<Vec<i64>> {
    Ok(std::fs::read_to_string(src)?
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect())
}

/// used to implemente different methods to compute the fuel cost.
trait FuelCost {
    fn compute_fuel_cost(crab_pos: i64, desired_pos: i64) -> i64;
}

/// a simple cost computation, taking into account that a single step
/// toward the destination costs one fuel.
struct SimpleCost;
impl FuelCost for SimpleCost {
    fn compute_fuel_cost(crab_pos: i64, desired_pos: i64) -> i64 {
        (desired_pos - crab_pos).abs()
    }
}

/// a more complex cost computation, taking into account that each step
/// toward the destination, the costs of the fuel increases by one.
struct AdvancedCost;
impl FuelCost for AdvancedCost {
    fn compute_fuel_cost(crab_pos: i64, desired_pos: i64) -> i64 {
        let distance = (desired_pos - crab_pos).abs();
        (1..distance + 1).fold(0, |acc, step| acc + step)
    }
}

/// align crabs horizontaly and returns the total fuel costs.
fn align_from_position<C: FuelCost>(crabs: &Vec<i64>, position: i64) -> usize {
    crabs.iter().fold(0, |acc, crab_pos| {
        acc + C::compute_fuel_cost(*crab_pos, position) as usize
    })
}

fn find_average(crabs: &Vec<i64>) -> usize {
    crabs.iter().sum::<i64>() as usize / crabs.len()
}

fn find_lowest_fuel_consumption<C: FuelCost>(crabs: &Vec<i64>) -> usize {
    let average = find_average(crabs);
    let mut lowest_fuel_consumption = align_from_position::<C>(crabs, average as i64);

    for i in (0..average).rev() {
        let concumption = align_from_position::<C>(crabs, i as i64);
        if concumption < lowest_fuel_consumption {
            lowest_fuel_consumption = concumption;
        }
    }

    let mut index = average;

    loop {
        let concumption = align_from_position::<C>(crabs, index as i64);
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
    use crate::{
        align_from_position, find_average, find_lowest_fuel_consumption, load_input, AdvancedCost,
        FuelCost, SimpleCost,
    };

    #[test]
    fn test_example() {
        let input = load_input("./src/test.txt").unwrap();

        assert_eq!(find_average(&input), 4);
        assert_eq!(align_from_position::<SimpleCost>(&input, 2), 37);
        assert_eq!(align_from_position::<SimpleCost>(&input, 1), 41);
        assert_eq!(align_from_position::<SimpleCost>(&input, 3), 39);
        assert_eq!(align_from_position::<SimpleCost>(&input, 10), 71);

        assert_eq!(find_lowest_fuel_consumption::<SimpleCost>(&input), 37);
    }

    #[test]
    fn test_compute_cost() {
        assert_eq!(AdvancedCost::compute_fuel_cost(16, 5), 66);
        assert_eq!(AdvancedCost::compute_fuel_cost(1, 5), 10);
        assert_eq!(AdvancedCost::compute_fuel_cost(2, 5), 6);
        assert_eq!(AdvancedCost::compute_fuel_cost(0, 5), 15);
        assert_eq!(AdvancedCost::compute_fuel_cost(4, 5), 1);
        assert_eq!(AdvancedCost::compute_fuel_cost(2, 5), 6);
        assert_eq!(AdvancedCost::compute_fuel_cost(7, 5), 3);
        assert_eq!(AdvancedCost::compute_fuel_cost(1, 5), 10);
        assert_eq!(AdvancedCost::compute_fuel_cost(2, 5), 6);
        assert_eq!(AdvancedCost::compute_fuel_cost(14, 5), 45);
    }
}
