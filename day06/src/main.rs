fn main() {
    println!("Hello, world!");
}

fn load_data(src: &str) -> std::io::Result<Vec<usize>> {
    Ok(std::fs::read_to_string(src)?
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect())
}

fn simulate(days: usize, mut data: Vec<usize>) -> Vec<usize> {
    println!("Initial state: {:?}", data);

    for day in 0..days {
        let mut spawn: Vec<usize> = Vec::new();

        data.iter_mut().for_each(|fish| {
            if *fish == 0 {
                *fish = 6;
                spawn.push(8);
            } else {
                *fish -= 1;
            }
        });

        data.extend(spawn);
        println!("After {} day(s): {:?}", day + 1, data);
    }

    data
}

#[cfg(test)]
mod test {
    use crate::{load_data, simulate};

    #[test]
    fn test_example() {
        let data = load_data("./src/example.txt").unwrap();

        println!("data: {:?}", data);
    }

    #[test]
    fn test_ex01() {
        let data1 = load_data("./src/example.txt").unwrap();
        let data2 = data1.clone();

        assert_eq!(simulate(18, data1).len(), 26);
        assert_eq!(simulate(80, data2).len(), 5934);
    }
}
