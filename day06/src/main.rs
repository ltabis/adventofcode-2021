fn main() {
    ()
}

fn load_data(src: &str) -> std::io::Result<Vec<u8>> {
    Ok(std::fs::read_to_string(src)?
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect())
}

struct Simulation(bool);

impl Simulation {
    pub fn new() -> Self {
        Self { 0: false }
    }

    pub fn debug(mut self) -> Self {
        self.0 = true;
        self
    }

    fn simulate(&self, days: usize, mut data: Vec<u8>) -> Vec<u8> {
        if self.0 {
            println!("Initial state: {:?}", data);
        }

        for day in 0..days {
            let mut spawn: Vec<u8> =
                Vec::with_capacity(data.iter().fold(0, |acc, time| acc + (*time == 0) as usize));

            data.iter_mut().for_each(|fish| {
                if *fish == 0 {
                    *fish = 6;
                    spawn.push(8);
                } else {
                    *fish -= 1;
                }
            });

            data.extend(spawn);

            if self.0 {
                println!("After {} day(s): {:?}", day + 1, data);
            }
        }

        data
    }

    fn simulate_enhanced(&self, days: usize, fishes: Vec<u8>) -> usize {
        let mut population_days_left = [0; 9];

        // init.
        fishes
            .iter()
            .for_each(|fish| population_days_left[*fish as usize] += 1);

        for _ in 0..days {
            let reset = population_days_left[0];

            population_days_left[0] = population_days_left[1];
            population_days_left[1] = population_days_left[2];
            population_days_left[2] = population_days_left[3];
            population_days_left[3] = population_days_left[4];
            population_days_left[4] = population_days_left[5];
            population_days_left[5] = population_days_left[6];
            population_days_left[6] = population_days_left[7] + reset;
            population_days_left[7] = population_days_left[8];
            population_days_left[8] = reset;
        }

        population_days_left.iter().fold(0, |acc, pop| acc + pop)
    }
}

#[cfg(test)]
mod test {
    use crate::{load_data, Simulation};

    #[test]
    fn test_example() {
        let data1 = load_data("./src/example.txt").unwrap();
        let data2 = data1.clone();
        let data3 = data1.clone();

        assert_eq!(Simulation::new().debug().simulate(18, data1).len(), 26);
        assert_eq!(Simulation::new().simulate(80, data2).len(), 5934);
        assert_eq!(Simulation::new().simulate(256, data3).len(), 26984457539);
    }

    #[test]
    fn test_example_enhanced() {
        let data1 = load_data("./src/example.txt").unwrap();
        let data2 = data1.clone();
        let data3 = data1.clone();

        assert_eq!(Simulation::new().debug().simulate_enhanced(18, data1), 26);
        assert_eq!(Simulation::new().simulate_enhanced(80, data2), 5934);
        assert_eq!(Simulation::new().simulate_enhanced(256, data3), 26984457539);
    }

    #[test]
    fn test_ex01() {
        let data = load_data("./src/input.txt").unwrap();

        println!(
            "ex01 result: {}",
            Simulation::new().simulate(80, data).len()
        );
    }

    #[test]
    fn test_ex02() {
        let data = load_data("./src/input.txt").unwrap();

        println!(
            "ex02 result: {}",
            Simulation::new().simulate_enhanced(256, data)
        );
    }
}
